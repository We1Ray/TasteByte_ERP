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
// Unauthenticated access must be rejected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pp_production_orders_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/pp/production-orders").await;
    resp.assert_status_unauthorized();
}
