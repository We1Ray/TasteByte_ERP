use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{
    PlatformAdmin, PlatformDeveloper, RequirePlatformRole,
};
use crate::lowcode::services::{form_builder, workflow};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, PaginatedResponse, PaginationParams};

pub async fn list_releases(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Query(filter): Query<ReleaseFilter>,
) -> Result<Json<ApiResponse<PaginatedResponse<Release>>>, AppError> {
    let per_page = filter.per_page.unwrap_or(20).clamp(1, 100);
    let page = filter.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    let (count, rows) = match (&filter.operation_id, &filter.status) {
        (Some(op_id), Some(status)) => {
            let (c,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM lc_releases WHERE operation_id = $1 AND status = $2",
            )
            .bind(op_id)
            .bind(status)
            .fetch_one(&state.pool)
            .await?;

            let r = sqlx::query_as::<_, Release>(
                "SELECT * FROM lc_releases WHERE operation_id = $1 AND status = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
            )
            .bind(op_id)
            .bind(status)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.pool)
            .await?;
            (c, r)
        }
        (Some(op_id), None) => {
            let (c,): (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM lc_releases WHERE operation_id = $1")
                    .bind(op_id)
                    .fetch_one(&state.pool)
                    .await?;

            let r = sqlx::query_as::<_, Release>(
                "SELECT * FROM lc_releases WHERE operation_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(op_id)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.pool)
            .await?;
            (c, r)
        }
        (None, Some(status)) => {
            let (c,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM lc_releases WHERE status = $1")
                .bind(status)
                .fetch_one(&state.pool)
                .await?;

            let r = sqlx::query_as::<_, Release>(
                "SELECT * FROM lc_releases WHERE status = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(status)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.pool)
            .await?;
            (c, r)
        }
        (None, None) => {
            let (c,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM lc_releases")
                .fetch_one(&state.pool)
                .await?;

            let r = sqlx::query_as::<_, Release>(
                "SELECT * FROM lc_releases ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.pool)
            .await?;
            (c, r)
        }
    };

    let params = PaginationParams {
        page: Some(page),
        per_page: Some(per_page),
    };
    Ok(Json(ApiResponse::success(PaginatedResponse::new(
        rows, count, &params,
    ))))
}

pub async fn create_release(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformDeveloper>,
    Json(input): Json<CreateRelease>,
) -> Result<Json<ApiResponse<Release>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Get current operation version
    let operation = sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE id = $1")
        .bind(input.operation_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    // Create form snapshot
    let snapshot = form_builder::create_snapshot(&state.pool, input.operation_id).await?;

    let release_number = crate::fi::repositories::next_number(&state.pool, "LCR").await?;

    let release = sqlx::query_as::<_, Release>(
        "INSERT INTO lc_releases (release_number, operation_id, version, title, description, status, form_snapshot, submitted_by) VALUES ($1, $2, $3, $4, $5, 'DRAFT', $6, $7) RETURNING *",
    )
    .bind(&release_number)
    .bind(input.operation_id)
    .bind(operation.version)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&snapshot)
    .bind(guard.claims.sub)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(release, "Release created")))
}

pub async fn submit_release(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Release>>, AppError> {
    let existing = sqlx::query_as::<_, Release>("SELECT * FROM lc_releases WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Release not found".to_string()))?;

    workflow::validate_release_transition(&existing.status, "SUBMITTED")?;

    let release = sqlx::query_as::<_, Release>(
        "UPDATE lc_releases SET status = 'SUBMITTED', submitted_at = NOW(), updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    // Notify all PLATFORM_ADMIN users about the new submission
    let notif_title = format!("[{}] Release submitted for review", release.release_number);
    let notif_message = format!(
        "Release \"{}\" (v{}) has been submitted and awaits review.",
        release.title, release.version,
    );
    if let Err(e) = crate::lowcode::services::notifications::notify_platform_admins(
        &state.pool,
        &notif_title,
        &notif_message,
        "RELEASE",
        Some("RELEASE"),
        Some(release.id),
    )
    .await
    {
        tracing::warn!("Failed to send release-submitted notifications: {e}");
    }

    Ok(Json(ApiResponse::with_message(
        release,
        "Release submitted for review",
    )))
}

pub async fn approve_release(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
    Json(input): Json<ReviewRelease>,
) -> Result<Json<ApiResponse<Release>>, AppError> {
    let existing = sqlx::query_as::<_, Release>("SELECT * FROM lc_releases WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Release not found".to_string()))?;

    workflow::validate_release_transition(&existing.status, "APPROVED")?;

    let release = sqlx::query_as::<_, Release>(
        "UPDATE lc_releases SET status = 'APPROVED', reviewed_by = $2, review_notes = $3, reviewed_at = NOW(), updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(guard.claims.sub)
    .bind(&input.notes)
    .fetch_one(&state.pool)
    .await?;

    // Notify the submitter that the release was approved
    if let Some(submitter_id) = existing.submitted_by {
        let notif_title = format!("[{}] Release approved", release.release_number);
        let notif_message = format!(
            "Your release \"{}\" (v{}) has been approved.",
            release.title, release.version,
        );
        if let Err(e) = crate::lowcode::services::notifications::create_notification(
            &state.pool,
            submitter_id,
            &notif_title,
            &notif_message,
            "RELEASE",
            Some("RELEASE"),
            Some(release.id),
        )
        .await
        {
            tracing::warn!("Failed to send release-approved notification: {e}");
        }
    }

    Ok(Json(ApiResponse::with_message(release, "Release approved")))
}

pub async fn reject_release(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
    Json(input): Json<ReviewRelease>,
) -> Result<Json<ApiResponse<Release>>, AppError> {
    let existing = sqlx::query_as::<_, Release>("SELECT * FROM lc_releases WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Release not found".to_string()))?;

    workflow::validate_release_transition(&existing.status, "REJECTED")?;

    let release = sqlx::query_as::<_, Release>(
        "UPDATE lc_releases SET status = 'REJECTED', reviewed_by = $2, review_notes = $3, reviewed_at = NOW(), updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(guard.claims.sub)
    .bind(&input.notes)
    .fetch_one(&state.pool)
    .await?;

    // Notify the submitter that the release was rejected
    if let Some(submitter_id) = existing.submitted_by {
        let notif_title = format!("[{}] Release rejected", release.release_number);
        let notif_message = format!(
            "Your release \"{}\" (v{}) has been rejected.{}",
            release.title,
            release.version,
            input
                .notes
                .as_deref()
                .map(|n| format!(" Reason: {n}"))
                .unwrap_or_default(),
        );
        if let Err(e) = crate::lowcode::services::notifications::create_notification(
            &state.pool,
            submitter_id,
            &notif_title,
            &notif_message,
            "RELEASE",
            Some("RELEASE"),
            Some(release.id),
        )
        .await
        {
            tracing::warn!("Failed to send release-rejected notification: {e}");
        }
    }

    Ok(Json(ApiResponse::with_message(release, "Release rejected")))
}

pub async fn publish_release(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Release>>, AppError> {
    let existing = sqlx::query_as::<_, Release>("SELECT * FROM lc_releases WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Release not found".to_string()))?;

    workflow::validate_release_transition(&existing.status, "RELEASED")?;

    // Publish the release and mark the operation as published
    let mut tx = state.pool.begin().await?;

    let release = sqlx::query_as::<_, Release>(
        "UPDATE lc_releases SET status = 'RELEASED', released_at = NOW(), updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE lc_operations SET is_published = true, version = $2, updated_at = NOW() WHERE id = $1",
    )
    .bind(release.operation_id)
    .bind(release.version)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(ApiResponse::with_message(
        release,
        "Release published",
    )))
}
