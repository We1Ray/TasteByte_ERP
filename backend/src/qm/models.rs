use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Inspection Lots
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct InspectionLot {
    pub id: Uuid,
    pub lot_number: String,
    pub material_id: Uuid,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub inspection_type: String,
    pub planned_quantity: Decimal,
    pub inspected_quantity: Decimal,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInspectionLot {
    pub material_id: Uuid,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub inspection_type: Option<String>,
    pub planned_quantity: Decimal,
}

// Inspection Results
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct InspectionResult {
    pub id: Uuid,
    pub inspection_lot_id: Uuid,
    pub characteristic: String,
    pub target_value: Option<String>,
    pub actual_value: Option<String>,
    pub is_conforming: Option<bool>,
    pub inspected_by: Option<Uuid>,
    pub inspected_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInspectionResult {
    pub inspection_lot_id: Uuid,
    #[validate(length(min = 1, max = 200))]
    pub characteristic: String,
    pub target_value: Option<String>,
    pub actual_value: Option<String>,
    pub is_conforming: Option<bool>,
}

// Quality Notifications
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct QualityNotification {
    pub id: Uuid,
    pub notification_number: String,
    pub notification_type: String,
    pub material_id: Option<Uuid>,
    pub description: String,
    pub priority: String,
    pub status: String,
    pub reported_by: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQualityNotification {
    #[validate(length(min = 1, max = 50))]
    pub notification_type: String,
    pub material_id: Option<Uuid>,
    #[validate(length(min = 1))]
    pub description: String,
    pub priority: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateQualityNotification {
    #[validate(length(min = 1))]
    pub status: Option<String>,
    #[validate(length(min = 1))]
    pub priority: Option<String>,
    pub assigned_to: Option<Uuid>,
}

/// Request body for completing an inspection lot.
/// `passed` determines whether the inspected goods are accepted or rejected.
#[derive(Debug, Deserialize, Validate)]
pub struct CompleteInspectionLot {
    /// true = quality passed (release hold), false = quality failed (keep hold, create notification)
    pub passed: bool,
    /// Optional notes explaining the decision
    pub notes: Option<String>,
}
