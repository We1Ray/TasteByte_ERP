use axum::extract::{Path, Query, State};
use axum::response::Response;
use axum::Json;
use sqlx;
use uuid::Uuid;
use validator::Validate;

use crate::auth::rbac::{HrRead, HrWrite, RequireRole};
use crate::hr::models::*;
use crate::hr::services;
use crate::shared::audit;
use crate::shared::export::csv_response;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, ListParams, PaginatedResponse};

// --- Departments ---
pub async fn list_departments(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Department>>>, AppError> {
    let result = services::list_departments(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_department(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Department>>, AppError> {
    let dept = services::get_department(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(dept)))
}

pub async fn create_department(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Json(input): Json<CreateDepartment>,
) -> Result<Json<ApiResponse<Department>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let dept = services::create_department(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_departments",
        dept.id,
        "CREATE",
        None,
        serde_json::to_value(&dept).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(dept, "Department created")))
}

pub async fn update_department(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateDepartment>,
) -> Result<Json<ApiResponse<Department>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let dept = services::update_department(&state.pool, id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_departments",
        dept.id,
        "UPDATE",
        None,
        serde_json::to_value(&dept).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(dept, "Department updated")))
}

// --- Positions ---
pub async fn list_positions(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Position>>>, AppError> {
    let result = services::list_positions(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_position(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Json(input): Json<CreatePosition>,
) -> Result<Json<ApiResponse<Position>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let position = services::create_position(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_positions",
        position.id,
        "CREATE",
        None,
        serde_json::to_value(&position).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        position,
        "Position created",
    )))
}

// --- Employees ---
pub async fn list_employees(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Employee>>>, AppError> {
    let result = services::list_employees(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_employee(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Employee>>, AppError> {
    let employee = services::get_employee(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(employee)))
}

pub async fn create_employee(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Json(input): Json<CreateEmployee>,
) -> Result<Json<ApiResponse<Employee>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let employee = services::create_employee(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_employees",
        employee.id,
        "CREATE",
        None,
        serde_json::to_value(&employee).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        employee,
        "Employee created",
    )))
}

pub async fn update_employee(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateEmployee>,
) -> Result<Json<ApiResponse<Employee>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let employee = services::update_employee(&state.pool, id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_employees",
        employee.id,
        "UPDATE",
        None,
        serde_json::to_value(&employee).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        employee,
        "Employee updated",
    )))
}

// --- Attendance ---
pub async fn list_attendance(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Path(employee_id): Path<Uuid>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Attendance>>>, AppError> {
    let result = services::list_attendance(&state.pool, employee_id, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_attendance(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Json(input): Json<CreateAttendance>,
) -> Result<Json<ApiResponse<Attendance>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let attendance = services::create_attendance(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_attendance",
        attendance.id,
        "CREATE",
        None,
        serde_json::to_value(&attendance).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        attendance,
        "Attendance recorded",
    )))
}

// --- Check-in / Check-out ---
pub async fn check_in(
    State(state): State<AppState>,
    _role: RequireRole<HrWrite>,
    Path(employee_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Attendance>>, AppError> {
    let today = chrono::Utc::now().date_naive();
    let now = chrono::Utc::now();
    let attendance = sqlx::query_as::<_, Attendance>(
        "INSERT INTO hr_attendance (employee_id, date, clock_in, status) VALUES ($1, $2, $3, 'PRESENT') RETURNING *",
    )
    .bind(employee_id)
    .bind(today)
    .bind(now)
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(ApiResponse::with_message(attendance, "Checked in")))
}

pub async fn check_out(
    State(state): State<AppState>,
    _role: RequireRole<HrWrite>,
    Path(employee_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Attendance>>, AppError> {
    let today = chrono::Utc::now().date_naive();
    let now = chrono::Utc::now();
    let attendance = sqlx::query_as::<_, Attendance>(
        "UPDATE hr_attendance SET clock_out = $1 WHERE employee_id = $2 AND date = $3 RETURNING *",
    )
    .bind(now)
    .bind(employee_id)
    .bind(today)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("No check-in record found for today".to_string()))?;
    Ok(Json(ApiResponse::with_message(attendance, "Checked out")))
}

// --- Export Employees ---
pub async fn export_employees(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
) -> Result<Response, AppError> {
    let employees =
        sqlx::query_as::<_, Employee>("SELECT * FROM hr_employees ORDER BY employee_number")
            .fetch_all(&state.pool)
            .await?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record([
        "Employee Number",
        "First Name",
        "Last Name",
        "Email",
        "Phone",
        "Hire Date",
        "Termination Date",
        "Status",
        "Created At",
    ])
    .map_err(|e| AppError::Internal(e.to_string()))?;

    for emp in &employees {
        let hire = emp.hire_date.to_string();
        let term = emp
            .termination_date
            .map(|d| d.to_string())
            .unwrap_or_default();
        let created = emp.created_at.to_rfc3339();
        wtr.write_record([
            emp.employee_number.as_str(),
            emp.first_name.as_str(),
            emp.last_name.as_str(),
            emp.email.as_deref().unwrap_or(""),
            emp.phone.as_deref().unwrap_or(""),
            hire.as_str(),
            term.as_str(),
            emp.status.as_str(),
            created.as_str(),
        ])
        .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let csv_data = String::from_utf8(
        wtr.into_inner()
            .map_err(|e| AppError::Internal(e.to_string()))?,
    )
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(csv_response(csv_data, "employees-export.csv"))
}

// --- Salary Structures ---
pub async fn list_salary_structures(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<SalaryStructure>>>, AppError> {
    let result = services::list_salary_structures(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_salary_structure(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SalaryStructure>>, AppError> {
    let ss = services::get_salary_structure(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(ss)))
}

pub async fn create_salary_structure(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Json(input): Json<CreateSalaryStructure>,
) -> Result<Json<ApiResponse<SalaryStructure>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let ss = services::create_salary_structure(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_salary_structures",
        ss.id,
        "CREATE",
        None,
        serde_json::to_value(&ss).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        ss,
        "Salary structure created",
    )))
}

pub async fn update_salary_structure(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateSalaryStructure>,
) -> Result<Json<ApiResponse<SalaryStructure>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let ss = services::update_salary_structure(&state.pool, id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_salary_structures",
        ss.id,
        "UPDATE",
        None,
        serde_json::to_value(&ss).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        ss,
        "Salary structure updated",
    )))
}

// --- Payroll Runs ---
pub async fn list_payroll_runs(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<PayrollRun>>>, AppError> {
    let result = services::list_payroll_runs(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_payroll_run(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PayrollRun>>, AppError> {
    let run = services::get_payroll_run(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(run)))
}

pub async fn create_payroll_run(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Json(input): Json<CreatePayrollRun>,
) -> Result<Json<ApiResponse<PayrollRun>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let run = services::create_payroll_run(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_payroll_runs",
        run.id,
        "CREATE",
        None,
        serde_json::to_value(&run).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(run, "Payroll run created")))
}

pub async fn execute_payroll_run(
    State(state): State<AppState>,
    role: RequireRole<HrWrite>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PayrollRun>>, AppError> {
    let run = services::execute_payroll_run(&state.pool, id, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "hr_payroll_runs",
        run.id,
        "EXECUTE",
        None,
        serde_json::to_value(&run).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(run, "Payroll run executed")))
}

pub async fn get_payroll_items(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<PayrollItem>>>, AppError> {
    let items = services::get_payroll_items(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(items)))
}
