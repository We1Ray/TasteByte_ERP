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
// MM - UOMs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mm_list_uoms() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/uoms")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// MM - Material Groups
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mm_list_material_groups() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/material-groups")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// MM - Materials
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mm_list_materials() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["items"].is_array());
}

#[tokio::test]
async fn mm_create_material() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Integration Test Material",
            "description": "Created by API test",
            "material_type": "RAW"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
    assert_eq!(
        body["data"]["name"].as_str().unwrap(),
        "Integration Test Material"
    );
}

#[tokio::test]
async fn mm_create_material_empty_name_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "",
            "material_type": "RAW"
        }))
        .await;
    resp.assert_status_bad_request();
}

#[tokio::test]
async fn mm_get_material() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create a material first
    let create_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Get Test Material",
            "material_type": "RAW"
        }))
        .await;
    create_resp.assert_status_ok();
    let create_body: serde_json::Value = create_resp.json();
    let material_id = create_body["data"]["id"].as_str().unwrap();

    // Get it back
    let resp = server
        .get(&format!("/api/v1/mm/materials/{}", material_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert_eq!(body["data"]["id"].as_str().unwrap(), material_id);
}

#[tokio::test]
async fn mm_update_material() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create
    let create_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Update Test Material",
            "material_type": "RAW"
        }))
        .await;
    let create_body: serde_json::Value = create_resp.json();
    let material_id = create_body["data"]["id"].as_str().unwrap();

    // Update
    let resp = server
        .put(&format!("/api/v1/mm/materials/{}", material_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Updated Material Name"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn mm_get_material_not_found() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/materials/00000000-0000-0000-0000-000000000099")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_not_found();
}

// ---------------------------------------------------------------------------
// MM - Vendors
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mm_list_vendors() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/vendors")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn mm_create_vendor() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/mm/vendors")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Integration Test Vendor",
            "contact_person": "Jane Doe",
            "email": "vendor@test.com",
            "payment_terms": 30
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

// ---------------------------------------------------------------------------
// MM - Plant Stock
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mm_list_plant_stock() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/plant-stock")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// MM - Purchase Orders
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mm_list_purchase_orders() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/purchase-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn mm_create_purchase_order() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Need a vendor and material first
    let vendor_resp = server
        .post("/api/v1/mm/vendors")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "PO Test Vendor",
            "payment_terms": 30
        }))
        .await;
    let vendor_body: serde_json::Value = vendor_resp.json();
    let vendor_id = vendor_body["data"]["id"].as_str().unwrap();

    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "PO Test Material",
            "material_type": "RAW"
        }))
        .await;
    let mat_body: serde_json::Value = mat_resp.json();
    let material_id = mat_body["data"]["id"].as_str().unwrap();

    let resp = server
        .post("/api/v1/mm/purchase-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "vendor_id": vendor_id,
            "order_date": "2026-01-15",
            "items": [{
                "material_id": material_id,
                "quantity": "10",
                "unit_price": "50.00"
            }]
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn mm_create_purchase_order_empty_items_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/mm/purchase-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "vendor_id": "00000000-0000-0000-0000-000000000001",
            "order_date": "2026-01-15",
            "items": []
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// MM - Material Movements
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mm_list_material_movements() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/material-movements")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// MM - Reports
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mm_report_stock_valuation() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/reports/stock-valuation")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn mm_report_slow_moving() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/mm/reports/slow-moving")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

// ---------------------------------------------------------------------------
// Unauthenticated access must be rejected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mm_materials_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/mm/materials").await;
    resp.assert_status_unauthorized();
}
