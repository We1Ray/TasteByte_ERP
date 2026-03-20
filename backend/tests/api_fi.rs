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
// FI - Journal Entry Item Sub-table CRUD
// ---------------------------------------------------------------------------

/// Helper: create a journal entry in DRAFT status. Returns (je_id, account_a_id, account_b_id).
/// Skips (returns None) if prerequisite data is missing.
async fn create_draft_journal_entry(
    server: &axum_test::TestServer,
    token: &str,
) -> Option<(String, String, String)> {
    // Get a company code
    let cc_resp = server
        .get("/api/v1/fi/company-codes")
        .add_header(auth(token).0, auth(token).1)
        .await;
    let cc_body: serde_json::Value = cc_resp.json();
    let empty = vec![];
    let company_codes = cc_body["data"].as_array().unwrap_or(&empty);
    if company_codes.is_empty() {
        return None;
    }
    let company_code_id = company_codes[0]["id"].as_str().unwrap().to_string();

    // Get at least 2 accounts
    let acct_resp = server
        .get("/api/v1/fi/accounts")
        .add_header(auth(token).0, auth(token).1)
        .await;
    let acct_body: serde_json::Value = acct_resp.json();
    let empty_accts = vec![];
    let accounts = acct_body["data"]["items"]
        .as_array()
        .unwrap_or(&empty_accts);
    if accounts.len() < 2 {
        return None;
    }
    let account_a = accounts[0]["id"].as_str().unwrap().to_string();
    let account_b = accounts[1]["id"].as_str().unwrap().to_string();

    let resp = server
        .post("/api/v1/fi/journal-entries")
        .add_header(auth(token).0, auth(token).1)
        .json(&json!({
            "company_code_id": company_code_id,
            "posting_date": "2026-01-15",
            "document_date": "2026-01-15",
            "description": "Sub-table test JE",
            "items": [
                { "account_id": account_a, "debit_amount": "100.00", "credit_amount": "0.00" },
                { "account_id": account_b, "debit_amount": "0.00", "credit_amount": "100.00" }
            ]
        }))
        .await;
    resp.assert_status_ok();
    let je_id = resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    Some((je_id, account_a, account_b))
}

#[tokio::test]
async fn fi_add_journal_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    let Some((je_id, account_a, _account_b)) =
        create_draft_journal_entry(&server, &token).await
    else {
        return; // skip if no prerequisites
    };

    let resp = server
        .post(&format!("/api/v1/fi/journal-entries/{}/items", je_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "account_id": account_a,
            "debit_amount": "50.00",
            "credit_amount": "0.00",
            "description": "Added line"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn fi_update_journal_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    let Some((je_id, _account_a, _account_b)) =
        create_draft_journal_entry(&server, &token).await
    else {
        return;
    };

    // Get JE detail to find item_id
    let detail_resp = server
        .get(&format!("/api/v1/fi/journal-entries/{}", je_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    detail_resp.assert_status_ok();
    let detail: serde_json::Value = detail_resp.json();
    let item_id = detail["data"]["items"][0]["id"].as_str().unwrap();

    let resp = server
        .put(&format!(
            "/api/v1/fi/journal-entries/{}/items/{}",
            je_id, item_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "debit_amount": "200.00",
            "credit_amount": "0.00"
        }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn fi_delete_journal_item() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    let Some((je_id, account_a, _account_b)) =
        create_draft_journal_entry(&server, &token).await
    else {
        return;
    };

    // Add a third item so we can safely delete one
    let add_resp = server
        .post(&format!("/api/v1/fi/journal-entries/{}/items", je_id))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "account_id": account_a,
            "debit_amount": "25.00",
            "credit_amount": "0.00"
        }))
        .await;
    add_resp.assert_status_ok();
    let added_item_id = add_resp.json::<serde_json::Value>()["data"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let resp = server
        .delete(&format!(
            "/api/v1/fi/journal-entries/{}/items/{}",
            je_id, added_item_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
}

#[tokio::test]
async fn fi_update_journal_item_posted_rejected() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    let Some((je_id, _account_a, _account_b)) =
        create_draft_journal_entry(&server, &token).await
    else {
        return;
    };

    // Get item_id
    let detail_resp = server
        .get(&format!("/api/v1/fi/journal-entries/{}", je_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    let detail: serde_json::Value = detail_resp.json();
    let item_id = detail["data"]["items"][0]["id"].as_str().unwrap();

    // Post the JE
    let post_resp = server
        .post(&format!("/api/v1/fi/journal-entries/{}/post", je_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    post_resp.assert_status_ok();

    // Now try to update item -- should be rejected (JE is POSTED)
    let resp = server
        .put(&format!(
            "/api/v1/fi/journal-entries/{}/items/{}",
            je_id, item_id
        ))
        .add_header(auth(&token).0, auth(&token).1)
        .json(&json!({
            "debit_amount": "999.00"
        }))
        .await;
    resp.assert_status_bad_request();
}

// ---------------------------------------------------------------------------
// FI - Get AR Invoice by ID
// ---------------------------------------------------------------------------

#[tokio::test]
async fn fi_get_ar_invoice() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // List AR invoices to find one from seed data
    let list_resp = server
        .get("/api/v1/fi/ar-invoices")
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
        // No AR invoices in seed data, skip
        return;
    }
    let invoice_id = items[0]["id"].as_str().unwrap();

    // Get AR invoice by ID
    let resp = server
        .get(&format!("/api/v1/fi/ar-invoices/{}", invoice_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["id"].as_str().unwrap(), invoice_id);
}

// ---------------------------------------------------------------------------
// FI - Get AP Invoice by ID
// ---------------------------------------------------------------------------

#[tokio::test]
async fn fi_get_ap_invoice() {
    let server = common::setup_server().await;
    let token = get_token(&server).await;

    // List AP invoices to find one from seed data
    let list_resp = server
        .get("/api/v1/fi/ap-invoices")
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
        // No AP invoices in seed data, skip
        return;
    }
    let invoice_id = items[0]["id"].as_str().unwrap();

    // Get AP invoice by ID
    let resp = server
        .get(&format!("/api/v1/fi/ap-invoices/{}", invoice_id))
        .add_header(auth(&token).0, auth(&token).1)
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["id"].as_str().unwrap(), invoice_id);
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
