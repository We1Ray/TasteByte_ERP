use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Departments
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Department {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub parent_department_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDepartment {
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub parent_department_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateDepartment {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub parent_department_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub is_active: Option<bool>,
}

// Positions
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Position {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub department_id: Option<Uuid>,
    pub grade: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePosition {
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    pub department_id: Option<Uuid>,
    pub grade: Option<String>,
}

// Employees
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub employee_number: String,
    pub user_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub hire_date: NaiveDate,
    pub termination_date: Option<NaiveDate>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEmployee {
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    pub user_id: Option<Uuid>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub hire_date: NaiveDate,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmployee {
    #[validate(length(min = 1, max = 100))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub status: Option<String>,
    pub termination_date: Option<NaiveDate>,
}

// Attendance
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Attendance {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub date: NaiveDate,
    pub clock_in: Option<DateTime<Utc>>,
    pub clock_out: Option<DateTime<Utc>>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAttendance {
    pub employee_id: Uuid,
    pub date: NaiveDate,
    pub clock_in: Option<DateTime<Utc>>,
    pub clock_out: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

// Salary Structures
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SalaryStructure {
    pub id: Uuid,
    pub structure_number: String,
    pub employee_id: Uuid,
    pub base_salary: Decimal,
    pub allowances: Decimal,
    pub deductions: Decimal,
    pub currency: String,
    pub effective_from: NaiveDate,
    pub effective_to: Option<NaiveDate>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSalaryStructure {
    pub employee_id: Uuid,
    pub base_salary: Decimal,
    #[serde(default)]
    pub allowances: Decimal,
    #[serde(default)]
    pub deductions: Decimal,
    pub effective_from: NaiveDate,
    pub effective_to: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSalaryStructure {
    pub base_salary: Option<Decimal>,
    pub allowances: Option<Decimal>,
    pub deductions: Option<Decimal>,
    pub effective_from: Option<NaiveDate>,
    pub effective_to: Option<NaiveDate>,
    pub is_active: Option<bool>,
}

// Payroll Runs
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PayrollRun {
    pub id: Uuid,
    pub run_number: String,
    pub period_year: i32,
    pub period_month: i32,
    pub status: String,
    pub total_gross: Decimal,
    pub total_deductions: Decimal,
    pub total_net: Decimal,
    pub employee_count: i32,
    pub journal_entry_id: Option<Uuid>,
    pub executed_by: Option<Uuid>,
    pub executed_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePayrollRun {
    pub period_year: i32,
    pub period_month: i32,
}

// Payroll Items
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PayrollItem {
    pub id: Uuid,
    pub payroll_run_id: Uuid,
    pub employee_id: Uuid,
    pub department_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub base_salary: Decimal,
    pub allowances: Decimal,
    pub deductions: Decimal,
    pub net_salary: Decimal,
    pub created_at: DateTime<Utc>,
}
