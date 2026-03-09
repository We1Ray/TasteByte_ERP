use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformDeveloper, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

/// Get the list definition for an operation, including columns and actions.
/// Creates a default definition if none exists yet.
pub async fn get_list_definition(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(operation_id): Path<Uuid>,
) -> Result<Json<ApiResponse<ListResponse>>, AppError> {
    // Verify operation exists
    sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE id = $1")
        .bind(operation_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    let list = get_or_create_list_definition(&state, operation_id).await?;

    let columns = sqlx::query_as::<_, ListColumnRow>(
        "SELECT * FROM lc_list_columns WHERE list_id = $1 ORDER BY sort_order",
    )
    .bind(list.id)
    .fetch_all(&state.pool)
    .await?;

    let actions = sqlx::query_as::<_, ListActionRow>(
        "SELECT * FROM lc_list_actions WHERE list_id = $1 ORDER BY sort_order",
    )
    .bind(list.id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(ListResponse {
        list,
        columns,
        actions,
    })))
}

/// Save (upsert) the list definition, replacing all columns and actions.
pub async fn save_list_definition(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(operation_id): Path<Uuid>,
    Json(input): Json<SaveListRequest>,
) -> Result<Json<ApiResponse<ListResponse>>, AppError> {
    // Verify operation exists
    sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE id = $1")
        .bind(operation_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    let mut tx = state.pool.begin().await?;

    // Upsert list definition
    let list = sqlx::query_as::<_, ListDefinitionRow>(
        "INSERT INTO lc_list_definitions (operation_id, data_source_sql, default_page_size, enable_search, enable_export, enable_import, settings) \
         VALUES ($1, $2, $3, $4, $5, $6, $7) \
         ON CONFLICT (operation_id) DO UPDATE SET \
             data_source_sql = EXCLUDED.data_source_sql, \
             default_page_size = EXCLUDED.default_page_size, \
             enable_search = EXCLUDED.enable_search, \
             enable_export = EXCLUDED.enable_export, \
             enable_import = EXCLUDED.enable_import, \
             settings = EXCLUDED.settings, \
             version = lc_list_definitions.version + 1, \
             updated_at = NOW() \
         RETURNING *",
    )
    .bind(operation_id)
    .bind(&input.data_source_sql)
    .bind(input.default_page_size.unwrap_or(20))
    .bind(input.enable_search.unwrap_or(true))
    .bind(input.enable_export.unwrap_or(false))
    .bind(input.enable_import.unwrap_or(false))
    .bind(input.settings.as_ref().unwrap_or(&serde_json::json!({})))
    .fetch_one(&mut *tx)
    .await?;

    // Delete existing columns and actions (CASCADE from list_id FK handles child rows)
    sqlx::query("DELETE FROM lc_list_columns WHERE list_id = $1")
        .bind(list.id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM lc_list_actions WHERE list_id = $1")
        .bind(list.id)
        .execute(&mut *tx)
        .await?;

    // Insert columns
    for col in &input.columns {
        sqlx::query(
            "INSERT INTO lc_list_columns (list_id, field_key, label, data_type, width, min_width, is_sortable, is_filterable, is_visible, format_pattern, cell_renderer, sort_order) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .bind(list.id)
        .bind(&col.field_key)
        .bind(&col.label)
        .bind(col.data_type.as_deref().unwrap_or("TEXT"))
        .bind(col.width)
        .bind(col.min_width)
        .bind(col.is_sortable.unwrap_or(true))
        .bind(col.is_filterable.unwrap_or(false))
        .bind(col.is_visible.unwrap_or(true))
        .bind(&col.format_pattern)
        .bind(&col.cell_renderer)
        .bind(col.sort_order)
        .execute(&mut *tx)
        .await?;
    }

    // Insert actions
    for action in &input.actions {
        sqlx::query(
            "INSERT INTO lc_list_actions (list_id, action_key, label, icon, action_type, target_url, confirm_message, sort_order) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(list.id)
        .bind(&action.action_key)
        .bind(&action.label)
        .bind(&action.icon)
        .bind(action.action_type.as_deref().unwrap_or("NAVIGATE"))
        .bind(&action.target_url)
        .bind(&action.confirm_message)
        .bind(action.sort_order)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    // Re-fetch full response
    let columns = sqlx::query_as::<_, ListColumnRow>(
        "SELECT * FROM lc_list_columns WHERE list_id = $1 ORDER BY sort_order",
    )
    .bind(list.id)
    .fetch_all(&state.pool)
    .await?;

    let actions = sqlx::query_as::<_, ListActionRow>(
        "SELECT * FROM lc_list_actions WHERE list_id = $1 ORDER BY sort_order",
    )
    .bind(list.id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        ListResponse {
            list,
            columns,
            actions,
        },
        "List definition saved",
    )))
}

/// Get or create a default list definition for the given operation.
async fn get_or_create_list_definition(
    state: &AppState,
    operation_id: Uuid,
) -> Result<ListDefinitionRow, AppError> {
    let existing = sqlx::query_as::<_, ListDefinitionRow>(
        "SELECT * FROM lc_list_definitions WHERE operation_id = $1",
    )
    .bind(operation_id)
    .fetch_optional(&state.pool)
    .await?;

    match existing {
        Some(list) => Ok(list),
        None => {
            let list = sqlx::query_as::<_, ListDefinitionRow>(
                "INSERT INTO lc_list_definitions (operation_id) VALUES ($1) RETURNING *",
            )
            .bind(operation_id)
            .fetch_one(&state.pool)
            .await?;
            Ok(list)
        }
    }
}
