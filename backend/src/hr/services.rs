use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::hr::models::*;
use crate::hr::repositories;
use crate::shared::pagination::ListParams;
use crate::shared::{AppError, PaginatedResponse};

pub async fn list_departments(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Department>, AppError> {
    let total = repositories::count_departments(pool, params).await?;
    let data = repositories::list_departments(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_department(pool: &PgPool, id: Uuid) -> Result<Department, AppError> {
    repositories::get_department(pool, id).await
}

pub async fn create_department(
    pool: &PgPool,
    input: CreateDepartment,
) -> Result<Department, AppError> {
    repositories::create_department(pool, &input).await
}

pub async fn update_department(
    pool: &PgPool,
    id: Uuid,
    input: UpdateDepartment,
) -> Result<Department, AppError> {
    repositories::update_department(pool, id, &input).await
}

pub async fn list_positions(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Position>, AppError> {
    let total = repositories::count_positions(pool, params).await?;
    let data = repositories::list_positions(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn create_position(pool: &PgPool, input: CreatePosition) -> Result<Position, AppError> {
    repositories::create_position(pool, &input).await
}

pub async fn list_employees(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Employee>, AppError> {
    let total = repositories::count_employees(pool, params).await?;
    let data = repositories::list_employees(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_employee(pool: &PgPool, id: Uuid) -> Result<Employee, AppError> {
    repositories::get_employee(pool, id).await
}

pub async fn create_employee(pool: &PgPool, input: CreateEmployee) -> Result<Employee, AppError> {
    let emp_number = repositories::next_number(pool, "EMP").await?;
    repositories::create_employee(pool, &emp_number, &input).await
}

pub async fn update_employee(
    pool: &PgPool,
    id: Uuid,
    input: UpdateEmployee,
) -> Result<Employee, AppError> {
    repositories::update_employee(pool, id, &input).await
}

pub async fn list_attendance(
    pool: &PgPool,
    employee_id: Uuid,
    params: &ListParams,
) -> Result<PaginatedResponse<Attendance>, AppError> {
    let total = repositories::count_attendance(pool, employee_id, params).await?;
    let data = repositories::list_attendance(pool, employee_id, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn create_attendance(
    pool: &PgPool,
    input: CreateAttendance,
) -> Result<Attendance, AppError> {
    repositories::create_attendance(pool, &input).await
}

// --- Salary Structures ---
pub async fn list_salary_structures(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<SalaryStructure>, AppError> {
    let total = repositories::count_salary_structures(pool, params).await?;
    let data = repositories::list_salary_structures(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_salary_structure(pool: &PgPool, id: Uuid) -> Result<SalaryStructure, AppError> {
    repositories::get_salary_structure(pool, id).await
}

pub async fn create_salary_structure(
    pool: &PgPool,
    input: CreateSalaryStructure,
) -> Result<SalaryStructure, AppError> {
    let structure_number = repositories::next_number(pool, "SAL").await?;
    repositories::create_salary_structure(pool, &structure_number, &input).await
}

pub async fn update_salary_structure(
    pool: &PgPool,
    id: Uuid,
    input: UpdateSalaryStructure,
) -> Result<SalaryStructure, AppError> {
    repositories::update_salary_structure(pool, id, &input).await
}

// --- Payroll Runs ---
pub async fn list_payroll_runs(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<PayrollRun>, AppError> {
    let total = repositories::count_payroll_runs(pool, params).await?;
    let data = repositories::list_payroll_runs(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_payroll_run(pool: &PgPool, id: Uuid) -> Result<PayrollRun, AppError> {
    repositories::get_payroll_run(pool, id).await
}

pub async fn get_payroll_items(
    pool: &PgPool,
    payroll_run_id: Uuid,
) -> Result<Vec<PayrollItem>, AppError> {
    repositories::get_payroll_items(pool, payroll_run_id).await
}

pub async fn create_payroll_run(
    pool: &PgPool,
    input: CreatePayrollRun,
    user_id: Uuid,
) -> Result<PayrollRun, AppError> {
    let run_number = repositories::next_number(pool, "PR").await?;
    repositories::create_payroll_run(pool, &run_number, &input, user_id).await
}

/// Execute a payroll run: calculate salaries, create FI journal entry, post CO allocations.
pub async fn execute_payroll_run(
    pool: &PgPool,
    run_id: Uuid,
    user_id: Uuid,
) -> Result<PayrollRun, AppError> {
    let mut tx = pool.begin().await?;
    let run = repositories::get_payroll_run_on_conn(&mut *tx, run_id).await?;

    if run.status != "DRAFT" {
        return Err(AppError::Validation(
            "Payroll run must be in DRAFT status".to_string(),
        ));
    }

    // Get active salary structures with employee/department info
    let structures = repositories::get_active_salary_structures_with_employees(&mut *tx).await?;
    if structures.is_empty() {
        return Err(AppError::Validation(
            "No active salary structures found".to_string(),
        ));
    }

    let mut total_gross = Decimal::ZERO;
    let mut total_deductions = Decimal::ZERO;
    let mut total_net = Decimal::ZERO;
    let mut employee_count = 0i32;
    let mut cc_amounts: std::collections::HashMap<Uuid, Decimal> = std::collections::HashMap::new();

    for (ss, emp_id, dept_id, cc_id) in &structures {
        let gross = ss.base_salary + ss.allowances;
        let net = gross - ss.deductions;

        repositories::create_payroll_item_on_conn(
            &mut *tx,
            run_id,
            *emp_id,
            *dept_id,
            *cc_id,
            ss.base_salary,
            ss.allowances,
            ss.deductions,
            net,
        )
        .await?;

        total_gross += gross;
        total_deductions += ss.deductions;
        total_net += net;
        employee_count += 1;

        if let Some(cc) = cc_id {
            *cc_amounts.entry(*cc).or_default() += net;
        }
    }

    // Create FI Journal Entry: DR Salary Expense(6100) CR Payroll Payable(2210)
    let salary_account =
        sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_accounts WHERE account_number = '6100'")
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::Internal("Salary expense account 6100 not found".to_string()))?
            .0;

    let payroll_payable =
        sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_accounts WHERE account_number = '2210'")
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| {
                AppError::Internal("Payroll payable account 2210 not found".to_string())
            })?
            .0;

    let company_code_id = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_company_codes LIMIT 1")
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::Internal("No company code found".to_string()))?
        .0;

    let today = chrono::Utc::now().date_naive();
    let je_input = crate::fi::models::CreateJournalEntry {
        company_code_id,
        posting_date: today,
        document_date: today,
        reference: Some(format!("PAYROLL:{}", run.run_number)),
        description: Some(format!(
            "Payroll run {} - {}/{}",
            run.run_number, run.period_year, run.period_month
        )),
        items: vec![
            crate::fi::models::CreateJournalItem {
                account_id: salary_account,
                debit_amount: total_gross,
                credit_amount: Decimal::ZERO,
                cost_center_id: None,
                description: Some("Salary expense".to_string()),
            },
            crate::fi::models::CreateJournalItem {
                account_id: payroll_payable,
                debit_amount: Decimal::ZERO,
                credit_amount: total_gross,
                cost_center_id: None,
                description: Some("Payroll payable".to_string()),
            },
        ],
    };
    let je = crate::fi::services::create_journal_entry_in_tx(&mut *tx, je_input, user_id).await?;
    crate::fi::services::post_journal_entry_in_tx(&mut *tx, je.id, user_id).await?;

    let updated = repositories::update_payroll_run_completed_on_conn(
        &mut *tx,
        run_id,
        total_gross,
        total_deductions,
        total_net,
        employee_count,
        Some(je.id),
        user_id,
    )
    .await?;

    tx.commit().await?;

    // CO auto-posting per cost center (best-effort, after commit)
    for (cc_id, amount) in cc_amounts {
        let desc = format!(
            "HR payroll: {} - {}/{}",
            run.run_number, run.period_year, run.period_month
        );
        if let Err(e) = crate::co::services::auto_post_cost_allocation(
            pool, "HR", run_id, cc_id, amount, today, &desc,
        )
        .await
        {
            tracing::warn!("CO auto-post for payroll failed: {e}");
        }
    }

    Ok(updated)
}
