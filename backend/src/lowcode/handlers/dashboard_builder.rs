use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformDeveloper, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

/// Get the dashboard definition for an operation, including widgets.
/// Creates a default definition if none exists yet.
pub async fn get_dashboard(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(operation_id): Path<Uuid>,
) -> Result<Json<ApiResponse<DashboardResponse>>, AppError> {
    // Verify operation exists
    sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE id = $1")
        .bind(operation_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    let dashboard = get_or_create_dashboard(&state, operation_id).await?;

    let widgets = sqlx::query_as::<_, DashboardWidgetRow>(
        "SELECT * FROM lc_dashboard_widgets WHERE dashboard_id = $1 ORDER BY sort_order",
    )
    .bind(dashboard.id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(DashboardResponse {
        dashboard,
        widgets,
    })))
}

/// Save (upsert) the dashboard definition, replacing all widgets.
pub async fn save_dashboard(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(operation_id): Path<Uuid>,
    Json(input): Json<SaveDashboardRequest>,
) -> Result<Json<ApiResponse<DashboardResponse>>, AppError> {
    // Verify operation exists
    sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE id = $1")
        .bind(operation_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    let mut tx = state.pool.begin().await?;

    // Upsert dashboard definition
    let dashboard = sqlx::query_as::<_, DashboardDefinitionRow>(
        "INSERT INTO lc_dashboard_definitions (operation_id, grid_columns, refresh_interval, settings) \
         VALUES ($1, $2, $3, $4) \
         ON CONFLICT (operation_id) DO UPDATE SET \
             grid_columns = EXCLUDED.grid_columns, \
             refresh_interval = EXCLUDED.refresh_interval, \
             settings = EXCLUDED.settings, \
             version = lc_dashboard_definitions.version + 1, \
             updated_at = NOW() \
         RETURNING *",
    )
    .bind(operation_id)
    .bind(input.grid_columns.unwrap_or(12))
    .bind(input.refresh_interval)
    .bind(input.settings.as_ref().unwrap_or(&serde_json::json!({})))
    .fetch_one(&mut *tx)
    .await?;

    // Delete existing widgets
    sqlx::query("DELETE FROM lc_dashboard_widgets WHERE dashboard_id = $1")
        .bind(dashboard.id)
        .execute(&mut *tx)
        .await?;

    // Insert new widgets
    for widget in &input.widgets {
        sqlx::query(
            "INSERT INTO lc_dashboard_widgets \
             (dashboard_id, title, widget_type, data_source_sql, x_axis_key, y_axis_key, \
              series_config, colors, grid_x, grid_y, grid_w, grid_h, widget_config, sort_order) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
        )
        .bind(dashboard.id)
        .bind(&widget.title)
        .bind(&widget.widget_type)
        .bind(&widget.data_source_sql)
        .bind(&widget.x_axis_key)
        .bind(&widget.y_axis_key)
        .bind(
            widget
                .series_config
                .as_ref()
                .unwrap_or(&serde_json::json!([])),
        )
        .bind(widget.colors.as_ref().unwrap_or(&serde_json::json!([])))
        .bind(widget.grid_x)
        .bind(widget.grid_y)
        .bind(widget.grid_w)
        .bind(widget.grid_h)
        .bind(
            widget
                .widget_config
                .as_ref()
                .unwrap_or(&serde_json::json!({})),
        )
        .bind(widget.sort_order)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    // Re-fetch full response
    let widgets = sqlx::query_as::<_, DashboardWidgetRow>(
        "SELECT * FROM lc_dashboard_widgets WHERE dashboard_id = $1 ORDER BY sort_order",
    )
    .bind(dashboard.id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        DashboardResponse { dashboard, widgets },
        "Dashboard saved",
    )))
}

/// Get or create a default dashboard definition for the given operation.
async fn get_or_create_dashboard(
    state: &AppState,
    operation_id: Uuid,
) -> Result<DashboardDefinitionRow, AppError> {
    let existing = sqlx::query_as::<_, DashboardDefinitionRow>(
        "SELECT * FROM lc_dashboard_definitions WHERE operation_id = $1",
    )
    .bind(operation_id)
    .fetch_optional(&state.pool)
    .await?;

    match existing {
        Some(d) => Ok(d),
        None => {
            let d = sqlx::query_as::<_, DashboardDefinitionRow>(
                "INSERT INTO lc_dashboard_definitions (operation_id) VALUES ($1) RETURNING *",
            )
            .bind(operation_id)
            .fetch_one(&state.pool)
            .await?;
            Ok(d)
        }
    }
}
