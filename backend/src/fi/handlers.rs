use axum::extract::{Path, Query, State};
use axum::response::Response;
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::auth::rbac::{FiRead, FiWrite, RequireRole};
use crate::fi::models::*;
use crate::fi::services;
use crate::shared::audit;
use crate::shared::export::csv_response;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, ListParams, PaginatedResponse};

// --- Accounts ---
pub async fn list_accounts(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Account>>>, AppError> {
    let result = services::list_accounts(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_account(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Account>>, AppError> {
    let account = services::get_account(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(account)))
}

pub async fn create_account(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Json(input): Json<CreateAccount>,
) -> Result<Json<ApiResponse<Account>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let account = services::create_account(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_accounts",
        account.id,
        "CREATE",
        None,
        serde_json::to_value(&account).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(account, "Account created")))
}

pub async fn update_account(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateAccount>,
) -> Result<Json<ApiResponse<Account>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let account = services::update_account(&state.pool, id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_accounts",
        id,
        "UPDATE",
        None,
        serde_json::to_value(&account).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(account, "Account updated")))
}

// --- Account Groups ---
pub async fn list_account_groups(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<AccountGroup>>>, AppError> {
    let result = services::list_account_groups(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_account_group(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Json(input): Json<CreateAccountGroup>,
) -> Result<Json<ApiResponse<AccountGroup>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let group = services::create_account_group(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_account_groups",
        group.id,
        "CREATE",
        None,
        serde_json::to_value(&group).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        group,
        "Account group created",
    )))
}

// --- Company Codes ---
pub async fn list_company_codes(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<CompanyCode>>>, AppError> {
    let result = services::list_company_codes(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_company_code(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Json(input): Json<CreateCompanyCode>,
) -> Result<Json<ApiResponse<CompanyCode>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let code = services::create_company_code(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_company_codes",
        code.id,
        "CREATE",
        None,
        serde_json::to_value(&code).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        code,
        "Company code created",
    )))
}

// --- Fiscal Years ---
pub async fn list_fiscal_years(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Path(company_code_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<FiscalYear>>>, AppError> {
    let years = services::list_fiscal_years(&state.pool, company_code_id).await?;
    Ok(Json(ApiResponse::success(years)))
}

pub async fn create_fiscal_year(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Json(input): Json<CreateFiscalYear>,
) -> Result<Json<ApiResponse<FiscalYear>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let year = services::create_fiscal_year(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_fiscal_years",
        year.id,
        "CREATE",
        None,
        serde_json::to_value(&year).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(year, "Fiscal year created")))
}

// --- Journal Entries ---
pub async fn list_journal_entries(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<JournalEntry>>>, AppError> {
    let result = services::list_journal_entries(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

#[derive(serde::Serialize)]
pub struct JournalEntryDetail {
    #[serde(flatten)]
    pub entry: JournalEntry,
    pub items: Vec<JournalItem>,
}

pub async fn get_journal_entry(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<JournalEntryDetail>>, AppError> {
    let (entry, items) = services::get_journal_entry(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(JournalEntryDetail {
        entry,
        items,
    })))
}

pub async fn create_journal_entry(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Json(input): Json<CreateJournalEntry>,
) -> Result<Json<ApiResponse<JournalEntry>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let entry = services::create_journal_entry(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_journal_entries",
        entry.id,
        "CREATE",
        None,
        serde_json::to_value(&entry).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        entry,
        "Journal entry created",
    )))
}

pub async fn post_journal_entry(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<JournalEntry>>, AppError> {
    let entry = services::post_journal_entry(&state.pool, id, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_journal_entries",
        id,
        "UPDATE",
        serde_json::to_value(serde_json::json!({"status": "DRAFT"})).ok(),
        serde_json::to_value(serde_json::json!({"status": &entry.status})).ok(),
        Some(role.claims.sub),
    )
    .await;

    // FI->CO auto-posting (best-effort, after the main operation succeeds)
    services::co_auto_post_for_journal(
        &state.pool,
        entry.id,
        &entry.document_number,
        entry.posting_date,
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        entry,
        "Journal entry posted",
    )))
}

// --- AR Invoices ---
pub async fn list_ar_invoices(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<ArInvoice>>>, AppError> {
    let result = services::list_ar_invoices(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_ar_invoice(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Json(input): Json<CreateArInvoice>,
) -> Result<Json<ApiResponse<ArInvoice>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let invoice = services::create_ar_invoice(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_ar_invoices",
        invoice.id,
        "CREATE",
        None,
        serde_json::to_value(&invoice).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        invoice,
        "AR invoice created",
    )))
}

// --- AP Invoices ---
pub async fn list_ap_invoices(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<ApInvoice>>>, AppError> {
    let result = services::list_ap_invoices(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_ap_invoice(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Json(input): Json<CreateApInvoice>,
) -> Result<Json<ApiResponse<ApInvoice>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let invoice = services::create_ap_invoice(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_ap_invoices",
        invoice.id,
        "CREATE",
        None,
        serde_json::to_value(&invoice).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        invoice,
        "AP invoice created",
    )))
}

// --- AR Payment ---
pub async fn record_ar_payment(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<RecordPaymentInput>,
) -> Result<Json<ApiResponse<ArInvoice>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let result = services::record_ar_payment(&state.pool, id, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_ar_invoices",
        id,
        "PAYMENT",
        None,
        serde_json::to_value(&result).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        result,
        "Payment recorded successfully",
    )))
}

// --- AP Payment ---
pub async fn record_ap_payment(
    State(state): State<AppState>,
    role: RequireRole<FiWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<RecordPaymentInput>,
) -> Result<Json<ApiResponse<ApInvoice>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let result = services::record_ap_payment(&state.pool, id, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "fi_ap_invoices",
        id,
        "PAYMENT",
        None,
        serde_json::to_value(&result).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        result,
        "Payment recorded successfully",
    )))
}

// --- Export Journal Entries ---
pub async fn export_journal_entries(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
) -> Result<Response, AppError> {
    let entries = sqlx::query_as::<_, JournalEntry>(
        "SELECT * FROM fi_journal_entries ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record([
        "Document Number",
        "Company Code ID",
        "Fiscal Year",
        "Fiscal Period",
        "Posting Date",
        "Document Date",
        "Reference",
        "Description",
        "Status",
        "Created At",
    ])
    .map_err(|e| AppError::Internal(e.to_string()))?;

    for je in &entries {
        let cc_id = je.company_code_id.to_string();
        let fy = je.fiscal_year.to_string();
        let fp = je.fiscal_period.to_string();
        let posting = je.posting_date.to_string();
        let doc_date = je.document_date.to_string();
        let created = je.created_at.to_rfc3339();
        wtr.write_record([
            je.document_number.as_str(),
            cc_id.as_str(),
            fy.as_str(),
            fp.as_str(),
            posting.as_str(),
            doc_date.as_str(),
            je.reference.as_deref().unwrap_or(""),
            je.description.as_deref().unwrap_or(""),
            je.status.as_str(),
            created.as_str(),
        ])
        .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let csv_data = String::from_utf8(
        wtr.into_inner()
            .map_err(|e| AppError::Internal(e.to_string()))?,
    )
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(csv_response(csv_data, "journal-entries-export.csv"))
}
