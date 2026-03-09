mod common;

#[tokio::test]
async fn health_returns_ok() {
    let server = common::setup_server().await;
    let response = server.get("/health").await;
    response.assert_status_ok();
    response.assert_json_contains(&serde_json::json!({"status": "ok"}));
}
