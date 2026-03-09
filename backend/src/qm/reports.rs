use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::auth::rbac::{QmRead, RequireRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

// --- Inspection Pass Rate ---
#[derive(Deserialize)]
pub struct DateRangeQuery {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct InspectionPassRateRow {
    pub material_name: String,
    pub total_inspections: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: Decimal,
}

pub async fn inspection_pass_rate(
    State(state): State<AppState>,
    _role: RequireRole<QmRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<Vec<InspectionPassRateRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, InspectionPassRateRow>(
        "SELECT m.name AS material_name, \
         COUNT(il.id) AS total_inspections, \
         COUNT(CASE WHEN il.status = 'ACCEPTED' THEN 1 END) AS passed, \
         COUNT(CASE WHEN il.status = 'REJECTED' THEN 1 END) AS failed, \
         CASE WHEN COUNT(il.id) > 0 THEN \
           ROUND(COUNT(CASE WHEN il.status = 'ACCEPTED' THEN 1 END)::DECIMAL \
                 / COUNT(il.id)::DECIMAL * 100, 2) \
         ELSE 0 END AS pass_rate \
         FROM qm_inspection_lots il \
         JOIN mm_materials m ON m.id = il.material_id \
         WHERE il.created_at::date BETWEEN $1 AND $2 \
         GROUP BY m.name \
         ORDER BY pass_rate ASC",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}

// --- Notification Summary ---
#[derive(Serialize, sqlx::FromRow)]
pub struct NotificationSummaryRow {
    pub priority: String,
    pub status: String,
    pub count: i64,
}

pub async fn notification_summary(
    State(state): State<AppState>,
    _role: RequireRole<QmRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<Vec<NotificationSummaryRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, NotificationSummaryRow>(
        "SELECT priority, status, COUNT(*) AS count \
         FROM qm_quality_notifications \
         WHERE created_at::date BETWEEN $1 AND $2 \
         GROUP BY priority, status \
         ORDER BY priority, status",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}
