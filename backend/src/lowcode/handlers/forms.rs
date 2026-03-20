use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::form_builder;
use crate::lowcode::services::permission_resolver::{PlatformDeveloper, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

pub async fn export_yaml(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(code): Path<String>,
) -> Result<axum::response::Response, AppError> {
    use axum::http::header;
    use axum::response::IntoResponse;
    let yaml =
        crate::lowcode::yaml_sync::exporter::export_operation(&state.pool, &code).await?;
    let filename = format!("{}.yaml", code.to_lowercase());
    Ok((
        [
            (header::CONTENT_TYPE, "application/x-yaml".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        yaml,
    )
        .into_response())
}

pub async fn get_form(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<FormResponse>>, AppError> {
    // Verify operation exists
    sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    let form = form_builder::get_form(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(form)))
}

pub async fn save_form(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
    Json(input): Json<SaveFormRequest>,
) -> Result<Json<ApiResponse<FormResponse>>, AppError> {
    if input.sections.is_empty() {
        return Err(AppError::Validation(
            "At least one section is required".to_string(),
        ));
    }

    // Verify operation exists
    sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    let form = form_builder::save_form(&state.pool, id, input).await?;
    Ok(Json(ApiResponse::with_message(form, "Form saved")))
}
