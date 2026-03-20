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

pub async fn get_release(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Release>>, AppError> {
    let release = sqlx::query_as::<_, Release>("SELECT * FROM lc_releases WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Release not found".to_string()))?;

    Ok(Json(ApiResponse::success(release)))
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

pub async fn rollback_release(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Release>>, AppError> {
    let release = sqlx::query_as::<_, Release>("SELECT * FROM lc_releases WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Release not found".to_string()))?;

    if release.status != "RELEASED" {
        return Err(AppError::Validation(
            "Only released versions can be rolled back".to_string(),
        ));
    }

    // Restore form from snapshot
    let snapshot = &release.form_snapshot;
    if snapshot.is_null() {
        return Err(AppError::Validation(
            "No form snapshot available for rollback".to_string(),
        ));
    }

    let form_response: crate::lowcode::models::FormResponse =
        serde_json::from_value(snapshot.clone())
            .map_err(|e| AppError::Internal(format!("Failed to parse snapshot: {}", e)))?;

    // Convert FormResponse back to SaveFormRequest
    let save_request = crate::lowcode::models::SaveFormRequest {
        layout_config: Some(form_response.form.layout_config),
        form_settings: Some(form_response.form.form_settings),
        sections: form_response
            .sections
            .into_iter()
            .map(|s| crate::lowcode::models::SaveSectionInput {
                id: Some(s.section.id),
                title: s.section.title,
                description: s.section.description,
                columns: Some(s.section.columns),
                sort_order: s.section.sort_order,
                is_collapsible: Some(s.section.is_collapsible),
                is_default_collapsed: Some(s.section.is_default_collapsed),
                visibility_rule: s.section.visibility_rule,
                fields: s
                    .fields
                    .into_iter()
                    .map(|f| crate::lowcode::models::SaveFieldInput {
                        id: Some(f.field.id),
                        field_name: f.field.field_name,
                        field_label: f.field.field_label,
                        field_type: f.field.field_type,
                        db_table: f.field.db_table,
                        db_column: f.field.db_column,
                        is_required: Some(f.field.is_required),
                        is_unique: Some(f.field.is_unique),
                        is_searchable: Some(f.field.is_searchable),
                        default_value: f.field.default_value,
                        default_value_sql: f.field.default_value_sql,
                        placeholder: f.field.placeholder,
                        help_text: f.field.help_text,
                        validation_regex: f.field.validation_regex,
                        validation_message: f.field.validation_message,
                        min_value: f.field.min_value,
                        max_value: f.field.max_value,
                        min_length: f.field.min_length,
                        max_length: f.field.max_length,
                        depends_on: f.field.depends_on,
                        data_source_sql: f.field.data_source_sql,
                        display_column: f.field.display_column,
                        value_column: f.field.value_column,
                        visibility_rule: f.field.visibility_rule,
                        field_config: Some(f.field.field_config),
                        sort_order: f.field.sort_order,
                        column_span: Some(f.field.column_span),
                        options: Some(
                            f.options
                                .into_iter()
                                .map(|o| crate::lowcode::models::SaveFieldOptionInput {
                                    id: Some(o.id),
                                    option_label: o.option_label,
                                    option_value: o.option_value,
                                    sort_order: o.sort_order,
                                    is_default: Some(o.is_default),
                                    is_active: Some(o.is_active),
                                })
                                .collect(),
                        ),
                    })
                    .collect(),
            })
            .collect(),
    };

    // Apply the snapshot
    form_builder::save_form(&state.pool, release.operation_id, save_request).await?;

    // Record journal entry
    sqlx::query(
        "INSERT INTO lc_dev_journal (operation_id, changed_by, change_type, diff_summary, form_snapshot, version) VALUES ($1, $2, 'ROLLBACK', $3, $4, $5)",
    )
    .bind(release.operation_id)
    .bind(guard.claims.sub)
    .bind(format!("Rolled back to release {}", release.release_number))
    .bind(&release.form_snapshot)
    .bind(release.version)
    .execute(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        release,
        "Release rolled back",
    )))
}
