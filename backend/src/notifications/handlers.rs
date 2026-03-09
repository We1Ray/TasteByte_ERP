use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;

use crate::notifications::models::{NotificationListParams, UnreadCount};
use crate::notifications::services;
use crate::shared::types::{AppState, Claims};
use crate::shared::{ApiResponse, AppError, PaginatedResponse};

use super::models::Notification;

pub async fn list_notifications(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<NotificationListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Notification>>>, AppError> {
    let result = services::get_notifications(&state.pool, claims.sub, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_unread_count(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<ApiResponse<UnreadCount>>, AppError> {
    let count = services::get_unread_count(&state.pool, claims.sub).await?;
    Ok(Json(ApiResponse::success(UnreadCount { count })))
}

pub async fn mark_as_read(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let updated = services::mark_as_read(&state.pool, id, claims.sub).await?;
    if !updated {
        return Err(AppError::NotFound("Notification not found".to_string()));
    }
    Ok(Json(ApiResponse::with_message(
        (),
        "Notification marked as read",
    )))
}

pub async fn mark_all_as_read(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let count = services::mark_all_as_read(&state.pool, claims.sub).await?;
    Ok(Json(ApiResponse::with_message(
        (),
        format!("{count} notifications marked as read"),
    )))
}

pub async fn delete_notification(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let deleted = services::delete_notification(&state.pool, id, claims.sub).await?;
    if !deleted {
        return Err(AppError::NotFound("Notification not found".to_string()));
    }
    Ok(Json(ApiResponse::with_message((), "Notification deleted")))
}
