use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Customers
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub customer_number: String,
    pub name: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub payment_terms: i32,
    pub credit_limit: Decimal,
    pub profit_center_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCustomer {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub payment_terms: Option<i32>,
    pub credit_limit: Option<Decimal>,
    pub profit_center_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCustomer {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub payment_terms: Option<i32>,
    pub credit_limit: Option<Decimal>,
    pub profit_center_id: Option<Uuid>,
    pub is_active: Option<bool>,
}

// Sales Orders
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SalesOrder {
    pub id: Uuid,
    pub order_number: String,
    pub customer_id: Uuid,
    pub order_date: NaiveDate,
    pub requested_delivery_date: Option<NaiveDate>,
    pub status: String,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSalesOrder {
    pub customer_id: Uuid,
    pub order_date: NaiveDate,
    pub requested_delivery_date: Option<NaiveDate>,
    pub notes: Option<String>,
    #[validate(length(min = 1, message = "At least one item is required"))]
    pub items: Vec<CreateSalesOrderItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSalesOrderItem {
    pub material_id: Uuid,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub uom_id: Option<Uuid>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SalesOrderItem {
    pub id: Uuid,
    pub sales_order_id: Uuid,
    pub line_number: i32,
    pub material_id: Uuid,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub total_price: Decimal,
    pub uom_id: Option<Uuid>,
    pub delivered_quantity: Decimal,
}

// Deliveries
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Delivery {
    pub id: Uuid,
    pub delivery_number: String,
    pub sales_order_id: Uuid,
    pub delivery_date: NaiveDate,
    pub status: String,
    pub shipped_by: Option<Uuid>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDelivery {
    pub sales_order_id: Uuid,
    pub delivery_date: NaiveDate,
    #[validate(length(min = 1))]
    pub items: Vec<CreateDeliveryItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDeliveryItem {
    pub sales_order_item_id: Uuid,
    pub quantity: Decimal,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DeliveryItem {
    pub id: Uuid,
    pub delivery_id: Uuid,
    pub sales_order_item_id: Uuid,
    pub quantity: Decimal,
    pub warehouse_id: Option<Uuid>,
}

// SD Invoices
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SdInvoice {
    pub id: Uuid,
    pub invoice_number: String,
    pub sales_order_id: Uuid,
    pub delivery_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_amount: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSdInvoice {
    pub sales_order_id: Uuid,
    pub delivery_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_amount: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn create_customer_valid() {
        let input = CreateCustomer {
            name: "Acme Corp".to_string(),
            contact_person: None,
            email: Some("info@acme.com".to_string()),
            phone: None,
            address: None,
            payment_terms: Some(30),
            credit_limit: None,
            profit_center_id: None,
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_customer_empty_name_rejected() {
        let input = CreateCustomer {
            name: "".to_string(),
            contact_person: None,
            email: None,
            phone: None,
            address: None,
            payment_terms: None,
            credit_limit: None,
            profit_center_id: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn update_customer_empty_name_rejected() {
        let input = UpdateCustomer {
            name: Some("".to_string()),
            contact_person: None,
            email: None,
            phone: None,
            address: None,
            payment_terms: None,
            credit_limit: None,
            profit_center_id: None,
            is_active: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn update_customer_none_fields_valid() {
        let input = UpdateCustomer {
            name: None,
            contact_person: None,
            email: None,
            phone: None,
            address: None,
            payment_terms: None,
            credit_limit: None,
            profit_center_id: None,
            is_active: None,
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_so_empty_items_rejected() {
        let input = CreateSalesOrder {
            customer_id: Uuid::new_v4(),
            order_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            requested_delivery_date: None,
            notes: None,
            items: vec![],
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn create_so_with_items_valid() {
        let input = CreateSalesOrder {
            customer_id: Uuid::new_v4(),
            order_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            requested_delivery_date: None,
            notes: None,
            items: vec![CreateSalesOrderItem {
                material_id: Uuid::new_v4(),
                quantity: Decimal::new(5, 0),
                unit_price: Decimal::new(100, 0),
                uom_id: None,
            }],
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_delivery_empty_items_rejected() {
        let input = CreateDelivery {
            sales_order_id: Uuid::new_v4(),
            delivery_date: NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
            items: vec![],
        };
        assert!(input.validate().is_err());
    }
}
