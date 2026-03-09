use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::auth::rbac::{HrRead, RequireRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

// --- Headcount by Department ---
#[derive(Serialize, sqlx::FromRow)]
pub struct HeadcountByDepartmentRow {
    pub department_code: String,
    pub department_name: String,
    pub headcount: i64,
    pub active_count: i64,
}

pub async fn headcount_by_department(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
) -> Result<Json<ApiResponse<Vec<HeadcountByDepartmentRow>>>, AppError> {
    let rows = sqlx::query_as::<_, HeadcountByDepartmentRow>(
        "SELECT d.code AS department_code, d.name AS department_name, \
         COUNT(e.id) AS headcount, \
         COUNT(CASE WHEN e.status = 'ACTIVE' THEN 1 END) AS active_count \
         FROM hr_departments d \
         LEFT JOIN hr_employees e ON e.department_id = d.id \
         WHERE d.is_active = true \
         GROUP BY d.code, d.name \
         ORDER BY headcount DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}

// --- Attendance Summary ---
#[derive(Deserialize)]
pub struct DateRangeQuery {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AttendanceSummaryRow {
    pub employee_number: String,
    pub employee_name: String,
    pub days_present: i64,
    pub days_absent: i64,
    pub days_late: i64,
}

pub async fn attendance_summary(
    State(state): State<AppState>,
    _role: RequireRole<HrRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<Vec<AttendanceSummaryRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, AttendanceSummaryRow>(
        "SELECT e.employee_number, \
         CONCAT(e.first_name, ' ', e.last_name) AS employee_name, \
         COUNT(CASE WHEN a.status = 'PRESENT' THEN 1 END) AS days_present, \
         COUNT(CASE WHEN a.status = 'ABSENT' THEN 1 END) AS days_absent, \
         COUNT(CASE WHEN a.status = 'LATE' THEN 1 END) AS days_late \
         FROM hr_employees e \
         LEFT JOIN hr_attendance a ON a.employee_id = e.id \
           AND a.date BETWEEN $1 AND $2 \
         WHERE e.status = 'ACTIVE' \
         GROUP BY e.employee_number, e.first_name, e.last_name \
         ORDER BY e.employee_number",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}
