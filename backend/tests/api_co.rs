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
// Unauthenticated access must be rejected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn co_cost_centers_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/co/cost-centers").await;
    resp.assert_status_unauthorized();
}
