mod common;

use axum_test::http::{HeaderName, HeaderValue};
use serde_json::json;

fn auth(token: &str) -> (HeaderName, HeaderValue) {
    (
        HeaderName::from_static("authorization"),
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    )
}

async fn get_token(server: &axum_test::TestServer) -> String {
    let resp = server
        .post("/api/v1/auth/login")
        .json(&json!({"username":"admin","password":"admin123"}))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    body["data"]["access_token"]
        .as_str()
        .expect("missing access_token")
        .to_string()
}

// ---------------------------------------------------------------------------
// HR - Departments
// ---------------------------------------------------------------------------

#[tokio::test]
async fn hr_list_departments() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/hr/departments")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn hr_create_department() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let unique_code = format!("TD{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let resp = server
        .post("/api/v1/hr/departments")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": unique_code,
            "name": "Integration Test Department"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

// ---------------------------------------------------------------------------
// HR - Positions
// ---------------------------------------------------------------------------

#[tokio::test]
async fn hr_list_positions() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/hr/positions")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn hr_create_position() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let unique_code = format!("PO{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let resp = server
        .post("/api/v1/hr/positions")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": unique_code,
            "title": "Integration Test Position"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// HR - Employees
// ---------------------------------------------------------------------------

#[tokio::test]
async fn hr_list_employees() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/hr/employees")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["items"].is_array());
}

#[tokio::test]
async fn hr_create_employee() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/hr/employees")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "first_name": "Test",
            "last_name": "Employee",
            "email": format!("test.{}@company.com", uuid::Uuid::new_v4().simple()),
            "hire_date": "2026-01-15"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn hr_create_employee_empty_name_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/hr/employees")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "first_name": "",
            "last_name": "Employee",
            "hire_date": "2026-01-15"
        }))
        .await;
    resp.assert_status_bad_request();
}

#[tokio::test]
async fn hr_get_employee_not_found() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/hr/employees/00000000-0000-0000-0000-000000000099")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_not_found();
}

// ---------------------------------------------------------------------------
// HR - Attendance
// ---------------------------------------------------------------------------

#[tokio::test]
async fn hr_create_and_list_attendance() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create employee first
    let emp_resp = server
        .post("/api/v1/hr/employees")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "first_name": "Attendance",
            "last_name": "Test",
            "hire_date": "2026-01-01"
        }))
        .await;
    let emp_body: serde_json::Value = emp_resp.json();
    let employee_id = emp_body["data"]["id"].as_str().unwrap();

    // Create attendance record
    let att_resp = server
        .post("/api/v1/hr/attendance")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "employee_id": employee_id,
            "date": "2026-02-15",
            "status": "PRESENT"
        }))
        .await;
    att_resp.assert_status_ok();

    // List attendance for the employee
    let list_resp = server
        .get(&format!("/api/v1/hr/employees/{}/attendance", employee_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    list_resp.assert_status_ok();
    let body: serde_json::Value = list_resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// HR - Reports
// ---------------------------------------------------------------------------

#[tokio::test]
async fn hr_report_headcount_by_department() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/hr/reports/headcount-by-department")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn hr_report_attendance_summary() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/hr/reports/attendance-summary")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

// ---------------------------------------------------------------------------
// Unauthenticated access must be rejected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn hr_employees_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/hr/employees").await;
    resp.assert_status_unauthorized();
}
