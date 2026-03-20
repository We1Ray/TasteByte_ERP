use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Company Codes
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CompanyCode {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub currency: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCompanyCode {
    #[validate(length(min = 1, max = 10))]
    pub code: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub currency: Option<String>,
    pub country: Option<String>,
}

// Fiscal Years
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FiscalYear {
    pub id: Uuid,
    pub company_code_id: Uuid,
    pub year: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_closed: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFiscalYear {
    pub company_code_id: Uuid,
    pub year: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

// Fiscal Periods
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FiscalPeriod {
    pub id: Uuid,
    pub fiscal_year_id: Uuid,
    pub period: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_closed: bool,
}

// Account Groups
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AccountGroup {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAccountGroup {
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub account_type: String,
}

// Accounts
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub account_number: String,
    pub name: String,
    pub account_group_id: Option<Uuid>,
    pub account_type: String,
    pub is_reconciliation: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAccount {
    #[validate(length(min = 1, max = 20))]
    pub account_number: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub account_group_id: Option<Uuid>,
    pub account_type: String,
    pub is_reconciliation: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAccount {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub account_group_id: Option<Uuid>,
    pub is_active: Option<bool>,
}

// Journal Entries
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct JournalEntry {
    pub id: Uuid,
    pub document_number: String,
    pub company_code_id: Uuid,
    pub fiscal_year: i32,
    pub fiscal_period: i32,
    pub posting_date: NaiveDate,
    pub document_date: NaiveDate,
    pub reference: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateJournalEntry {
    pub company_code_id: Uuid,
    pub posting_date: NaiveDate,
    pub document_date: NaiveDate,
    pub reference: Option<String>,
    pub description: Option<String>,
    #[validate(length(min = 1, message = "At least one line item is required"))]
    pub items: Vec<CreateJournalItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateJournalItem {
    pub account_id: Uuid,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub cost_center_id: Option<Uuid>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct JournalItem {
    pub id: Uuid,
    pub journal_entry_id: Uuid,
    pub line_number: i32,
    pub account_id: Uuid,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub cost_center_id: Option<Uuid>,
    pub description: Option<String>,
}

// AR/AP Invoices
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ArInvoice {
    pub id: Uuid,
    pub document_number: String,
    pub customer_id: Option<Uuid>,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_amount: Decimal,
    pub paid_amount: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateArInvoice {
    pub customer_id: Option<Uuid>,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ApInvoice {
    pub id: Uuid,
    pub document_number: String,
    pub vendor_id: Option<Uuid>,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_amount: Decimal,
    pub paid_amount: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateApInvoice {
    pub vendor_id: Option<Uuid>,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_amount: Decimal,
}

// Payment Documents
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PaymentDocument {
    pub id: Uuid,
    pub document_number: String,
    pub payment_type: String,
    pub invoice_id: Uuid,
    pub amount: Decimal,
    pub payment_date: NaiveDate,
    pub journal_entry_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecordPaymentInput {
    pub amount: Decimal,
    pub payment_date: NaiveDate,
}

// --- Journal Item CRUD (sub-table) ---
#[derive(Debug, Deserialize, Validate)]
pub struct AddJournalItem {
    pub account_id: Uuid,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub cost_center_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateJournalItem {
    pub account_id: Option<Uuid>,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub description: Option<String>,
    pub cost_center_id: Option<Uuid>,
}
