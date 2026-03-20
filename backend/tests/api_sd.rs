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
// SD - Sales Order Item Sub-table CRUD
// ---------------------------------------------------------------------------

/// Helper: create a customer, material, and SO with one item. Returns (so_id, material_id).
async fn create_so_with_item(server: &axum_test::TestServer, token: &str) -> (String, String) {
    let cust_resp = server
        .post("/api/v1/sd/customers")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "name": "SO Item Test Customer",
            "credit_limit": "100000.00"
        }))
        .await;
    let customer_id = cust_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "name": "SO Item Test Material",
            "material_type": "FINISHED"
        }))
        .await;
    let material_id = mat_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let so_resp = server
        .post("/api/v1/sd/sales-orders")
        .add_header(auth(token).0, auth(token).1)
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
    so_resp.assert_status_ok();
    let so_id = so_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    (so_id, material_id)
}

#[tokio::test]
async fn sd_add_so_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (so_id, material_id) = create_so_with_item(&server, &token).await;

    let resp = server
        .post(&format!("/api/v1/sd/sales-orders/{}/items", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material_id,
            "quantity": "10",
            "unit_price": "150.00"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn sd_update_so_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (so_id, _material_id) = create_so_with_item(&server, &token).await;

    // Get SO detail to find item_id
    let detail_resp = server
        .get(&format!("/api/v1/sd/sales-orders/{}", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    detail_resp.assert_status_ok();
    let detail: serde_json::Value = detail_resp.json();
    let item_id = detail["data"]["items"][0]["id"].as_str().unwrap();

    let resp = server
        .put(&format!(
            "/api/v1/sd/sales-orders/{}/items/{}",
            so_id, item_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "quantity": "15",
            "unit_price": "180.00"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn sd_delete_so_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (so_id, material_id) = create_so_with_item(&server, &token).await;

    // Add a second item so we can delete one
    let add_resp = server
        .post(&format!("/api/v1/sd/sales-orders/{}/items", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material_id,
            "quantity": "2",
            "unit_price": "100.00"
        }))
        .await;
    add_resp.assert_status_ok();
    let added_item_id = add_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let resp = server
        .delete(&format!(
            "/api/v1/sd/sales-orders/{}/items/{}",
            so_id, added_item_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

/// Helper to create SO with a stocked material (needed for confirm tests)
async fn create_so_with_stocked_item(
    server: &axum_test::TestServer,
    token: &str,
) -> (String, String) {
    let cust_resp = server
        .post("/api/v1/sd/customers")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "name": "Confirm Test Customer",
            "credit_limit": "9999999.00"
        }))
        .await;
    let customer_id = cust_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    // Use existing material MAT-00001 which has 15000 stock from seed data
    let material_id = "b2000000-0000-0000-0000-000000000001".to_string();

    let so_resp = server
        .post("/api/v1/sd/sales-orders")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "customer_id": customer_id,
            "order_date": "2026-02-01",
            "items": [{
                "material_id": material_id,
                "quantity": "2",
                "unit_price": "35.00"
            }]
        }))
        .await;
    so_resp.assert_status_ok();
    let so_id = so_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    (so_id, material_id)
}

#[tokio::test]
async fn sd_update_so_item_confirmed_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (so_id, _material_id) = create_so_with_stocked_item(&server, &token).await;

    // Get item_id
    let detail_resp = server
        .get(&format!("/api/v1/sd/sales-orders/{}", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    let detail: serde_json::Value = detail_resp.json();
    let item_id = detail["data"]["items"][0]["id"].as_str().unwrap();

    // Confirm the SO (requires stock)
    let confirm_resp = server
        .post(&format!("/api/v1/sd/sales-orders/{}/confirm", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    confirm_resp.assert_status_ok();

    // Now try to update item -- should be rejected
    let resp = server
        .put(&format!(
            "/api/v1/sd/sales-orders/{}/items/{}",
            so_id, item_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "quantity": "999"
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// SD - Delivery with Items
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_create_delivery_with_items() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (so_id, _material_id) = create_so_with_stocked_item(&server, &token).await;

    // Confirm the SO first (delivery requires CONFIRMED status, needs stock)
    let confirm_resp = server
        .post(&format!("/api/v1/sd/sales-orders/{}/confirm", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    confirm_resp.assert_status_ok();

    // Get SO detail to find SO item_id
    let detail_resp = server
        .get(&format!("/api/v1/sd/sales-orders/{}", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    let detail: serde_json::Value = detail_resp.json();
    let so_item_id = detail["data"]["items"][0]["id"].as_str().unwrap();

    // Create delivery
    let resp = server
        .post("/api/v1/sd/deliveries")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "sales_order_id": so_id,
            "delivery_date": "2026-02-15",
            "items": [{
                "sales_order_item_id": so_item_id,
                "quantity": "2"
            }]
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn sd_get_delivery_with_items() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (so_id, _material_id) = create_so_with_stocked_item(&server, &token).await;

    // Confirm SO (needs stock)
    server
        .post(&format!("/api/v1/sd/sales-orders/{}/confirm", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await
        .assert_status_ok();

    // Get SO item id
    let detail_resp = server
        .get(&format!("/api/v1/sd/sales-orders/{}", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    let detail: serde_json::Value = detail_resp.json();
    let so_item_id = detail["data"]["items"][0]["id"].as_str().unwrap();

    // Create delivery
    let del_resp = server
        .post("/api/v1/sd/deliveries")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "sales_order_id": so_id,
            "delivery_date": "2026-02-15",
            "items": [{
                "sales_order_item_id": so_item_id,
                "quantity": "2"
            }]
        }))
        .await;
    del_resp.assert_status_ok();
    let del_id = del_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    // Get delivery detail -- should include items
    let resp = server
        .get(&format!("/api/v1/sd/deliveries/{}", del_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["items"].is_array());
    assert!(!body["data"]["items"].as_array().unwrap().is_empty());
}

// ---------------------------------------------------------------------------
// SD - Get Invoice by ID
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_get_invoice() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // List invoices to find one from seed data
    let list_resp = server
        .get("/api/v1/sd/invoices")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    list_resp.assert_status_ok();
    let list_body: serde_json::Value = list_resp.json();
    let empty = vec![];
    let items = list_body["data"]["items"]
        .as_array()
        .or_else(|| list_body["data"].as_array())
        .unwrap_or(&empty);
    if items.is_empty() {
        // No invoices in seed data, skip
        return;
    }
    let invoice_id = items[0]["id"].as_str().unwrap();

    // Get invoice by ID
    let resp = server
        .get(&format!("/api/v1/sd/invoices/{}", invoice_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["id"].as_str().unwrap(), invoice_id);
}

// ---------------------------------------------------------------------------
// SD - Cancel Sales Order
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_cancel_sales_order() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (so_id, _material_id) = create_so_with_item(&server, &token).await;

    // Cancel the SO (should work on DRAFT status)
    let resp = server
        .post(&format!("/api/v1/sd/sales-orders/{}/cancel", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());

    // Verify the SO is now CANCELLED
    let get_resp = server
        .get(&format!("/api/v1/sd/sales-orders/{}", so_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    get_resp.assert_status_ok();
    let get_body: serde_json::Value = get_resp.json();
    assert_eq!(get_body["data"]["status"].as_str().unwrap(), "CANCELLED");
}

// ---------------------------------------------------------------------------
// SD - Customer Sales Orders
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_customer_sales_orders() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create a customer
    let cust_resp = server
        .post("/api/v1/sd/customers")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Customer SO Query Test",
            "credit_limit": "100000.00"
        }))
        .await;
    cust_resp.assert_status_ok();
    let customer_id = cust_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    // Create a material
    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Customer SO Query Material",
            "material_type": "FINISHED"
        }))
        .await;
    let material_id = mat_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    // Create an SO for this customer
    let so_resp = server
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
    so_resp.assert_status_ok();

    // Query customer's sales orders
    let resp = server
        .get(&format!(
            "/api/v1/sd/customers/{}/sales-orders",
            customer_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// SD - Sales Order Document Flow
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sd_sales_order_document_flow() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (so_id, _material_id) = create_so_with_item(&server, &token).await;

    // Get document flow for the SO
    let resp = server
        .get(&format!(
            "/api/v1/sd/sales-orders/{}/document-flow",
            so_id
        ))
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
