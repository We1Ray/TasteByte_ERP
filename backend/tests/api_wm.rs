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
// WM - Warehouses
// ---------------------------------------------------------------------------

#[tokio::test]
async fn wm_list_warehouses() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/wm/warehouses")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn wm_create_warehouse() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let unique_code = format!("WH{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let resp = server
        .post("/api/v1/wm/warehouses")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": unique_code,
            "name": "Integration Test Warehouse",
            "address": "123 Test Street",
            "warehouse_type": "STANDARD"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn wm_create_warehouse_empty_code_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/wm/warehouses")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": "",
            "name": "Should Fail"
        }))
        .await;
    resp.assert_status_bad_request();
}

#[tokio::test]
async fn wm_get_warehouse_not_found() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/wm/warehouses/00000000-0000-0000-0000-000000000099")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_not_found();
}

// ---------------------------------------------------------------------------
// WM - Storage Bins
// ---------------------------------------------------------------------------

#[tokio::test]
async fn wm_create_and_list_storage_bins() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create warehouse first
    let unique_code = format!("SB{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let wh_resp = server
        .post("/api/v1/wm/warehouses")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": unique_code,
            "name": "Storage Bin Test WH"
        }))
        .await;
    let wh_body: serde_json::Value = wh_resp.json();
    let warehouse_id = wh_body["data"]["id"].as_str().unwrap();

    // Create storage bin
    let bin_resp = server
        .post("/api/v1/wm/storage-bins")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "warehouse_id": warehouse_id,
            "bin_code": format!("BIN{}", &uuid::Uuid::new_v4().simple().to_string()[..4]),
            "zone": "A",
            "aisle": "01",
            "rack": "R1",
            "level": "L1"
        }))
        .await;
    bin_resp.assert_status_ok();

    // List storage bins for the warehouse
    let list_resp = server
        .get(&format!(
            "/api/v1/wm/warehouses/{}/storage-bins",
            warehouse_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    list_resp.assert_status_ok();
    let body: serde_json::Value = list_resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// WM - Stock Transfers
// ---------------------------------------------------------------------------

#[tokio::test]
async fn wm_list_stock_transfers() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/wm/stock-transfers")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// WM - Stock Counts
// ---------------------------------------------------------------------------

#[tokio::test]
async fn wm_list_stock_counts() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/wm/stock-counts")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn wm_create_stock_count() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create warehouse and material
    let unique_code = format!("SC{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let wh_resp = server
        .post("/api/v1/wm/warehouses")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": unique_code,
            "name": "Stock Count Test WH"
        }))
        .await;
    let wh_body: serde_json::Value = wh_resp.json();
    let warehouse_id = wh_body["data"]["id"].as_str().unwrap();

    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Stock Count Test Material",
            "material_type": "RAW"
        }))
        .await;
    let mat_body: serde_json::Value = mat_resp.json();
    let material_id = mat_body["data"]["id"].as_str().unwrap();

    let resp = server
        .post("/api/v1/wm/stock-counts")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "warehouse_id": warehouse_id,
            "count_date": "2026-02-20",
            "items": [{
                "material_id": material_id,
                "book_quantity": "100",
                "counted_quantity": "98"
            }]
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn wm_create_stock_count_empty_items_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/wm/stock-counts")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "warehouse_id": "00000000-0000-0000-0000-000000000001",
            "count_date": "2026-02-20",
            "items": []
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// WM - Reports
// ---------------------------------------------------------------------------

#[tokio::test]
async fn wm_report_warehouse_utilization() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/wm/reports/warehouse-utilization")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn wm_report_transfer_summary() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/wm/reports/transfer-summary")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

// ---------------------------------------------------------------------------
// WM - Stock Count Item Sub-table CRUD
// ---------------------------------------------------------------------------

/// Helper: create a stock count with one item. Returns (sc_id, material_id).
async fn create_stock_count_with_item(
    server: &axum_test::TestServer,
    token: &str,
) -> (String, String) {
    let unique_code = format!("SI{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let wh_resp = server
        .post("/api/v1/wm/warehouses")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "code": unique_code,
            "name": "SC Item Test WH"
        }))
        .await;
    let warehouse_id = wh_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "name": "SC Item Test Material",
            "material_type": "RAW"
        }))
        .await;
    let material_id = mat_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let sc_resp = server
        .post("/api/v1/wm/stock-counts")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "warehouse_id": warehouse_id,
            "count_date": "2026-03-01",
            "items": [{
                "material_id": material_id,
                "book_quantity": "100",
                "counted_quantity": "98"
            }]
        }))
        .await;
    sc_resp.assert_status_ok();
    let sc_id = sc_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    (sc_id, material_id)
}

#[tokio::test]
async fn wm_add_stock_count_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (sc_id, material_id) = create_stock_count_with_item(&server, &token).await;

    // Create another material for the new item
    let mat2_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "SC Item Test Material 2",
            "material_type": "RAW"
        }))
        .await;
    let material2_id = mat2_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let resp = server
        .post(&format!("/api/v1/wm/stock-counts/{}/items", sc_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material2_id,
            "book_quantity": "50",
            "counted_quantity": "49"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());

    // Use material_id to suppress unused variable warning
    let _ = material_id;
}

#[tokio::test]
async fn wm_update_stock_count_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (sc_id, _material_id) = create_stock_count_with_item(&server, &token).await;

    // Get stock count detail to find item_id
    let detail_resp = server
        .get(&format!("/api/v1/wm/stock-counts/{}", sc_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    detail_resp.assert_status_ok();
    let detail: serde_json::Value = detail_resp.json();
    let item_id = detail["data"]["items"][0]["id"].as_str().unwrap();

    let resp = server
        .put(&format!(
            "/api/v1/wm/stock-counts/{}/items/{}",
            sc_id, item_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "counted_quantity": "95",
            "book_quantity": "100"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn wm_delete_stock_count_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (sc_id, _material_id) = create_stock_count_with_item(&server, &token).await;

    // Add a second item so we can delete one
    let mat2_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "SC Delete Test Material",
            "material_type": "RAW"
        }))
        .await;
    let material2_id = mat2_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let add_resp = server
        .post(&format!("/api/v1/wm/stock-counts/{}/items", sc_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material2_id,
            "book_quantity": "30",
            "counted_quantity": "30"
        }))
        .await;
    add_resp.assert_status_ok();
    let added_item_id = add_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let resp = server
        .delete(&format!(
            "/api/v1/wm/stock-counts/{}/items/{}",
            sc_id, added_item_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// WM - Get Stock Transfer by ID
// ---------------------------------------------------------------------------

#[tokio::test]
async fn wm_get_stock_transfer() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Use seed warehouse WH-TPE as source (has stock for seed materials)
    let from_wh_id = "f1000000-0000-0000-0000-000000000001";

    // Create a destination warehouse
    let to_code = format!("TT{}", &uuid::Uuid::new_v4().simple().to_string()[..6]);
    let to_resp = server
        .post("/api/v1/wm/warehouses")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "code": to_code,
            "name": "Transfer To WH"
        }))
        .await;
    to_resp.assert_status_ok();
    let to_wh_id = to_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    // Use seed material with stock at WH-TPE
    let material_id = "b2000000-0000-0000-0000-000000000001";

    // Create a stock transfer from seed warehouse
    let create_resp = server
        .post("/api/v1/wm/stock-transfers")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "from_warehouse_id": from_wh_id,
            "to_warehouse_id": to_wh_id,
            "material_id": material_id,
            "quantity": "5"
        }))
        .await;
    create_resp.assert_status_ok();
    let transfer_id = create_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    // Get stock transfer by ID
    let resp = server
        .get(&format!("/api/v1/wm/stock-transfers/{}", transfer_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["id"].as_str().unwrap(), transfer_id);
}

// ---------------------------------------------------------------------------
// Unauthenticated access must be rejected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn wm_warehouses_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/wm/warehouses").await;
    resp.assert_status_unauthorized();
}
