use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::auth::rbac::{PpRead, RequireRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

// --- Production Analysis ---
#[derive(Deserialize)]
pub struct DateRangeQuery {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ProductionAnalysisRow {
    pub order_number: String,
    pub material_name: String,
    pub planned_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub status: String,
    pub completion_rate: Decimal,
}

pub async fn production_analysis(
    State(state): State<AppState>,
    _role: RequireRole<PpRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<Vec<ProductionAnalysisRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, ProductionAnalysisRow>(
        "SELECT po.order_number, m.name AS material_name, \
         po.planned_quantity, po.actual_quantity, po.status, \
         CASE WHEN po.planned_quantity > 0 THEN \
           ROUND(po.actual_quantity / po.planned_quantity * 100, 2) \
         ELSE 0 END AS completion_rate \
         FROM pp_production_orders po \
         JOIN mm_materials m ON m.id = po.material_id \
         WHERE po.planned_start BETWEEN $1 AND $2 \
            OR po.planned_start IS NULL \
         ORDER BY po.order_number",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}

// --- Production Lead Time ---
#[derive(Serialize, sqlx::FromRow)]
pub struct ProductionLeadTimeRow {
    pub material_name: String,
    pub completed_orders: i64,
    pub avg_lead_days: Option<f64>,
}

pub async fn production_lead_time(
    State(state): State<AppState>,
    _role: RequireRole<PpRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<Vec<ProductionLeadTimeRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, ProductionLeadTimeRow>(
        "SELECT m.name AS material_name, \
         COUNT(*) AS completed_orders, \
         AVG(po.actual_end - po.actual_start)::FLOAT8 AS avg_lead_days \
         FROM pp_production_orders po \
         JOIN mm_materials m ON m.id = po.material_id \
         WHERE po.status = 'COMPLETED' \
           AND po.actual_start IS NOT NULL AND po.actual_end IS NOT NULL \
           AND po.actual_end BETWEEN $1 AND $2 \
         GROUP BY m.name \
         ORDER BY avg_lead_days DESC NULLS LAST",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}
