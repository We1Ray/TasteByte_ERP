use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::hr::models::*;
use crate::shared::pagination::ListParams;
use crate::shared::AppError;

// --- Departments ---
pub async fn list_departments(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<Department>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, Department>(
        r#"SELECT * FROM hr_departments
           WHERE is_active = true
             AND ($1 = false OR (code ILIKE $2 OR name ILIKE $2))
           ORDER BY code
           LIMIT $3 OFFSET $4"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_departments(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM hr_departments
           WHERE is_active = true
             AND ($1 = false OR (code ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_department(pool: &PgPool, id: Uuid) -> Result<Department, AppError> {
    sqlx::query_as::<_, Department>("SELECT * FROM hr_departments WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Department not found".to_string()))
}

pub async fn create_department(
    pool: &PgPool,
    input: &CreateDepartment,
) -> Result<Department, AppError> {
    let row = sqlx::query_as::<_, Department>(
        "INSERT INTO hr_departments (code, name, parent_department_id, manager_id, cost_center_id) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(&input.code).bind(&input.name).bind(input.parent_department_id).bind(input.manager_id).bind(input.cost_center_id)
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn update_department(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateDepartment,
) -> Result<Department, AppError> {
    let row = sqlx::query_as::<_, Department>(
        "UPDATE hr_departments SET name = COALESCE($2, name), parent_department_id = COALESCE($3, parent_department_id), manager_id = COALESCE($4, manager_id), cost_center_id = COALESCE($5, cost_center_id), is_active = COALESCE($6, is_active) WHERE id = $1 RETURNING *"
    )
    .bind(id).bind(&input.name).bind(input.parent_department_id).bind(input.manager_id).bind(input.cost_center_id).bind(input.is_active)
    .fetch_optional(pool).await?
    .ok_or_else(|| AppError::NotFound("Department not found".to_string()))?;
    Ok(row)
}

pub async fn get_department_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
) -> Result<Department, AppError> {
    sqlx::query_as::<_, Department>("SELECT * FROM hr_departments WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| AppError::NotFound("Department not found".to_string()))
}

// --- Positions ---
pub async fn list_positions(pool: &PgPool, params: &ListParams) -> Result<Vec<Position>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, Position>(
        r#"SELECT * FROM hr_positions
           WHERE is_active = true
             AND ($1 = false OR (code ILIKE $2 OR title ILIKE $2))
           ORDER BY code
           LIMIT $3 OFFSET $4"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_positions(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM hr_positions
           WHERE is_active = true
             AND ($1 = false OR (code ILIKE $2 OR title ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_position(pool: &PgPool, input: &CreatePosition) -> Result<Position, AppError> {
    let row = sqlx::query_as::<_, Position>(
        "INSERT INTO hr_positions (code, title, department_id, grade) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&input.code).bind(&input.title).bind(input.department_id).bind(&input.grade)
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Employees ---
pub async fn list_employees(pool: &PgPool, params: &ListParams) -> Result<Vec<Employee>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, Employee>(
        r#"SELECT e.* FROM hr_employees e
           WHERE ($1 = false OR (e.employee_number ILIKE $2 OR e.first_name ILIKE $2 OR e.last_name ILIKE $2 OR e.email ILIKE $2))
             AND ($3 = false OR e.status = $4)
             AND ($5 = false OR e.department_id::text = $6)
           ORDER BY e.employee_number
           LIMIT $7 OFFSET $8"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_filter_type)
    .bind(&filter_type)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_employees(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM hr_employees e
           WHERE ($1 = false OR (e.employee_number ILIKE $2 OR e.first_name ILIKE $2 OR e.last_name ILIKE $2 OR e.email ILIKE $2))
             AND ($3 = false OR e.status = $4)
             AND ($5 = false OR e.department_id::text = $6)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_filter_type)
    .bind(&filter_type)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_employee(pool: &PgPool, id: Uuid) -> Result<Employee, AppError> {
    sqlx::query_as::<_, Employee>("SELECT * FROM hr_employees WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Employee not found".to_string()))
}

pub async fn create_employee(
    pool: &PgPool,
    emp_number: &str,
    input: &CreateEmployee,
) -> Result<Employee, AppError> {
    let row = sqlx::query_as::<_, Employee>(
        "INSERT INTO hr_employees (employee_number, user_id, first_name, last_name, email, phone, department_id, position_id, hire_date) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(emp_number).bind(input.user_id).bind(&input.first_name).bind(&input.last_name)
    .bind(&input.email).bind(&input.phone).bind(input.department_id).bind(input.position_id).bind(input.hire_date)
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn update_employee(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateEmployee,
) -> Result<Employee, AppError> {
    let row = sqlx::query_as::<_, Employee>(
        "UPDATE hr_employees SET first_name = COALESCE($2, first_name), last_name = COALESCE($3, last_name), email = COALESCE($4, email), phone = COALESCE($5, phone), department_id = COALESCE($6, department_id), position_id = COALESCE($7, position_id), status = COALESCE($8, status), termination_date = COALESCE($9, termination_date), updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id).bind(&input.first_name).bind(&input.last_name).bind(&input.email).bind(&input.phone)
    .bind(input.department_id).bind(input.position_id).bind(&input.status).bind(input.termination_date)
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Attendance ---
pub async fn list_attendance(
    pool: &PgPool,
    employee_id: Uuid,
    params: &ListParams,
) -> Result<Vec<Attendance>, AppError> {
    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let rows = sqlx::query_as::<_, Attendance>(
        r#"SELECT * FROM hr_attendance
           WHERE employee_id = $1
             AND ($2 = false OR status = $3)
             AND ($4 = false OR date >= $5)
             AND ($6 = false OR date <= $7)
           ORDER BY date DESC
           LIMIT $8 OFFSET $9"#,
    )
    .bind(employee_id)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_attendance(
    pool: &PgPool,
    employee_id: Uuid,
    params: &ListParams,
) -> Result<i64, AppError> {
    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM hr_attendance
           WHERE employee_id = $1
             AND ($2 = false OR status = $3)
             AND ($4 = false OR date >= $5)
             AND ($6 = false OR date <= $7)"#,
    )
    .bind(employee_id)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_attendance(
    pool: &PgPool,
    input: &CreateAttendance,
) -> Result<Attendance, AppError> {
    let row = sqlx::query_as::<_, Attendance>(
        "INSERT INTO hr_attendance (employee_id, date, clock_in, clock_out, status, notes) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(input.employee_id).bind(input.date).bind(input.clock_in).bind(input.clock_out)
    .bind(input.status.as_deref().unwrap_or("PRESENT")).bind(&input.notes)
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn next_number(pool: &PgPool, object_type: &str) -> Result<String, AppError> {
    crate::shared::number_range::next_number(pool, object_type).await
}

// --- Salary Structures ---
pub async fn list_salary_structures(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<SalaryStructure>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, SalaryStructure>(
        r#"SELECT * FROM hr_salary_structures
           WHERE ($1 = false OR (structure_number ILIKE $2))
           ORDER BY created_at DESC
           LIMIT $3 OFFSET $4"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_salary_structures(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM hr_salary_structures
           WHERE ($1 = false OR (structure_number ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_salary_structure(pool: &PgPool, id: Uuid) -> Result<SalaryStructure, AppError> {
    sqlx::query_as::<_, SalaryStructure>("SELECT * FROM hr_salary_structures WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Salary structure not found".to_string()))
}

pub async fn create_salary_structure(
    pool: &PgPool,
    structure_number: &str,
    input: &CreateSalaryStructure,
) -> Result<SalaryStructure, AppError> {
    let row = sqlx::query_as::<_, SalaryStructure>(
        "INSERT INTO hr_salary_structures (structure_number, employee_id, base_salary, allowances, deductions, effective_from, effective_to) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(structure_number)
    .bind(input.employee_id)
    .bind(input.base_salary)
    .bind(input.allowances)
    .bind(input.deductions)
    .bind(input.effective_from)
    .bind(input.effective_to)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn update_salary_structure(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateSalaryStructure,
) -> Result<SalaryStructure, AppError> {
    let row = sqlx::query_as::<_, SalaryStructure>(
        "UPDATE hr_salary_structures SET base_salary = COALESCE($2, base_salary), allowances = COALESCE($3, allowances), deductions = COALESCE($4, deductions), effective_from = COALESCE($5, effective_from), effective_to = COALESCE($6, effective_to), is_active = COALESCE($7, is_active), updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(input.base_salary)
    .bind(input.allowances)
    .bind(input.deductions)
    .bind(input.effective_from)
    .bind(input.effective_to)
    .bind(input.is_active)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Salary structure not found".to_string()))?;
    Ok(row)
}

// --- Payroll Runs ---
pub async fn list_payroll_runs(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<PayrollRun>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, PayrollRun>(
        r#"SELECT * FROM hr_payroll_runs
           WHERE ($1 = false OR (run_number ILIKE $2))
             AND ($3 = false OR status = $4)
           ORDER BY created_at DESC
           LIMIT $5 OFFSET $6"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_payroll_runs(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM hr_payroll_runs
           WHERE ($1 = false OR (run_number ILIKE $2))
             AND ($3 = false OR status = $4)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_payroll_run(pool: &PgPool, id: Uuid) -> Result<PayrollRun, AppError> {
    sqlx::query_as::<_, PayrollRun>("SELECT * FROM hr_payroll_runs WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Payroll run not found".to_string()))
}

pub async fn get_payroll_run_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
) -> Result<PayrollRun, AppError> {
    sqlx::query_as::<_, PayrollRun>("SELECT * FROM hr_payroll_runs WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| AppError::NotFound("Payroll run not found".to_string()))
}

pub async fn create_payroll_run(
    pool: &PgPool,
    run_number: &str,
    input: &CreatePayrollRun,
    user_id: Uuid,
) -> Result<PayrollRun, AppError> {
    let row = sqlx::query_as::<_, PayrollRun>(
        "INSERT INTO hr_payroll_runs (run_number, period_year, period_month, created_by) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(run_number)
    .bind(input.period_year)
    .bind(input.period_month)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

#[allow(clippy::too_many_arguments)]
pub async fn create_payroll_item_on_conn(
    conn: &mut sqlx::PgConnection,
    payroll_run_id: Uuid,
    employee_id: Uuid,
    department_id: Option<Uuid>,
    cost_center_id: Option<Uuid>,
    base_salary: Decimal,
    allowances: Decimal,
    deductions: Decimal,
    net_salary: Decimal,
) -> Result<PayrollItem, AppError> {
    let row = sqlx::query_as::<_, PayrollItem>(
        "INSERT INTO hr_payroll_items (payroll_run_id, employee_id, department_id, cost_center_id, base_salary, allowances, deductions, net_salary) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
    )
    .bind(payroll_run_id)
    .bind(employee_id)
    .bind(department_id)
    .bind(cost_center_id)
    .bind(base_salary)
    .bind(allowances)
    .bind(deductions)
    .bind(net_salary)
    .fetch_one(&mut *conn)
    .await?;
    Ok(row)
}

#[allow(clippy::too_many_arguments)]
pub async fn update_payroll_run_completed_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
    total_gross: Decimal,
    total_deductions: Decimal,
    total_net: Decimal,
    employee_count: i32,
    journal_entry_id: Option<Uuid>,
    user_id: Uuid,
) -> Result<PayrollRun, AppError> {
    let row = sqlx::query_as::<_, PayrollRun>(
        "UPDATE hr_payroll_runs SET status = 'COMPLETED', total_gross = $2, total_deductions = $3, total_net = $4, employee_count = $5, journal_entry_id = $6, executed_by = $7, executed_at = NOW(), updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(total_gross)
    .bind(total_deductions)
    .bind(total_net)
    .bind(employee_count)
    .bind(journal_entry_id)
    .bind(user_id)
    .fetch_one(&mut *conn)
    .await?;
    Ok(row)
}

pub async fn get_payroll_items(
    pool: &PgPool,
    payroll_run_id: Uuid,
) -> Result<Vec<PayrollItem>, AppError> {
    let rows = sqlx::query_as::<_, PayrollItem>(
        "SELECT * FROM hr_payroll_items WHERE payroll_run_id = $1 ORDER BY created_at",
    )
    .bind(payroll_run_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Returns active salary structures joined with employee department info.
/// Each tuple: (SalaryStructure, employee_id, department_id, department.cost_center_id)
#[allow(clippy::type_complexity)]
pub async fn get_active_salary_structures_with_employees(
    conn: &mut sqlx::PgConnection,
) -> Result<Vec<(SalaryStructure, Uuid, Option<Uuid>, Option<Uuid>)>, AppError> {
    let rows: Vec<(
        Uuid,
        String,
        Uuid,
        Decimal,
        Decimal,
        Decimal,
        String,
        NaiveDate,
        Option<NaiveDate>,
        bool,
        chrono::DateTime<chrono::Utc>,
        chrono::DateTime<chrono::Utc>,
        Uuid,
        Option<Uuid>,
        Option<Uuid>,
    )> = sqlx::query_as(
        r#"SELECT
            ss.id, ss.structure_number, ss.employee_id, ss.base_salary,
            ss.allowances, ss.deductions, ss.currency, ss.effective_from,
            ss.effective_to, ss.is_active, ss.created_at, ss.updated_at,
            e.id AS emp_id, e.department_id, d.cost_center_id
           FROM hr_salary_structures ss
           JOIN hr_employees e ON e.id = ss.employee_id
           LEFT JOIN hr_departments d ON d.id = e.department_id
           WHERE ss.is_active = true AND e.status = 'ACTIVE'
           ORDER BY e.employee_number"#,
    )
    .fetch_all(&mut *conn)
    .await?;

    let result = rows
        .into_iter()
        .map(|r| {
            let ss = SalaryStructure {
                id: r.0,
                structure_number: r.1,
                employee_id: r.2,
                base_salary: r.3,
                allowances: r.4,
                deductions: r.5,
                currency: r.6,
                effective_from: r.7,
                effective_to: r.8,
                is_active: r.9,
                created_at: r.10,
                updated_at: r.11,
            };
            (ss, r.12, r.13, r.14)
        })
        .collect();
    Ok(result)
}
