mod common;

use serde_json::json;

#[tokio::test]
async fn login_success() {
    let server = common::setup_server().await;
    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({"username": "admin", "password": "admin123"}))
        .await;
    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["access_token"].as_str().is_some());
    assert!(body["data"]["refresh_token"].as_str().is_some());
}

#[tokio::test]
async fn login_wrong_password() {
    let server = common::setup_server().await;
    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({"username": "admin", "password": "wrongpassword"}))
        .await;
    response.assert_status_unauthorized();
}

#[tokio::test]
async fn login_empty_username_rejected() {
    let server = common::setup_server().await;
    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({"username": "", "password": "admin123"}))
        .await;
    response.assert_status_bad_request();
}

#[tokio::test]
async fn refresh_with_invalid_token_rejected() {
    let server = common::setup_server().await;
    let response = server
        .post("/api/v1/auth/refresh")
        .json(&json!({"refresh_token": "invalid_token"}))
        .await;
    response.assert_status_unauthorized();
}
