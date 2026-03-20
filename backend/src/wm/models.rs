use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Warehouses
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Warehouse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub warehouse_type: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateWarehouse {
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub address: Option<String>,
    pub warehouse_type: Option<String>,
}

// Storage Bins
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StorageBin {
    pub id: Uuid,
    pub warehouse_id: Uuid,
    pub bin_code: String,
    pub zone: Option<String>,
    pub aisle: Option<String>,
    pub rack: Option<String>,
    pub level: Option<String>,
    pub max_weight: Option<Decimal>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStorageBin {
    pub warehouse_id: Uuid,
    #[validate(length(min = 1, max = 30))]
    pub bin_code: String,
    pub zone: Option<String>,
    pub aisle: Option<String>,
    pub rack: Option<String>,
    pub level: Option<String>,
    pub max_weight: Option<Decimal>,
}

// Stock Transfers
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StockTransfer {
    pub id: Uuid,
    pub transfer_number: String,
    pub from_warehouse_id: Uuid,
    pub to_warehouse_id: Uuid,
    pub material_id: Uuid,
    pub quantity: Decimal,
    pub uom_id: Option<Uuid>,
    pub status: String,
    pub requested_by: Option<Uuid>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStockTransfer {
    pub from_warehouse_id: Uuid,
    pub to_warehouse_id: Uuid,
    pub material_id: Uuid,
    pub quantity: Decimal,
    pub uom_id: Option<Uuid>,
}

// Stock Counts
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StockCount {
    pub id: Uuid,
    pub count_number: String,
    pub warehouse_id: Uuid,
    pub count_date: NaiveDate,
    pub status: String,
    pub counted_by: Option<Uuid>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStockCount {
    pub warehouse_id: Uuid,
    pub count_date: NaiveDate,
    #[validate(length(min = 1))]
    pub items: Vec<CreateStockCountItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateStockCountItem {
    pub material_id: Uuid,
    pub storage_bin_id: Option<Uuid>,
    pub book_quantity: Decimal,
    pub counted_quantity: Option<Decimal>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StockCountItem {
    pub id: Uuid,
    pub stock_count_id: Uuid,
    pub material_id: Uuid,
    pub storage_bin_id: Option<Uuid>,
    pub book_quantity: Decimal,
    pub counted_quantity: Option<Decimal>,
    pub difference: Option<Decimal>,
}

// --- Stock Count Item Sub-table CRUD ---
#[derive(Debug, Deserialize, Validate)]
pub struct AddStockCountItem {
    pub material_id: Uuid,
    pub storage_bin_id: Option<Uuid>,
    pub book_quantity: Decimal,
    pub counted_quantity: Option<Decimal>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStockCountItem {
    pub material_id: Option<Uuid>,
    pub storage_bin_id: Option<Uuid>,
    pub book_quantity: Option<Decimal>,
    pub counted_quantity: Option<Decimal>,
}
