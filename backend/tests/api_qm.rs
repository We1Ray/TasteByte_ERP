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
// QM - Inspection Lots
// ---------------------------------------------------------------------------

#[tokio::test]
async fn qm_list_inspection_lots() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/qm/inspection-lots")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn qm_create_inspection_lot() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create a material first
    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "QM Test Material",
            "material_type": "RAW"
        }))
        .await;
    let mat_body: serde_json::Value = mat_resp.json();
    let material_id = mat_body["data"]["id"].as_str().unwrap();

    let resp = server
        .post("/api/v1/qm/inspection-lots")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material_id,
            "inspection_type": "INCOMING",
            "planned_quantity": "50"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn qm_get_inspection_lot() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create material
    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "QM Get Lot Material",
            "material_type": "RAW"
        }))
        .await;
    let mat_body: serde_json::Value = mat_resp.json();
    let material_id = mat_body["data"]["id"].as_str().unwrap();

    // Create inspection lot
    let create_resp = server
        .post("/api/v1/qm/inspection-lots")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material_id,
            "planned_quantity": "25"
        }))
        .await;
    let create_body: serde_json::Value = create_resp.json();
    let lot_id = create_body["data"]["id"].as_str().unwrap();

    // Get it
    let resp = server
        .get(&format!("/api/v1/qm/inspection-lots/{}", lot_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert_eq!(body["data"]["id"].as_str().unwrap(), lot_id);
}

#[tokio::test]
async fn qm_get_inspection_lot_not_found() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/qm/inspection-lots/00000000-0000-0000-0000-000000000099")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_not_found();
}

// ---------------------------------------------------------------------------
// QM - Inspection Results
// ---------------------------------------------------------------------------

#[tokio::test]
async fn qm_create_inspection_result() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create material + inspection lot
    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "QM Result Test Material",
            "material_type": "RAW"
        }))
        .await;
    let mat_body: serde_json::Value = mat_resp.json();
    let material_id = mat_body["data"]["id"].as_str().unwrap();

    let lot_resp = server
        .post("/api/v1/qm/inspection-lots")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material_id,
            "planned_quantity": "10"
        }))
        .await;
    let lot_body: serde_json::Value = lot_resp.json();
    let lot_id = lot_body["data"]["id"].as_str().unwrap();

    let resp = server
        .post("/api/v1/qm/inspection-results")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "inspection_lot_id": lot_id,
            "characteristic": "Weight",
            "target_value": "100g",
            "actual_value": "99.5g",
            "is_conforming": true
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn qm_create_inspection_result_empty_characteristic_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/qm/inspection-results")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "inspection_lot_id": "00000000-0000-0000-0000-000000000001",
            "characteristic": ""
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// QM - Quality Notifications
// ---------------------------------------------------------------------------

#[tokio::test]
async fn qm_list_quality_notifications() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/qm/notifications")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn qm_create_quality_notification() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/qm/notifications")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "notification_type": "COMPLAINT",
            "description": "Integration test quality notification",
            "priority": "MEDIUM"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn qm_create_quality_notification_empty_description_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/qm/notifications")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "notification_type": "COMPLAINT",
            "description": ""
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// QM - Reports
// ---------------------------------------------------------------------------

#[tokio::test]
async fn qm_report_inspection_pass_rate() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/qm/reports/inspection-pass-rate")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn qm_report_notification_summary() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/qm/reports/notification-summary")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

// ---------------------------------------------------------------------------
// QM - Inspection Result Update/Delete
// ---------------------------------------------------------------------------

/// Helper: create a material, inspection lot, and inspection result. Returns (result_id, lot_id).
async fn create_inspection_result_helper(
    server: &axum_test::TestServer,
    token: &str,
) -> (String, String) {
    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "name": "QM Result CRUD Material",
            "material_type": "RAW"
        }))
        .await;
    let material_id = mat_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let lot_resp = server
        .post("/api/v1/qm/inspection-lots")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "material_id": material_id,
            "planned_quantity": "20"
        }))
        .await;
    lot_resp.assert_status_ok();
    let lot_id = lot_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let result_resp = server
        .post("/api/v1/qm/inspection-results")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "inspection_lot_id": lot_id,
            "characteristic": "Temperature",
            "target_value": "25C",
            "actual_value": "24.5C",
            "is_conforming": true
        }))
        .await;
    result_resp.assert_status_ok();
    let result_id = result_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    (result_id, lot_id)
}

#[tokio::test]
async fn qm_update_inspection_result() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (result_id, _lot_id) = create_inspection_result_helper(&server, &token).await;

    let resp = server
        .put(&format!("/api/v1/qm/inspection-results/{}", result_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "actual_value": "26.0C",
            "is_conforming": false
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn qm_delete_inspection_result() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (result_id, _lot_id) = create_inspection_result_helper(&server, &token).await;

    let resp = server
        .delete(&format!("/api/v1/qm/inspection-results/{}", result_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// Unauthenticated access must be rejected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn qm_inspection_lots_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/qm/inspection-lots").await;
    resp.assert_status_unauthorized();
}
