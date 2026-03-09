use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Cost Centers
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CostCenter {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub responsible_person: Option<Uuid>,
    pub is_active: bool,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCostCenter {
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub responsible_person: Option<Uuid>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
}

// Profit Centers
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProfitCenter {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub responsible_person: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProfitCenter {
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub responsible_person: Option<Uuid>,
}

// Internal Orders
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct InternalOrder {
    pub id: Uuid,
    pub order_number: String,
    pub name: String,
    pub order_type: String,
    pub cost_center_id: Option<Uuid>,
    pub status: String,
    pub budget: Decimal,
    pub actual_cost: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInternalOrder {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(min = 1, max = 50))]
    pub order_type: String,
    pub cost_center_id: Option<Uuid>,
    pub budget: Option<Decimal>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateInternalOrder {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub status: Option<String>,
    pub budget: Option<Decimal>,
    pub actual_cost: Option<Decimal>,
}

// Cost Allocations
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CostAllocation {
    pub id: Uuid,
    pub from_cost_center_id: Uuid,
    pub to_cost_center_id: Uuid,
    pub allocation_date: NaiveDate,
    pub amount: Decimal,
    pub description: Option<String>,
    pub source_module: Option<String>,
    pub reference_id: Option<Uuid>,
    pub profit_center_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCostAllocation {
    pub from_cost_center_id: Uuid,
    pub to_cost_center_id: Uuid,
    pub allocation_date: NaiveDate,
    pub amount: Decimal,
    pub description: Option<String>,
}

/// Input for auto-posting cost allocations from other modules.
#[derive(Debug)]
pub struct AutoPostCostAllocation {
    pub from_cost_center_id: Uuid,
    pub to_cost_center_id: Uuid,
    pub allocation_date: NaiveDate,
    pub amount: Decimal,
    pub description: String,
    pub source_module: String,
    pub reference_id: Uuid,
    pub profit_center_id: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn create_cost_center_valid() {
        let input = CreateCostCenter {
            code: "CC001".to_string(),
            name: "Engineering".to_string(),
            description: None,
            responsible_person: None,
            valid_from: None,
            valid_to: None,
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_cost_center_empty_code_rejected() {
        let input = CreateCostCenter {
            code: "".to_string(),
            name: "Engineering".to_string(),
            description: None,
            responsible_person: None,
            valid_from: None,
            valid_to: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn create_cost_center_empty_name_rejected() {
        let input = CreateCostCenter {
            code: "CC001".to_string(),
            name: "".to_string(),
            description: None,
            responsible_person: None,
            valid_from: None,
            valid_to: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn create_profit_center_valid() {
        let input = CreateProfitCenter {
            code: "PC001".to_string(),
            name: "Sales Division".to_string(),
            description: Some("Main sales".to_string()),
            responsible_person: None,
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_internal_order_valid() {
        let input = CreateInternalOrder {
            name: "Marketing Campaign".to_string(),
            order_type: "MARKETING".to_string(),
            cost_center_id: None,
            budget: Some(Decimal::new(50000, 0)),
        };
        assert!(input.validate().is_ok());
    }

    #[test]
    fn create_internal_order_empty_name_rejected() {
        let input = CreateInternalOrder {
            name: "".to_string(),
            order_type: "MARKETING".to_string(),
            cost_center_id: None,
            budget: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn update_internal_order_empty_name_rejected() {
        let input = UpdateInternalOrder {
            name: Some("".to_string()),
            status: None,
            budget: None,
            actual_cost: None,
        };
        assert!(input.validate().is_err());
    }

    #[test]
    fn update_internal_order_none_fields_valid() {
        let input = UpdateInternalOrder {
            name: None,
            status: None,
            budget: None,
            actual_cost: None,
        };
        assert!(input.validate().is_ok());
    }
}
