use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::auth::rbac::{CoRead, RequireRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

// --- Cost Center Summary ---
#[derive(Deserialize)]
pub struct DateRangeQuery {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CostCenterSummaryRow {
    pub cost_center_code: String,
    pub cost_center_name: String,
    pub total_allocated_in: Decimal,
    pub total_allocated_out: Decimal,
    pub net_allocation: Decimal,
}

pub async fn cost_center_summary(
    State(state): State<AppState>,
    _role: RequireRole<CoRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<Vec<CostCenterSummaryRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, CostCenterSummaryRow>(
        "SELECT cc.code AS cost_center_code, cc.name AS cost_center_name, \
         COALESCE(alloc_in.total_in, 0) AS total_allocated_in, \
         COALESCE(alloc_out.total_out, 0) AS total_allocated_out, \
         COALESCE(alloc_in.total_in, 0) - COALESCE(alloc_out.total_out, 0) AS net_allocation \
         FROM co_cost_centers cc \
         LEFT JOIN ( \
           SELECT to_cost_center_id AS cc_id, SUM(amount) AS total_in \
           FROM co_cost_allocations \
           WHERE allocation_date BETWEEN $1 AND $2 \
           GROUP BY to_cost_center_id \
         ) alloc_in ON alloc_in.cc_id = cc.id \
         LEFT JOIN ( \
           SELECT from_cost_center_id AS cc_id, SUM(amount) AS total_out \
           FROM co_cost_allocations \
           WHERE allocation_date BETWEEN $1 AND $2 \
           GROUP BY from_cost_center_id \
         ) alloc_out ON alloc_out.cc_id = cc.id \
         WHERE cc.is_active = true \
         ORDER BY cc.code",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}

// --- Internal Order Budget vs Actual ---
#[derive(Serialize, sqlx::FromRow)]
pub struct InternalOrderBudgetRow {
    pub order_number: String,
    pub order_name: String,
    pub order_type: String,
    pub status: String,
    pub budget: Decimal,
    pub actual_cost: Decimal,
    pub variance: Decimal,
    pub utilization_pct: Decimal,
}

pub async fn internal_order_budget(
    State(state): State<AppState>,
    _role: RequireRole<CoRead>,
) -> Result<Json<ApiResponse<Vec<InternalOrderBudgetRow>>>, AppError> {
    let rows = sqlx::query_as::<_, InternalOrderBudgetRow>(
        "SELECT order_number, name AS order_name, order_type, status, \
         budget, actual_cost, \
         actual_cost - budget AS variance, \
         CASE WHEN budget > 0 THEN \
           ROUND(actual_cost / budget * 100, 2) \
         ELSE 0 END AS utilization_pct \
         FROM co_internal_orders \
         ORDER BY order_number",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}
