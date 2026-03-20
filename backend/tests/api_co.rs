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
// CO - Cost Centers
// ---------------------------------------------------------------------------

#[tokio::test]
async fn co_list_cost_centers() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/co/cost-centers")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn co_create_cost_center() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let unique_code = format!("TC{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let resp = server
        .post("/api/v1/co/cost-centers")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": unique_code,
            "name": "Integration Test CC",
            "description": "Created by integration test"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn co_create_cost_center_empty_code_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/co/cost-centers")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": "",
            "name": "Should Fail"
        }))
        .await;
    resp.assert_status_bad_request();
}

#[tokio::test]
async fn co_get_cost_center_not_found() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/co/cost-centers/00000000-0000-0000-0000-000000000099")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_not_found();
}

// ---------------------------------------------------------------------------
// CO - Profit Centers
// ---------------------------------------------------------------------------

#[tokio::test]
async fn co_list_profit_centers() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/co/profit-centers")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn co_create_profit_center() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let unique_code = format!("TP{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let resp = server
        .post("/api/v1/co/profit-centers")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": unique_code,
            "name": "Integration Test PC"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// CO - Internal Orders
// ---------------------------------------------------------------------------

#[tokio::test]
async fn co_list_internal_orders() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/co/internal-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn co_create_internal_order() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/co/internal-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Integration Test Order",
            "order_type": "MARKETING",
            "budget": "5000.00"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

// ---------------------------------------------------------------------------
// CO - Cost Allocations
// ---------------------------------------------------------------------------

#[tokio::test]
async fn co_list_cost_allocations() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/co/cost-allocations")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// CO - Reports
// ---------------------------------------------------------------------------

#[tokio::test]
async fn co_report_cost_center_summary() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/co/reports/cost-center-summary")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn co_report_internal_order_budget() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/co/reports/internal-order-budget")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

// ---------------------------------------------------------------------------
// CO - Cost Allocation Update/Delete
// ---------------------------------------------------------------------------

/// Helper: create two cost centers and a cost allocation. Returns allocation_id.
async fn create_cost_allocation_helper(
    server: &axum_test::TestServer,
    token: &str,
) -> String {
    let from_code = format!("CF{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let from_resp = server
        .post("/api/v1/co/cost-centers")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "code": from_code,
            "name": "Alloc From CC"
        }))
        .await;
    from_resp.assert_status_ok();
    let from_id = from_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let to_code = format!("CT{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let to_resp = server
        .post("/api/v1/co/cost-centers")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "code": to_code,
            "name": "Alloc To CC"
        }))
        .await;
    to_resp.assert_status_ok();
    let to_id = to_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let alloc_resp = server
        .post("/api/v1/co/cost-allocations")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "from_cost_center_id": from_id,
            "to_cost_center_id": to_id,
            "allocation_date": "2026-03-01",
            "amount": "5000.00",
            "description": "Test allocation"
        }))
        .await;
    alloc_resp.assert_status_ok();
    alloc_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn co_update_cost_allocation() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let alloc_id = create_cost_allocation_helper(&server, &token).await;

    let resp = server
        .put(&format!("/api/v1/co/cost-allocations/{}", alloc_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "amount": "7500.00",
            "description": "Updated allocation"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn co_delete_cost_allocation() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let alloc_id = create_cost_allocation_helper(&server, &token).await;

    let resp = server
        .delete(&format!("/api/v1/co/cost-allocations/{}", alloc_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// CO - Get Cost Allocation by ID
// ---------------------------------------------------------------------------

#[tokio::test]
async fn co_get_cost_allocation() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create a cost allocation using the existing helper
    let alloc_id = create_cost_allocation_helper(&server, &token).await;

    // Get cost allocation by ID
    let resp = server
        .get(&format!("/api/v1/co/cost-allocations/{}", alloc_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["id"].as_str().unwrap(), alloc_id);
}

// ---------------------------------------------------------------------------
// Unauthenticated access must be rejected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn co_cost_centers_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/co/cost-centers").await;
    resp.assert_status_unauthorized();
}
