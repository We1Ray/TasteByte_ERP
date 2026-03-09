use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformUser, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

pub async fn list_notifications(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
) -> Result<Json<ApiResponse<Vec<Notification>>>, AppError> {
    let notifications = sqlx::query_as::<_, Notification>(
        "SELECT * FROM lc_notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50",
    )
    .bind(guard.claims.sub)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(notifications)))
}

pub async fn mark_read(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Notification>>, AppError> {
    let notification = sqlx::query_as::<_, Notification>(
        "UPDATE lc_notifications SET is_read = true WHERE id = $1 AND user_id = $2 RETURNING *",
    )
    .bind(id)
    .bind(guard.claims.sub)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Notification not found".to_string()))?;

    Ok(Json(ApiResponse::success(notification)))
}

pub async fn mark_all_read(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let result = sqlx::query(
        "UPDATE lc_notifications SET is_read = true WHERE user_id = $1 AND is_read = false",
    )
    .bind(guard.claims.sub)
    .execute(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "updated": result.rows_affected() }),
        "All notifications marked as read",
    )))
}
