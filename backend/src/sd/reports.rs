use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::rbac::{RequireRole, SdRead};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

// --- Sales Summary ---
#[derive(Deserialize)]
pub struct SalesSummaryQuery {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub group_by: Option<String>, // "customer" or "material"
}

#[derive(Serialize, sqlx::FromRow)]
pub struct SalesSummaryRow {
    pub group_key: String,
    pub group_name: String,
    pub order_count: i64,
    pub total_amount: Decimal,
}

pub async fn sales_summary(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Query(query): Query<SalesSummaryQuery>,
) -> Result<Json<ApiResponse<Vec<SalesSummaryRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());
    let group_by = query.group_by.as_deref().unwrap_or("customer");

    let rows = match group_by {
        "material" => {
            sqlx::query_as::<_, SalesSummaryRow>(
                "SELECT m.material_number AS group_key, m.name AS group_name, \
                 COUNT(DISTINCT so.id) AS order_count, SUM(soi.total_price) AS total_amount \
                 FROM sd_sales_orders so \
                 JOIN sd_sales_order_items soi ON soi.sales_order_id = so.id \
                 JOIN mm_materials m ON m.id = soi.material_id \
                 WHERE so.order_date BETWEEN $1 AND $2 AND so.status != 'CANCELLED' \
                 GROUP BY m.material_number, m.name \
                 ORDER BY total_amount DESC",
            )
            .bind(from)
            .bind(to)
            .fetch_all(&state.pool)
            .await?
        }
        _ => {
            sqlx::query_as::<_, SalesSummaryRow>(
                "SELECT c.customer_number AS group_key, c.name AS group_name, \
                 COUNT(so.id) AS order_count, SUM(so.total_amount) AS total_amount \
                 FROM sd_sales_orders so \
                 JOIN sd_customers c ON c.id = so.customer_id \
                 WHERE so.order_date BETWEEN $1 AND $2 AND so.status != 'CANCELLED' \
                 GROUP BY c.customer_number, c.name \
                 ORDER BY total_amount DESC",
            )
            .bind(from)
            .bind(to)
            .fetch_all(&state.pool)
            .await?
        }
    };

    Ok(Json(ApiResponse::success(rows)))
}

// --- Order Fulfillment ---
#[derive(Serialize, sqlx::FromRow)]
pub struct OrderFulfillmentRow {
    pub total_orders: i64,
    pub fully_delivered: i64,
    pub partially_delivered: i64,
    pub not_delivered: i64,
    pub fulfillment_rate: Decimal,
}

pub async fn order_fulfillment(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Query(query): Query<SalesSummaryQuery>,
) -> Result<Json<ApiResponse<OrderFulfillmentRow>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let row = sqlx::query_as::<_, OrderFulfillmentRow>(
        "SELECT COUNT(*) AS total_orders, \
         COUNT(*) FILTER (WHERE status IN ('DELIVERED', 'CLOSED')) AS fully_delivered, \
         COUNT(*) FILTER (WHERE status = 'PARTIALLY_DELIVERED') AS partially_delivered, \
         COUNT(*) FILTER (WHERE status IN ('DRAFT', 'CONFIRMED')) AS not_delivered, \
         CASE WHEN COUNT(*) > 0 THEN \
           ROUND(COUNT(*) FILTER (WHERE status IN ('DELIVERED', 'CLOSED'))::DECIMAL \
                 / COUNT(*)::DECIMAL * 100, 2) \
         ELSE 0 END AS fulfillment_rate \
         FROM sd_sales_orders \
         WHERE order_date BETWEEN $1 AND $2 AND status != 'CANCELLED'",
    )
    .bind(from)
    .bind(to)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(row)))
}

// --- Top Customers ---
#[derive(Deserialize)]
pub struct TopNQuery {
    pub limit: Option<i64>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TopCustomerRow {
    pub customer_id: Uuid,
    pub customer_number: String,
    pub customer_name: String,
    pub order_count: i64,
    pub total_amount: Decimal,
}

pub async fn top_customers(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Query(query): Query<TopNQuery>,
) -> Result<Json<ApiResponse<Vec<TopCustomerRow>>>, AppError> {
    let limit = query.limit.unwrap_or(10);
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, TopCustomerRow>(
        "SELECT c.id AS customer_id, c.customer_number, c.name AS customer_name, \
         COUNT(so.id) AS order_count, COALESCE(SUM(so.total_amount), 0) AS total_amount \
         FROM sd_customers c \
         LEFT JOIN sd_sales_orders so ON so.customer_id = c.id \
             AND so.order_date BETWEEN $2 AND $3 AND so.status != 'CANCELLED' \
         WHERE c.is_active = true \
         GROUP BY c.id, c.customer_number, c.name \
         ORDER BY total_amount DESC \
         LIMIT $1",
    )
    .bind(limit)
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}
