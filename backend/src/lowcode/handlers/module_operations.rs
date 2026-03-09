use axum::extract::{Path, State};
use axum::Json;

use crate::lowcode::models::ModuleOperationSummary;
use crate::lowcode::services::permission_resolver::{PlatformUser, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

const VALID_MODULES: &[&str] = &["FI", "CO", "MM", "SD", "PP", "HR", "WM", "QM"];

pub async fn list_module_operations(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformUser>,
    Path(module): Path<String>,
) -> Result<Json<ApiResponse<Vec<ModuleOperationSummary>>>, AppError> {
    let module_upper = module.to_uppercase();
    if !VALID_MODULES.contains(&module_upper.as_str()) {
        return Err(AppError::Validation(format!(
            "Invalid module: {}. Valid modules: {:?}",
            module, VALID_MODULES
        )));
    }

    let ops = sqlx::query_as::<_, ModuleOperationSummary>(
        "SELECT id, operation_code, name, operation_type, sidebar_icon, sidebar_sort_order \
         FROM lc_operations \
         WHERE module = $1 AND is_published = true \
         ORDER BY sidebar_sort_order, name",
    )
    .bind(&module_upper)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(ops)))
}
