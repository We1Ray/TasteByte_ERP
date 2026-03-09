use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::auth::rbac::{RequireRole, WmRead};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

// --- Warehouse Utilization ---
#[derive(Serialize, sqlx::FromRow)]
pub struct WarehouseUtilizationRow {
    pub warehouse_code: String,
    pub warehouse_name: String,
    pub warehouse_type: String,
    pub total_bins: i64,
    pub active_bins: i64,
    pub bins_with_stock: i64,
    pub utilization_pct: Decimal,
}

pub async fn warehouse_utilization(
    State(state): State<AppState>,
    _role: RequireRole<WmRead>,
) -> Result<Json<ApiResponse<Vec<WarehouseUtilizationRow>>>, AppError> {
    let rows = sqlx::query_as::<_, WarehouseUtilizationRow>(
        "SELECT w.code AS warehouse_code, w.name AS warehouse_name, \
         w.warehouse_type, \
         COUNT(sb.id) AS total_bins, \
         COUNT(CASE WHEN sb.is_active THEN 1 END) AS active_bins, \
         COUNT(DISTINCT sci.storage_bin_id) AS bins_with_stock, \
         CASE WHEN COUNT(sb.id) > 0 THEN \
           ROUND(COUNT(DISTINCT sci.storage_bin_id)::DECIMAL \
                 / COUNT(sb.id)::DECIMAL * 100, 2) \
         ELSE 0 END AS utilization_pct \
         FROM wm_warehouses w \
         LEFT JOIN wm_storage_bins sb ON sb.warehouse_id = w.id \
         LEFT JOIN wm_stock_count_items sci ON sci.storage_bin_id = sb.id \
           AND sci.counted_quantity IS NOT NULL AND sci.counted_quantity > 0 \
         WHERE w.is_active = true \
         GROUP BY w.code, w.name, w.warehouse_type \
         ORDER BY w.code",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}

// --- Transfer Summary ---
#[derive(Deserialize)]
pub struct DateRangeQuery {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TransferSummaryRow {
    pub from_warehouse_code: String,
    pub from_warehouse_name: String,
    pub to_warehouse_code: String,
    pub to_warehouse_name: String,
    pub transfer_count: i64,
    pub total_quantity: Decimal,
}

pub async fn transfer_summary(
    State(state): State<AppState>,
    _role: RequireRole<WmRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<Vec<TransferSummaryRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, TransferSummaryRow>(
        "SELECT wf.code AS from_warehouse_code, wf.name AS from_warehouse_name, \
         wt.code AS to_warehouse_code, wt.name AS to_warehouse_name, \
         COUNT(*) AS transfer_count, SUM(st.quantity) AS total_quantity \
         FROM wm_stock_transfers st \
         JOIN wm_warehouses wf ON wf.id = st.from_warehouse_id \
         JOIN wm_warehouses wt ON wt.id = st.to_warehouse_id \
         WHERE st.created_at::date BETWEEN $1 AND $2 \
         GROUP BY wf.code, wf.name, wt.code, wt.name \
         ORDER BY transfer_count DESC",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}
