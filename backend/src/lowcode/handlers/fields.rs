use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformDeveloper, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

pub async fn get_field(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<FieldWithOptions>>, AppError> {
    let field =
        sqlx::query_as::<_, FieldDefinition>("SELECT * FROM lc_field_definitions WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Field not found".to_string()))?;

    let options = sqlx::query_as::<_, FieldOption>(
        "SELECT * FROM lc_field_options WHERE field_id = $1 ORDER BY sort_order",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(FieldWithOptions {
        field,
        options,
    })))
}

pub async fn delete_field(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let result = sqlx::query("DELETE FROM lc_field_definitions WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Field not found".to_string()));
    }

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "deleted": true }),
        "Field deleted",
    )))
}
