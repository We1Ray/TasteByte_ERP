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
// SD - Customers
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_list_customers() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/sd/customers")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn sd_create_customer() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/sd/customers")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Integration Test Customer",
            "contact_person": "John Smith",
            "email": "customer@test.com",
            "payment_terms": 30,
            "credit_limit": "10000.00"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn sd_create_customer_empty_name_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/sd/customers")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": ""
        }))
        .await;
    resp.assert_status_bad_request();
}

#[tokio::test]
async fn sd_get_customer_not_found() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/sd/customers/00000000-0000-0000-0000-000000000099")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_not_found();
}

// ---------------------------------------------------------------------------
// SD - Sales Orders
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_list_sales_orders() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/sd/sales-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["items"].is_array());
}

#[tokio::test]
async fn sd_create_sales_order() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create customer and material
    let cust_resp = server
        .post("/api/v1/sd/customers")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "SO Test Customer",
            "credit_limit": "100000.00"
        }))
        .await;
    let cust_body: serde_json::Value = cust_resp.json();
    let customer_id = cust_body["data"]["id"].as_str().unwrap();

    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "SO Test Material",
            "material_type": "FINISHED"
        }))
        .await;
    let mat_body: serde_json::Value = mat_resp.json();
    let material_id = mat_body["data"]["id"].as_str().unwrap();

    let resp = server
        .post("/api/v1/sd/sales-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "customer_id": customer_id,
            "order_date": "2026-02-01",
            "items": [{
                "material_id": material_id,
                "quantity": "5",
                "unit_price": "200.00"
            }]
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn sd_create_sales_order_empty_items_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/sd/sales-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "customer_id": "00000000-0000-0000-0000-000000000001",
            "order_date": "2026-02-01",
            "items": []
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// SD - Deliveries
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_list_deliveries() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/sd/deliveries")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// SD - Invoices
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_list_invoices() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/sd/invoices")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// SD - Reports
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_report_sales_summary() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/sd/reports/sales-summary")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn sd_report_top_customers() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/sd/reports/top-customers")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

// ---------------------------------------------------------------------------
// Unauthenticated access must be rejected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_customers_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/sd/customers").await;
    resp.assert_status_unauthorized();
}

#[tokio::test]
async fn sd_sales_orders_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/sd/sales-orders").await;
    resp.assert_status_unauthorized();
}
