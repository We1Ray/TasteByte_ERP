use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{
    PlatformDeveloper, PlatformUser, RequirePlatformRole,
};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

/// GET /lowcode/operations/{id}/buttons — Developer: get buttons for an operation
pub async fn get_buttons(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<OperationButton>>>, AppError> {
    let buttons = sqlx::query_as::<_, OperationButton>(
        "SELECT * FROM lc_operation_buttons WHERE operation_id = $1 ORDER BY sort_order, button_key",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(buttons)))
}

/// PUT /lowcode/operations/{id}/buttons — Developer: save buttons (delete-and-recreate)
pub async fn save_buttons(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
    Json(input): Json<SaveOperationButtonsInput>,
) -> Result<Json<ApiResponse<Vec<OperationButton>>>, AppError> {
    // Verify operation exists
    let exists: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM lc_operations WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?;

    if exists.is_none() {
        return Err(AppError::NotFound("Operation not found".to_string()));
    }

    let mut tx = state.pool.begin().await?;

    // Delete existing buttons
    sqlx::query("DELETE FROM lc_operation_buttons WHERE operation_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    // Insert new buttons
    for btn in &input.buttons {
        sqlx::query(
            "INSERT INTO lc_operation_buttons \
             (operation_id, button_key, label, icon, variant, action_type, action_config, \
              confirm_message, required_permission, is_visible, sort_order) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(id)
        .bind(&btn.button_key)
        .bind(&btn.label)
        .bind(&btn.icon)
        .bind(btn.variant.as_deref().unwrap_or("secondary"))
        .bind(btn.action_type.as_deref().unwrap_or("API_CALL"))
        .bind(btn.action_config.as_ref().unwrap_or(&serde_json::json!({})))
        .bind(&btn.confirm_message)
        .bind(&btn.required_permission)
        .bind(btn.is_visible.unwrap_or(true))
        .bind(btn.sort_order)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    // Re-fetch to return saved buttons
    let buttons = sqlx::query_as::<_, OperationButton>(
        "SELECT * FROM lc_operation_buttons WHERE operation_id = $1 ORDER BY sort_order, button_key",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(buttons, "Buttons saved")))
}

/// GET /lowcode/exec/{code}/buttons — User: get buttons for a published operation by code
pub async fn get_buttons_by_code(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformUser>,
    Path(code): Path<String>,
) -> Result<Json<ApiResponse<Vec<OperationButton>>>, AppError> {
    let operation_id: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM lc_operations WHERE operation_code = $1 AND is_published = true",
    )
    .bind(&code)
    .fetch_optional(&state.pool)
    .await?;

    let (op_id,) = operation_id
        .ok_or_else(|| AppError::NotFound("Published operation not found".to_string()))?;

    let buttons = sqlx::query_as::<_, OperationButton>(
        "SELECT * FROM lc_operation_buttons \
         WHERE operation_id = $1 AND is_visible = true \
         ORDER BY sort_order, button_key",
    )
    .bind(op_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(buttons)))
}
