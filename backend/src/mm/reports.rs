use axum::extract::{Query, State};
use axum::Json;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::rbac::{MmRead, RequireRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

// --- Stock Valuation ---
#[derive(Serialize, sqlx::FromRow)]
pub struct StockValuationRow {
    pub material_id: Uuid,
    pub material_number: String,
    pub material_name: String,
    pub warehouse_id: Option<Uuid>,
    pub quantity: Decimal,
    pub reserved_quantity: Decimal,
    pub available_quantity: Decimal,
}

pub async fn stock_valuation(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
) -> Result<Json<ApiResponse<Vec<StockValuationRow>>>, AppError> {
    let rows = sqlx::query_as::<_, StockValuationRow>(
        "SELECT ps.material_id, m.material_number, m.name AS material_name, \
         ps.warehouse_id, ps.quantity, ps.reserved_quantity, \
         ps.quantity - ps.reserved_quantity AS available_quantity \
         FROM mm_plant_stock ps \
         JOIN mm_materials m ON m.id = ps.material_id \
         WHERE m.is_active = true \
         ORDER BY m.material_number",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}

// --- Movement Summary ---
#[derive(Deserialize)]
pub struct MovementSummaryQuery {
    pub from_date: Option<chrono::NaiveDate>,
    pub to_date: Option<chrono::NaiveDate>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct MovementSummaryRow {
    pub movement_type: String,
    pub material_id: Uuid,
    pub material_number: String,
    pub material_name: String,
    pub total_quantity: Decimal,
    pub movement_count: i64,
}

pub async fn movement_summary(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Query(query): Query<MovementSummaryQuery>,
) -> Result<Json<ApiResponse<Vec<MovementSummaryRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(chrono::NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, MovementSummaryRow>(
        "SELECT mv.movement_type, mv.material_id, m.material_number, m.name AS material_name, \
         SUM(mv.quantity) AS total_quantity, COUNT(*) AS movement_count \
         FROM mm_material_movements mv \
         JOIN mm_materials m ON m.id = mv.material_id \
         WHERE mv.posted_at::date BETWEEN $1 AND $2 \
         GROUP BY mv.movement_type, mv.material_id, m.material_number, m.name \
         ORDER BY mv.movement_type, m.material_number",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}

// --- Slow Moving Items ---
#[derive(Deserialize)]
pub struct SlowMovingQuery {
    pub days: Option<i32>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct SlowMovingRow {
    pub material_id: Uuid,
    pub material_number: String,
    pub material_name: String,
    pub quantity: Decimal,
    pub last_movement_at: Option<chrono::DateTime<chrono::Utc>>,
    pub days_since_movement: Option<i32>,
}

pub async fn slow_moving(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Query(query): Query<SlowMovingQuery>,
) -> Result<Json<ApiResponse<Vec<SlowMovingRow>>>, AppError> {
    let threshold_days = query.days.unwrap_or(90);

    let rows = sqlx::query_as::<_, SlowMovingRow>(
        "SELECT ps.material_id, m.material_number, m.name AS material_name, ps.quantity, \
         latest.last_movement_at, \
         EXTRACT(DAY FROM NOW() - latest.last_movement_at)::INT AS days_since_movement \
         FROM mm_plant_stock ps \
         JOIN mm_materials m ON m.id = ps.material_id \
         LEFT JOIN LATERAL ( \
             SELECT MAX(posted_at) AS last_movement_at \
             FROM mm_material_movements mv WHERE mv.material_id = ps.material_id \
         ) latest ON true \
         WHERE ps.quantity > 0 AND m.is_active = true \
         AND (latest.last_movement_at IS NULL \
              OR latest.last_movement_at < NOW() - make_interval(days => $1)) \
         ORDER BY latest.last_movement_at ASC NULLS FIRST",
    )
    .bind(threshold_days)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}
