use axum::{
    routing::{get, post, put},
    Router,
};

use crate::fi::handlers;
use crate::fi::reports;
use crate::shared::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        // Accounts
        .route(
            "/accounts",
            get(handlers::list_accounts).post(handlers::create_account),
        )
        .route(
            "/accounts/{id}",
            get(handlers::get_account).put(handlers::update_account),
        )
        // Account Groups
        .route(
            "/account-groups",
            get(handlers::list_account_groups).post(handlers::create_account_group),
        )
        // Company Codes
        .route(
            "/company-codes",
            get(handlers::list_company_codes).post(handlers::create_company_code),
        )
        // Fiscal Years
        .route(
            "/company-codes/{company_code_id}/fiscal-years",
            get(handlers::list_fiscal_years),
        )
        .route("/fiscal-years", post(handlers::create_fiscal_year))
        // Journal Entries
        .route(
            "/journal-entries/export",
            get(handlers::export_journal_entries),
        )
        .route(
            "/journal-entries",
            get(handlers::list_journal_entries).post(handlers::create_journal_entry),
        )
        .route("/journal-entries/{id}", get(handlers::get_journal_entry))
        .route(
            "/journal-entries/{je_id}/items",
            post(handlers::add_journal_item),
        )
        .route(
            "/journal-entries/{je_id}/items/{item_id}",
            put(handlers::update_journal_item).delete(handlers::delete_journal_item),
        )
        .route(
            "/journal-entries/{id}/post",
            post(handlers::post_journal_entry),
        )
        // AR Invoices
        .route(
            "/ar-invoices",
            get(handlers::list_ar_invoices).post(handlers::create_ar_invoice),
        )
        .route("/ar-invoices/{id}", get(handlers::get_ar_invoice))
        .route(
            "/ar-invoices/{id}/payment",
            post(handlers::record_ar_payment),
        )
        // AP Invoices
        .route(
            "/ap-invoices",
            get(handlers::list_ap_invoices).post(handlers::create_ap_invoice),
        )
        .route("/ap-invoices/{id}", get(handlers::get_ap_invoice))
        .route(
            "/ap-invoices/{id}/payment",
            post(handlers::record_ap_payment),
        )
        // Reports
        .route("/reports/trial-balance", get(reports::trial_balance))
        .route("/reports/income-statement", get(reports::income_statement))
        .route("/reports/balance-sheet", get(reports::balance_sheet))
        .route("/reports/ar-aging", get(reports::ar_aging))
        .route("/reports/ap-aging", get(reports::ap_aging))
}
