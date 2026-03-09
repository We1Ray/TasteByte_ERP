use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{
    PlatformAdmin, PlatformUser, RequirePlatformRole,
};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

pub async fn list_navigation(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformUser>,
) -> Result<Json<ApiResponse<Vec<NavigationItem>>>, AppError> {
    let items = sqlx::query_as::<_, NavigationItem>(
        "SELECT * FROM lc_navigation_items ORDER BY sort_order, title",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(items)))
}

pub async fn create_navigation_item(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Json(input): Json<CreateNavigationItem>,
) -> Result<Json<ApiResponse<NavigationItem>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let item = sqlx::query_as::<_, NavigationItem>(
        "INSERT INTO lc_navigation_items (parent_id, title, icon, route, operation_id, sort_order, is_visible, required_role) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
    )
    .bind(input.parent_id)
    .bind(&input.title)
    .bind(&input.icon)
    .bind(&input.route)
    .bind(input.operation_id)
    .bind(input.sort_order.unwrap_or(0))
    .bind(input.is_visible.unwrap_or(true))
    .bind(&input.required_role)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        item,
        "Navigation item created",
    )))
}

pub async fn update_navigation_item(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateNavigationItem>,
) -> Result<Json<ApiResponse<NavigationItem>>, AppError> {
    let item = sqlx::query_as::<_, NavigationItem>(
        "UPDATE lc_navigation_items SET parent_id = COALESCE($2, parent_id), title = COALESCE($3, title), icon = COALESCE($4, icon), route = COALESCE($5, route), operation_id = COALESCE($6, operation_id), sort_order = COALESCE($7, sort_order), is_visible = COALESCE($8, is_visible), required_role = COALESCE($9, required_role), updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(input.parent_id)
    .bind(&input.title)
    .bind(&input.icon)
    .bind(&input.route)
    .bind(input.operation_id)
    .bind(input.sort_order)
    .bind(input.is_visible)
    .bind(&input.required_role)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Navigation item not found".to_string()))?;

    Ok(Json(ApiResponse::with_message(
        item,
        "Navigation item updated",
    )))
}

pub async fn delete_navigation_item(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let result = sqlx::query("DELETE FROM lc_navigation_items WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Navigation item not found".to_string()));
    }

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "deleted": true }),
        "Navigation item deleted",
    )))
}

pub async fn reorder_navigation(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Json(input): Json<ReorderInput>,
) -> Result<Json<ApiResponse<Vec<NavigationItem>>>, AppError> {
    let mut tx = state.pool.begin().await?;

    for item in &input.items {
        sqlx::query(
            "UPDATE lc_navigation_items SET sort_order = $2, updated_at = NOW() WHERE id = $1",
        )
        .bind(item.id)
        .bind(item.sort_order)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    let items = sqlx::query_as::<_, NavigationItem>(
        "SELECT * FROM lc_navigation_items ORDER BY sort_order, title",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        items,
        "Navigation reordered",
    )))
}
