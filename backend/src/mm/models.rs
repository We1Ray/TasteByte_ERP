use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// UOM
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Uom {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub is_base: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUom {
    #[validate(length(min = 1, max = 10))]
    pub code: String,
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub is_base: Option<bool>,
}

// Material Groups
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MaterialGroup {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMaterialGroup {
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
}

// Materials
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Material {
    pub id: Uuid,
    pub material_number: String,
    pub name: String,
    pub description: Option<String>,
    pub material_group_id: Option<Uuid>,
    pub base_uom_id: Option<Uuid>,
    pub material_type: String,
    pub weight: Option<Decimal>,
    pub weight_uom: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMaterial {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub material_group_id: Option<Uuid>,
    pub base_uom_id: Option<Uuid>,
    pub material_type: Option<String>,
    pub weight: Option<Decimal>,
    pub weight_uom: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMaterial {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub material_group_id: Option<Uuid>,
    pub base_uom_id: Option<Uuid>,
    pub weight: Option<Decimal>,
    pub weight_uom: Option<String>,
    pub is_active: Option<bool>,
}

// Vendors
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Vendor {
    pub id: Uuid,
    pub vendor_number: String,
    pub name: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub payment_terms: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVendor {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub payment_terms: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateVendor {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub payment_terms: Option<i32>,
    pub is_active: Option<bool>,
}

// Plant Stock
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlantStock {
    pub id: Uuid,
    pub material_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub quantity: Decimal,
    pub reserved_quantity: Decimal,
    pub uom_id: Option<Uuid>,
    pub last_count_date: Option<NaiveDate>,
    pub updated_at: DateTime<Utc>,
}

// Material Movements
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MaterialMovement {
    pub id: Uuid,
    pub document_number: String,
    pub movement_type: String,
    pub material_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub quantity: Decimal,
    pub uom_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub posted_by: Option<Uuid>,
    pub posted_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMaterialMovement {
    pub movement_type: String,
    pub material_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub quantity: Decimal,
    pub uom_id: Option<Uuid>,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
}

// Purchase Orders
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub po_number: String,
    pub vendor_id: Uuid,
    pub order_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub status: String,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePurchaseOrder {
    pub vendor_id: Uuid,
    pub order_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub notes: Option<String>,
    #[validate(length(min = 1, message = "At least one item is required"))]
    pub items: Vec<CreatePurchaseOrderItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePurchaseOrderItem {
    pub material_id: Uuid,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub uom_id: Option<Uuid>,
    pub delivery_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PurchaseOrderItem {
    pub id: Uuid,
    pub purchase_order_id: Uuid,
    pub line_number: i32,
    pub material_id: Uuid,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub total_price: Decimal,
    pub uom_id: Option<Uuid>,
    pub delivery_date: Option<NaiveDate>,
    pub received_quantity: Decimal,
}

// Receive Purchase Order
#[derive(Debug, Deserialize, Validate)]
pub struct ReceivePurchaseOrder {
    #[validate(length(min = 1, message = "At least one item is required"))]
    pub items: Vec<ReceivePurchaseOrderItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReceivePurchaseOrderItem {
    pub po_item_id: Uuid,
    pub quantity: Decimal,
    pub warehouse_id: Option<Uuid>,
}

// Goods Receipt Notes (GRN)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct GoodsReceipt {
    pub id: Uuid,
    pub grn_number: String,
    pub purchase_order_id: Option<Uuid>,
    pub vendor_id: Option<Uuid>,
    pub receipt_date: NaiveDate,
    pub warehouse_id: Option<Uuid>,
    pub status: String,
    pub notes: Option<String>,
    pub received_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct GoodsReceiptItem {
    pub id: Uuid,
    pub goods_receipt_id: Uuid,
    pub po_item_id: Option<Uuid>,
    pub material_id: Uuid,
    pub ordered_quantity: Option<Decimal>,
    pub received_quantity: Decimal,
    pub rejected_quantity: Option<Decimal>,
    pub uom_id: Option<Uuid>,
    pub batch_number: Option<String>,
    pub expiry_date: Option<NaiveDate>,
    pub storage_bin: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGoodsReceipt {
    pub purchase_order_id: Option<Uuid>,
    pub vendor_id: Option<Uuid>,
    pub receipt_date: Option<NaiveDate>,
    pub warehouse_id: Option<Uuid>,
    pub notes: Option<String>,
    #[validate(length(min = 1, message = "At least one item is required"))]
    pub items: Vec<CreateGoodsReceiptItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateGoodsReceiptItem {
    pub po_item_id: Option<Uuid>,
    pub material_id: Uuid,
    pub ordered_quantity: Option<Decimal>,
    pub received_quantity: Decimal,
    pub rejected_quantity: Option<Decimal>,
    pub uom_id: Option<Uuid>,
    pub batch_number: Option<String>,
    pub expiry_date: Option<NaiveDate>,
    pub storage_bin: Option<String>,
    pub notes: Option<String>,
}

// Stock Reservation
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StockReservation {
    pub id: Uuid,
    pub material_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub reserved_quantity: Decimal,
    pub reference_type: String,
    pub reference_id: Uuid,
    pub status: String,
    pub reserved_by: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// Stock Movement (unified ledger)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StockMovement {
    pub id: Uuid,
    pub material_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub movement_type: String,
    pub quantity: Decimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub batch_number: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// Fiscal Period
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FiscalPeriod {
    pub id: Uuid,
    pub fiscal_year_id: Uuid,
    pub period: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_closed: bool,
    pub status: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn create_uom_valid() {
        let input = CreateUom {
            code: "KG".to_string(),
            name: "Kilogram".to_string(),
            is_base: Some(true),
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_uom_empty_code_rejected() {
        let input = CreateUom {
            code: "".to_string(),
            name: "Kilogram".to_string(),
            is_base: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn create_material_valid() {
        let input = CreateMaterial {
            name: "Steel Rod".to_string(),
            description: Some("High grade steel".to_string()),
            material_group_id: None,
            base_uom_id: None,
            material_type: Some("RAW".to_string()),
            weight: None,
            weight_uom: None,
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_material_empty_name_rejected() {
        let input = CreateMaterial {
            name: "".to_string(),
            description: None,
            material_group_id: None,
            base_uom_id: None,
            material_type: None,
            weight: None,
            weight_uom: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn update_material_empty_name_rejected() {
        let input = UpdateMaterial {
            name: Some("".to_string()),
            description: None,
            material_group_id: None,
            base_uom_id: None,
            weight: None,
            weight_uom: None,
            is_active: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn update_material_none_fields_valid() {
        let input = UpdateMaterial {
            name: None,
            description: None,
            material_group_id: None,
            base_uom_id: None,
            weight: None,
            weight_uom: None,
            is_active: None,
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_vendor_valid() {
        let input = CreateVendor {
            name: "Steel Corp".to_string(),
            contact_person: Some("John".to_string()),
            email: None,
            phone: None,
            address: None,
            payment_terms: Some(30),
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_vendor_empty_name_rejected() {
        let input = CreateVendor {
            name: "".to_string(),
            contact_person: None,
            email: None,
            phone: None,
            address: None,
            payment_terms: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn create_po_empty_items_rejected() {
        let input = CreatePurchaseOrder {
            vendor_id: Uuid::new_v4(),
            order_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            delivery_date: None,
            notes: None,
            items: vec![],
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn create_po_with_items_valid() {
        let input = CreatePurchaseOrder {
            vendor_id: Uuid::new_v4(),
            order_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            delivery_date: None,
            notes: None,
            items: vec![CreatePurchaseOrderItem {
                material_id: Uuid::new_v4(),
                quantity: Decimal::new(10, 0),
                unit_price: Decimal::new(100, 0),
                uom_id: None,
                delivery_date: None,
            }],
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn receive_po_empty_items_rejected() {
        let input = ReceivePurchaseOrder { items: vec![] };
        assert!(input.validate().is_err());
    }
}
