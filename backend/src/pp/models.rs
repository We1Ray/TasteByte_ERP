use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// BOMs
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Bom {
    pub id: Uuid,
    pub bom_number: String,
    pub material_id: Uuid,
    pub name: String,
    pub version: i32,
    pub status: String,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBom {
    pub material_id: Uuid,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    #[validate(length(min = 1))]
    pub items: Vec<CreateBomItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBomItem {
    pub component_material_id: Uuid,
    pub quantity: Decimal,
    pub uom_id: Option<Uuid>,
    pub scrap_percentage: Option<Decimal>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct BomItem {
    pub id: Uuid,
    pub bom_id: Uuid,
    pub line_number: i32,
    pub component_material_id: Uuid,
    pub quantity: Decimal,
    pub uom_id: Option<Uuid>,
    pub scrap_percentage: Decimal,
}

// Routings
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Routing {
    pub id: Uuid,
    pub routing_number: String,
    pub material_id: Uuid,
    pub name: String,
    pub version: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRouting {
    pub material_id: Uuid,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(min = 1))]
    pub operations: Vec<CreateRoutingOperation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRoutingOperation {
    pub operation_number: i32,
    pub work_center: String,
    pub description: Option<String>,
    pub setup_time_minutes: Option<i32>,
    pub run_time_minutes: Option<i32>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct RoutingOperation {
    pub id: Uuid,
    pub routing_id: Uuid,
    pub operation_number: i32,
    pub work_center: String,
    pub description: Option<String>,
    pub setup_time_minutes: i32,
    pub run_time_minutes: i32,
}

// Production Orders
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProductionOrder {
    pub id: Uuid,
    pub order_number: String,
    pub material_id: Uuid,
    pub bom_id: Uuid,
    pub routing_id: Option<Uuid>,
    pub planned_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub uom_id: Option<Uuid>,
    pub planned_start: Option<NaiveDate>,
    pub planned_end: Option<NaiveDate>,
    pub actual_start: Option<NaiveDate>,
    pub actual_end: Option<NaiveDate>,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductionOrder {
    pub material_id: Uuid,
    pub bom_id: Uuid,
    pub routing_id: Option<Uuid>,
    pub planned_quantity: Decimal,
    pub uom_id: Option<Uuid>,
    pub planned_start: Option<NaiveDate>,
    pub planned_end: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductionOrderStatus {
    #[validate(length(min = 1))]
    pub status: String,
    pub actual_quantity: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmProductionOrder {
    pub quantity: Decimal,
}
