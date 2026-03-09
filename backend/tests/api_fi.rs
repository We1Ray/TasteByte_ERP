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
// FI - Accounts
// ---------------------------------------------------------------------------

#[tokio::test]
async fn fi_list_accounts() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/fi/accounts")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["items"].is_array());
}

#[tokio::test]
async fn fi_create_account() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/fi/accounts")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "account_number": format!("T{}", &uuid::Uuid::new_v4().simple().to_string()[..10]),
            "name": "Integration Test Account",
            "account_type": "ASSET"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn fi_create_account_missing_name_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .post("/api/v1/fi/accounts")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "account_number": "X999",
            "name": "",
            "account_type": "ASSET"
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// FI - Account Groups
// ---------------------------------------------------------------------------

#[tokio::test]
async fn fi_list_account_groups() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/fi/account-groups")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// FI - Company Codes
// ---------------------------------------------------------------------------

#[tokio::test]
async fn fi_list_company_codes() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/fi/company-codes")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// FI - Journal Entries
// ---------------------------------------------------------------------------

#[tokio::test]
async fn fi_list_journal_entries() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/fi/journal-entries")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["items"].is_array());
}

#[tokio::test]
async fn fi_create_journal_entry() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // First, get a company code and two accounts
    let cc_resp = server
        .get("/api/v1/fi/company-codes")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    let cc_body: serde_json::Value = cc_resp.json();
    let empty = vec![];
    let company_codes = cc_body["data"].as_array().unwrap_or(&empty);
    if company_codes.is_empty() {
        // No company codes in DB, skip
        return;
    }
    let company_code_id = company_codes[0]["id"].as_str().unwrap();

    let acct_resp = server
        .get("/api/v1/fi/accounts")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    let acct_body: serde_json::Value = acct_resp.json();
    let empty_accts = vec![];
    let accounts = acct_body["data"]["items"]
        .as_array()
        .unwrap_or(&empty_accts);
    if accounts.len() < 2 {
        return;
    }
    let account_a = accounts[0]["id"].as_str().unwrap();
    let account_b = accounts[1]["id"].as_str().unwrap();

    let resp = server
        .post("/api/v1/fi/journal-entries")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "company_code_id": company_code_id,
            "posting_date": "2026-01-15",
            "document_date": "2026-01-15",
            "description": "Integration test JE",
            "items": [
                { "account_id": account_a, "debit_amount": "100.00", "credit_amount": "0.00" },
                { "account_id": account_b, "debit_amount": "0.00", "credit_amount": "100.00" }
            ]
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn fi_create_journal_entry_empty_items_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    let resp = server
        .post("/api/v1/fi/journal-entries")
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "company_code_id": "00000000-0000-0000-0000-000000000001",
            "posting_date": "2026-01-15",
            "document_date": "2026-01-15",
            "items": []
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// FI - AR / AP Invoices
// ---------------------------------------------------------------------------

#[tokio::test]
async fn fi_list_ar_invoices() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/fi/ar-invoices")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn fi_list_ap_invoices() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/fi/ap-invoices")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

// ---------------------------------------------------------------------------
// FI - Reports
// ---------------------------------------------------------------------------

#[tokio::test]
async fn fi_report_trial_balance() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/fi/reports/trial-balance")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn fi_report_income_statement() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/fi/reports/income-statement")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn fi_report_balance_sheet() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;
    let resp = server
        .get("/api/v1/fi/reports/balance-sheet")
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
}

// ---------------------------------------------------------------------------
// Unauthenticated access must be rejected
// ---------------------------------------------------------------------------

#[tokio::test]
async fn fi_accounts_requires_auth() {
    let server = common::setup_server().await;
    let resp = server.get("/api/v1/fi/accounts").await;
    resp.assert_status_unauthorized();
}
