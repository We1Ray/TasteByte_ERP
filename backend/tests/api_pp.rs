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
// PP - BOMs
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pp_list_boms() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/pp/boms")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn pp_create_bom() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create finished material and component material
    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "BOM Finished Product",
            "material_type": "FINISHED"
        }))
        .await;
    let mat_body: serde_json::Value = mat_resp.json();
    let finished_id = mat_body["data"]["id"].as_str().unwrap();

    let comp_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "BOM Component",
            "material_type": "RAW"
        }))
        .await;
    let comp_body: serde_json::Value = comp_resp.json();
    let component_id = comp_body["data"]["id"].as_str().unwrap();

    let resp = server
        .post("/api/v1/pp/boms")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": finished_id,
            "name": "Integration Test BOM",
            "items": [{
                "component_material_id": component_id,
                "quantity": "2.5"
            }]
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn pp_create_bom_empty_items_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/pp/boms")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": "00000000-0000-0000-0000-000000000001",
            "name": "Empty BOM",
            "items": []
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// PP - Routings
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pp_list_routings() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/pp/routings")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn pp_create_routing() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "Routing Test Material",
            "material_type": "FINISHED"
        }))
        .await;
    let mat_body: serde_json::Value = mat_resp.json();
    let material_id = mat_body["data"]["id"].as_str().unwrap();

    let resp = server
        .post("/api/v1/pp/routings")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material_id,
            "name": "Integration Test Routing",
            "operations": [{
                "operation_number": 10,
                "work_center": "WC-ASSEMBLY",
                "description": "Assembly step",
                "setup_time_minutes": 15,
                "run_time_minutes": 30
            }]
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// PP - Production Orders
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pp_list_production_orders() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/pp/production-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["items"].is_array());
}

#[tokio::test]
async fn pp_create_production_order() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // Create material, BOM
    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "ProdOrder Finished",
            "material_type": "FINISHED"
        }))
        .await;
    let mat_body: serde_json::Value = mat_resp.json();
    let material_id = mat_body["data"]["id"].as_str().unwrap();

    let comp_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "name": "ProdOrder Component",
            "material_type": "RAW"
        }))
        .await;
    let comp_body: serde_json::Value = comp_resp.json();
    let component_id = comp_body["data"]["id"].as_str().unwrap();

    let bom_resp = server
        .post("/api/v1/pp/boms")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material_id,
            "name": "ProdOrder BOM",
            "items": [{
                "component_material_id": component_id,
                "quantity": "1"
            }]
        }))
        .await;
    let bom_body: serde_json::Value = bom_resp.json();
    let bom_id = bom_body["data"]["id"].as_str().unwrap();

    let resp = server
        .post("/api/v1/pp/production-orders")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "material_id": material_id,
            "bom_id": bom_id,
            "planned_quantity": "100",
            "planned_start": "2026-03-01",
            "planned_end": "2026-03-15"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

// ---------------------------------------------------------------------------
// PP - Reports
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pp_report_production_analysis() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/pp/reports/production-analysis")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

// ---------------------------------------------------------------------------
// PP - BOM Item Sub-table CRUD
// ---------------------------------------------------------------------------

/// Helper: create a BOM with one item. Returns (bom_id, component_material_id).
async fn create_bom_with_item(
    server: &axum_test::TestServer,
    token: &str,
) -> (String, String) {
    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "name": "BOM Item Test Finished",
            "material_type": "FINISHED"
        }))
        .await;
    let finished_id = mat_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let comp_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "name": "BOM Item Test Component",
            "material_type": "RAW"
        }))
        .await;
    let component_id = comp_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let bom_resp = server
        .post("/api/v1/pp/boms")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "material_id": finished_id,
            "name": "BOM Item Test BOM",
            "items": [{
                "component_material_id": component_id,
                "quantity": "2.5"
            }]
        }))
        .await;
    bom_resp.assert_status_ok();
    let bom_id = bom_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    (bom_id, component_id)
}

#[tokio::test]
async fn pp_add_bom_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (bom_id, component_id) = create_bom_with_item(&server, &token).await;

    let resp = server
        .post(&format!("/api/v1/pp/boms/{}/items", bom_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "component_material_id": component_id,
            "quantity": "1.0"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn pp_update_bom_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (bom_id, _component_id) = create_bom_with_item(&server, &token).await;

    // Get BOM detail to find item_id
    let detail_resp = server
        .get(&format!("/api/v1/pp/boms/{}", bom_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    detail_resp.assert_status_ok();
    let detail: serde_json::Value = detail_resp.json();
    let item_id = detail["data"]["items"][0]["id"].as_str().unwrap();

    let resp = server
        .put(&format!("/api/v1/pp/boms/{}/items/{}", bom_id, item_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "quantity": "5.0",
            "scrap_percentage": "2.5"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn pp_delete_bom_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let (bom_id, component_id) = create_bom_with_item(&server, &token).await;

    // Add a second item so we can delete one
    let add_resp = server
        .post(&format!("/api/v1/pp/boms/{}/items", bom_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "component_material_id": component_id,
            "quantity": "0.5"
        }))
        .await;
    add_resp.assert_status_ok();
    let added_item_id = add_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let resp = server
        .delete(&format!(
            "/api/v1/pp/boms/{}/items/{}",
            bom_id, added_item_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// PP - Routing Operation Sub-table CRUD
// ---------------------------------------------------------------------------

/// Helper: create a routing with one operation. Returns routing_id.
async fn create_routing_with_op(
    server: &axum_test::TestServer,
    token: &str,
) -> String {
    let mat_resp = server
        .post("/api/v1/mm/materials")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "name": "Routing Op Test Material",
            "material_type": "FINISHED"
        }))
        .await;
    let material_id = mat_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let routing_resp = server
        .post("/api/v1/pp/routings")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "material_id": material_id,
            "name": "Routing Op Test",
            "operations": [{
                "operation_number": 10,
                "work_center": "WC-MILL",
                "description": "Milling",
                "setup_time_minutes": 10,
                "run_time_minutes": 20
            }]
        }))
        .await;
    routing_resp.assert_status_ok();
    routing_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn pp_add_routing_operation() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let routing_id = create_routing_with_op(&server, &token).await;

    let resp = server
        .post(&format!(
            "/api/v1/pp/routings/{}/operations",
            routing_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "operation_number": 20,
            "work_center": "WC-ASSEMBLY",
            "description": "Assembly step",
            "setup_time_minutes": 5,
            "run_time_minutes": 15
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn pp_update_routing_operation() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let routing_id = create_routing_with_op(&server, &token).await;

    // Get routing detail to find operation_id
    let detail_resp = server
        .get(&format!("/api/v1/pp/routings/{}", routing_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    detail_resp.assert_status_ok();
    let detail: serde_json::Value = detail_resp.json();
    let op_id = detail["data"]["operations"][0]["id"].as_str().unwrap();

    let resp = server
        .put(&format!(
            "/api/v1/pp/routings/{}/operations/{}",
            routing_id, op_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "work_center": "WC-UPDATED",
            "run_time_minutes": 45
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn pp_delete_routing_operation() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let routing_id = create_routing_with_op(&server, &token).await;

    // Add a second operation so we can delete one
    let add_resp = server
        .post(&format!(
            "/api/v1/pp/routings/{}/operations",
            routing_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "operation_number": 30,
            "work_center": "WC-PAINT",
            "description": "Painting"
        }))
        .await;
    add_resp.assert_status_ok();
    let added_op_id = add_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let resp = server
        .delete(&format!(
            "/api/v1/pp/routings/{}/operations/{}",
            routing_id, added_op_id
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
async fn pp_production_orders_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/pp/production-orders").await;
    resp.assert_status_unauthorized();
}
