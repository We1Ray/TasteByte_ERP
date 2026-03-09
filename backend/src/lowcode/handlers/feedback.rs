use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{
    PlatformDeveloper, PlatformUser, RequirePlatformRole,
};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, PaginatedResponse, PaginationParams};

pub async fn list_feedback(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformUser>,
    Query(filter): Query<FeedbackFilter>,
) -> Result<Json<ApiResponse<PaginatedResponse<Feedback>>>, AppError> {
    let per_page = filter.per_page.unwrap_or(20).clamp(1, 100);
    let page = filter.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM lc_feedback WHERE ($1::uuid IS NULL OR operation_id = $1) AND ($2::text IS NULL OR status = $2) AND ($3::text IS NULL OR feedback_type = $3)",
    )
    .bind(filter.operation_id)
    .bind(&filter.status)
    .bind(&filter.feedback_type)
    .fetch_one(&state.pool)
    .await?;

    let rows = sqlx::query_as::<_, Feedback>(
        "SELECT * FROM lc_feedback WHERE ($1::uuid IS NULL OR operation_id = $1) AND ($2::text IS NULL OR status = $2) AND ($3::text IS NULL OR feedback_type = $3) ORDER BY created_at DESC LIMIT $4 OFFSET $5",
    )
    .bind(filter.operation_id)
    .bind(&filter.status)
    .bind(&filter.feedback_type)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let params = PaginationParams {
        page: Some(page),
        per_page: Some(per_page),
    };
    Ok(Json(ApiResponse::success(PaginatedResponse::new(
        rows, count, &params,
    ))))
}

pub async fn create_feedback(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Json(input): Json<CreateFeedback>,
) -> Result<Json<ApiResponse<Feedback>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let ticket_number = crate::fi::repositories::next_number(&state.pool, "TKT").await?;

    let feedback = sqlx::query_as::<_, Feedback>(
        "INSERT INTO lc_feedback (ticket_number, operation_id, feedback_type, title, description, priority, submitted_by) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(&ticket_number)
    .bind(input.operation_id)
    .bind(input.feedback_type.as_deref().unwrap_or("BUG"))
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.priority.as_deref().unwrap_or("MEDIUM"))
    .bind(guard.claims.sub)
    .fetch_one(&state.pool)
    .await?;

    // Notify project developers about new feedback
    let notif_title = format!(
        "[{}] New feedback: {}",
        feedback.ticket_number, feedback.title
    );
    let notif_message = format!(
        "A new {} ticket ({}) has been submitted.",
        feedback.feedback_type.to_lowercase(),
        feedback.ticket_number,
    );
    if let Err(e) = crate::lowcode::services::notifications::notify_project_developers(
        &state.pool,
        feedback.operation_id,
        &notif_title,
        &notif_message,
        "FEEDBACK",
        Some("FEEDBACK"),
        Some(feedback.id),
    )
    .await
    {
        tracing::warn!("Failed to send feedback-created notifications: {e}");
    }

    Ok(Json(ApiResponse::with_message(
        feedback,
        "Feedback submitted",
    )))
}

pub async fn update_feedback(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateFeedback>,
) -> Result<Json<ApiResponse<Feedback>>, AppError> {
    let existing = sqlx::query_as::<_, Feedback>("SELECT * FROM lc_feedback WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Feedback not found".to_string()))?;

    // Validate status transition if status is being changed
    if let Some(ref new_status) = input.status {
        crate::lowcode::services::workflow::validate_feedback_transition(
            &existing.status,
            new_status,
        )?;
    }

    let feedback = sqlx::query_as::<_, Feedback>(
        "UPDATE lc_feedback SET status = COALESCE($2, status), priority = COALESCE($3, priority), assigned_to = COALESCE($4, assigned_to), resolved_at = CASE WHEN $2 = 'RESOLVED' THEN NOW() ELSE resolved_at END, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(&input.status)
    .bind(&input.priority)
    .bind(input.assigned_to)
    .fetch_one(&state.pool)
    .await?;

    // Notify submitter when status changes to IN_PROGRESS
    if input.status.as_deref() == Some("IN_PROGRESS") {
        let notif_title = format!("[{}] Feedback in progress", feedback.ticket_number,);
        let notif_message = format!(
            "Your feedback \"{}\" is now being worked on.",
            feedback.title,
        );
        if let Err(e) = crate::lowcode::services::notifications::create_notification(
            &state.pool,
            existing.submitted_by,
            &notif_title,
            &notif_message,
            "FEEDBACK",
            Some("FEEDBACK"),
            Some(feedback.id),
        )
        .await
        {
            tracing::warn!("Failed to send feedback-in-progress notification: {e}");
        }
    }

    // Notify assignee when feedback is assigned
    if let Some(assignee_id) = input.assigned_to {
        // Only notify if the assignment actually changed
        if existing.assigned_to != Some(assignee_id) {
            let notif_title = format!("[{}] Feedback assigned to you", feedback.ticket_number,);
            let notif_message =
                format!("You have been assigned to feedback \"{}\".", feedback.title,);
            if let Err(e) = crate::lowcode::services::notifications::create_notification(
                &state.pool,
                assignee_id,
                &notif_title,
                &notif_message,
                "FEEDBACK",
                Some("FEEDBACK"),
                Some(feedback.id),
            )
            .await
            {
                tracing::warn!("Failed to send feedback-assigned notification: {e}");
            }
        }
    }

    Ok(Json(ApiResponse::with_message(
        feedback,
        "Feedback updated",
    )))
}

pub async fn list_comments(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformUser>,
    Path(feedback_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<FeedbackComment>>>, AppError> {
    let comments = sqlx::query_as::<_, FeedbackComment>(
        "SELECT * FROM lc_feedback_comments WHERE feedback_id = $1 ORDER BY created_at",
    )
    .bind(feedback_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(comments)))
}

pub async fn add_comment(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(feedback_id): Path<Uuid>,
    Json(input): Json<CreateFeedbackComment>,
) -> Result<Json<ApiResponse<FeedbackComment>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Verify feedback exists
    sqlx::query("SELECT id FROM lc_feedback WHERE id = $1")
        .bind(feedback_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Feedback not found".to_string()))?;

    let comment = sqlx::query_as::<_, FeedbackComment>(
        "INSERT INTO lc_feedback_comments (feedback_id, user_id, content) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(feedback_id)
    .bind(guard.claims.sub)
    .bind(&input.content)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(comment, "Comment added")))
}
