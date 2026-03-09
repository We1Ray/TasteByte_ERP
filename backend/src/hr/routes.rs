use axum::{
    routing::{get, post},
    Router,
};

use crate::hr::handlers;
use crate::hr::reports;
use crate::shared::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/departments",
            get(handlers::list_departments).post(handlers::create_department),
        )
        .route(
            "/departments/{id}",
            get(handlers::get_department).put(handlers::update_department),
        )
        .route(
            "/positions",
            get(handlers::list_positions).post(handlers::create_position),
        )
        .route("/employees/export", get(handlers::export_employees))
        .route(
            "/employees",
            get(handlers::list_employees).post(handlers::create_employee),
        )
        .route(
            "/employees/{id}",
            get(handlers::get_employee).put(handlers::update_employee),
        )
        .route(
            "/employees/{employee_id}/attendance",
            get(handlers::list_attendance),
        )
        .route("/attendance", post(handlers::create_attendance))
        .route(
            "/attendance/{employee_id}/check-in",
            post(handlers::check_in),
        )
        .route(
            "/attendance/{employee_id}/check-out",
            post(handlers::check_out),
        )
        // Salary Structures
        .route(
            "/salary-structures",
            get(handlers::list_salary_structures).post(handlers::create_salary_structure),
        )
        .route(
            "/salary-structures/{id}",
            get(handlers::get_salary_structure).put(handlers::update_salary_structure),
        )
        // Payroll Runs
        .route(
            "/payroll-runs",
            get(handlers::list_payroll_runs).post(handlers::create_payroll_run),
        )
        .route("/payroll-runs/{id}", get(handlers::get_payroll_run))
        .route(
            "/payroll-runs/{id}/execute",
            post(handlers::execute_payroll_run),
        )
        .route("/payroll-runs/{id}/items", get(handlers::get_payroll_items))
        // Reports
        .route(
            "/reports/headcount-by-department",
            get(reports::headcount_by_department),
        )
        .route(
            "/reports/attendance-summary",
            get(reports::attendance_summary),
        )
}
