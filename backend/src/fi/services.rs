use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::fi::models::*;
use crate::fi::repositories;
use crate::shared::pagination::ListParams;
use crate::shared::status::DocumentType;
use crate::shared::status_history;
use crate::shared::{AppError, PaginatedResponse};

pub async fn list_accounts(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Account>, AppError> {
    let total = repositories::count_accounts(pool, params).await?;
    let data = repositories::list_accounts(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_account(pool: &PgPool, id: Uuid) -> Result<Account, AppError> {
    repositories::get_account(pool, id).await
}

pub async fn create_account(pool: &PgPool, input: CreateAccount) -> Result<Account, AppError> {
    repositories::create_account(pool, &input).await
}

pub async fn update_account(
    pool: &PgPool,
    id: Uuid,
    input: UpdateAccount,
) -> Result<Account, AppError> {
    repositories::update_account(pool, id, &input).await
}

pub async fn list_account_groups(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<AccountGroup>, AppError> {
    let total = repositories::count_account_groups(pool, params).await?;
    let data = repositories::list_account_groups(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn create_account_group(
    pool: &PgPool,
    input: CreateAccountGroup,
) -> Result<AccountGroup, AppError> {
    repositories::create_account_group(pool, &input).await
}

pub async fn list_company_codes(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<CompanyCode>, AppError> {
    let total = repositories::count_company_codes(pool, params).await?;
    let data = repositories::list_company_codes(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn create_company_code(
    pool: &PgPool,
    input: CreateCompanyCode,
) -> Result<CompanyCode, AppError> {
    repositories::create_company_code(pool, &input).await
}

pub async fn list_fiscal_years(
    pool: &PgPool,
    company_code_id: Uuid,
) -> Result<Vec<FiscalYear>, AppError> {
    repositories::list_fiscal_years(pool, company_code_id).await
}

pub async fn create_fiscal_year(
    pool: &PgPool,
    input: CreateFiscalYear,
) -> Result<FiscalYear, AppError> {
    repositories::create_fiscal_year(pool, &input).await
}

pub async fn list_journal_entries(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<JournalEntry>, AppError> {
    let total = repositories::count_journal_entries(pool, params).await?;
    let data = repositories::list_journal_entries(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_journal_entry(
    pool: &PgPool,
    id: Uuid,
) -> Result<(JournalEntry, Vec<JournalItem>), AppError> {
    let entry = repositories::get_journal_entry(pool, id).await?;
    let items = repositories::get_journal_items(pool, id).await?;
    Ok((entry, items))
}

/// Create a journal entry. Called from handlers with `&PgPool`.
/// For transactional cross-module use, call `create_journal_entry_in_tx` instead.
pub async fn create_journal_entry(
    pool: &PgPool,
    input: CreateJournalEntry,
    user_id: Uuid,
) -> Result<JournalEntry, AppError> {
    validate_journal_entry_input(&input)?;
    let doc_number = repositories::next_number(pool, "JE").await?;
    let (fiscal_year, fiscal_period) = compute_fiscal_period(&input);
    validate_fiscal_year(pool, fiscal_year, input.posting_date).await?;
    repositories::create_journal_entry(
        pool,
        &doc_number,
        &input,
        user_id,
        fiscal_year,
        fiscal_period,
    )
    .await
}

/// Create a journal entry within an existing transaction.
pub async fn create_journal_entry_in_tx(
    tx: &mut sqlx::PgConnection,
    input: CreateJournalEntry,
    user_id: Uuid,
) -> Result<JournalEntry, AppError> {
    validate_journal_entry_input(&input)?;
    let doc_number = repositories::next_number_on_conn(&mut *tx, "JE").await?;
    let (fiscal_year, fiscal_period) = compute_fiscal_period(&input);
    validate_fiscal_year_on_conn(&mut *tx, fiscal_year, input.posting_date).await?;
    repositories::create_journal_entry_on_conn(
        &mut *tx,
        &doc_number,
        &input,
        user_id,
        fiscal_year,
        fiscal_period,
    )
    .await
}

fn validate_journal_entry_input(input: &CreateJournalEntry) -> Result<(), AppError> {
    let total_debit: Decimal = input.items.iter().map(|i| i.debit_amount).sum();
    let total_credit: Decimal = input.items.iter().map(|i| i.credit_amount).sum();
    if total_debit != total_credit {
        return Err(AppError::Validation(
            "Total debits must equal total credits".to_string(),
        ));
    }
    Ok(())
}

fn compute_fiscal_period(input: &CreateJournalEntry) -> (i32, i32) {
    let fiscal_year = input
        .posting_date
        .format("%Y")
        .to_string()
        .parse::<i32>()
        .unwrap_or(2024);
    let fiscal_period = input
        .posting_date
        .format("%m")
        .to_string()
        .parse::<i32>()
        .unwrap_or(1);
    (fiscal_year, fiscal_period)
}

async fn validate_fiscal_year(
    pool: &PgPool,
    fiscal_year: i32,
    posting_date: chrono::NaiveDate,
) -> Result<(), AppError> {
    let fy_row: Option<(bool,)> = sqlx::query_as(
        "SELECT is_closed FROM fi_fiscal_years WHERE year = $1 AND start_date <= $2 AND end_date >= $2 LIMIT 1",
    )
    .bind(fiscal_year)
    .bind(posting_date)
    .fetch_optional(pool)
    .await?;
    handle_fiscal_year_result(fy_row, fiscal_year, posting_date)
}

async fn validate_fiscal_year_on_conn(
    conn: &mut sqlx::PgConnection,
    fiscal_year: i32,
    posting_date: chrono::NaiveDate,
) -> Result<(), AppError> {
    let fy_row: Option<(bool,)> = sqlx::query_as(
        "SELECT is_closed FROM fi_fiscal_years WHERE year = $1 AND start_date <= $2 AND end_date >= $2 LIMIT 1",
    )
    .bind(fiscal_year)
    .bind(posting_date)
    .fetch_optional(&mut *conn)
    .await?;
    handle_fiscal_year_result(fy_row, fiscal_year, posting_date)
}

fn handle_fiscal_year_result(
    fy_row: Option<(bool,)>,
    fiscal_year: i32,
    posting_date: chrono::NaiveDate,
) -> Result<(), AppError> {
    match fy_row {
        Some((true,)) => Err(AppError::Validation(
            "No open fiscal year for the posting date".to_string(),
        )),
        None => {
            tracing::warn!(
                "No fiscal year record found for year {} / posting date {}. Proceeding without fiscal year validation.",
                fiscal_year, posting_date
            );
            Ok(())
        }
        Some((false,)) => Ok(()),
    }
}

/// Post a journal entry. Called from handlers with `&PgPool`.
/// For transactional cross-module use, call `post_journal_entry_in_tx` instead.
pub async fn post_journal_entry(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<JournalEntry, AppError> {
    let result = repositories::post_journal_entry(pool, id).await?;
    status_history::record_transition(
        pool,
        &DocumentType::JournalEntry,
        id,
        Some("DRAFT"),
        "POSTED",
        user_id,
        None,
    )
    .await?;

    crate::shared::audit::log_change(
        pool,
        "fi_journal_entries",
        id,
        "POST",
        None,
        Some(serde_json::json!({
            "status": "POSTED",
            "document_number": &result.document_number,
        })),
        Some(user_id),
    )
    .await
    .ok();

    Ok(result)
}

/// Post a journal entry within an existing transaction.
pub async fn post_journal_entry_in_tx(
    tx: &mut sqlx::PgConnection,
    id: Uuid,
    user_id: Uuid,
) -> Result<JournalEntry, AppError> {
    let result = repositories::post_journal_entry_on_conn(&mut *tx, id).await?;
    status_history::record_transition(
        &mut *tx,
        &DocumentType::JournalEntry,
        id,
        Some("DRAFT"),
        "POSTED",
        user_id,
        None,
    )
    .await?;

    crate::shared::audit::log_change(
        &mut *tx,
        "fi_journal_entries",
        id,
        "POST",
        None,
        Some(serde_json::json!({
            "status": "POSTED",
            "document_number": &result.document_number,
        })),
        Some(user_id),
    )
    .await
    .ok();

    Ok(result)
}

/// FI->CO auto-posting helper: for each journal line with a cost_center_id,
/// create a CO cost allocation record. Must be called AFTER the transaction commits.
pub async fn co_auto_post_for_journal(
    pool: &PgPool,
    je_id: Uuid,
    document_number: &str,
    posting_date: chrono::NaiveDate,
) {
    let items = match repositories::get_journal_items(pool, je_id).await {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to load journal items for CO auto-posting: {}", e);
            return;
        }
    };
    for item in &items {
        if let Some(cost_center_id) = item.cost_center_id {
            let amount = if item.debit_amount > Decimal::ZERO {
                item.debit_amount
            } else {
                item.credit_amount
            };

            let description = format!(
                "FI auto-post: JE {} line {} - {}",
                document_number,
                item.line_number,
                item.description.as_deref().unwrap_or("journal entry")
            );

            if let Err(e) = crate::co::services::auto_post_cost_allocation(
                pool,
                "FI",
                je_id,
                cost_center_id,
                amount,
                posting_date,
                &description,
            )
            .await
            {
                tracing::error!(
                    "FI->CO auto-posting failed for JE {} line {}: {} (data inconsistency: cost allocation missing)",
                    document_number,
                    item.line_number,
                    e
                );
            }
        }
    }
}

pub async fn list_ar_invoices(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<ArInvoice>, AppError> {
    let total = repositories::count_ar_invoices(pool, params).await?;
    let data = repositories::list_ar_invoices(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

/// Create an AR invoice. Called from handlers with `&PgPool`.
/// For transactional cross-module use, call `create_ar_invoice_in_tx` instead.
pub async fn create_ar_invoice(
    pool: &PgPool,
    input: CreateArInvoice,
) -> Result<ArInvoice, AppError> {
    let doc_number = repositories::next_number(pool, "INV").await?;
    repositories::create_ar_invoice(pool, &doc_number, &input).await
}

/// Create an AR invoice within an existing transaction.
pub async fn create_ar_invoice_in_tx(
    tx: &mut sqlx::PgConnection,
    input: CreateArInvoice,
) -> Result<ArInvoice, AppError> {
    let doc_number = repositories::next_number_on_conn(&mut *tx, "INV").await?;
    repositories::create_ar_invoice_on_conn(&mut *tx, &doc_number, &input).await
}

pub async fn list_ap_invoices(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<ApInvoice>, AppError> {
    let total = repositories::count_ap_invoices(pool, params).await?;
    let data = repositories::list_ap_invoices(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

/// Create an AP invoice. Called from handlers with `&PgPool`.
/// For transactional cross-module use, call `create_ap_invoice_in_tx` instead.
pub async fn create_ap_invoice(
    pool: &PgPool,
    input: CreateApInvoice,
) -> Result<ApInvoice, AppError> {
    let doc_number = repositories::next_number(pool, "INV").await?;
    repositories::create_ap_invoice(pool, &doc_number, &input).await
}

/// Create an AP invoice within an existing transaction.
pub async fn create_ap_invoice_in_tx(
    tx: &mut sqlx::PgConnection,
    input: CreateApInvoice,
) -> Result<ApInvoice, AppError> {
    let doc_number = repositories::next_number_on_conn(&mut *tx, "INV").await?;
    repositories::create_ap_invoice_on_conn(&mut *tx, &doc_number, &input).await
}

// --- Payment Processing ---

/// Record a payment against an AR invoice (customer payment received).
/// Creates journal entry: DR Bank(1100), CR AR(1200), then updates invoice paid_amount.
pub async fn record_ar_payment(
    pool: &PgPool,
    invoice_id: Uuid,
    input: RecordPaymentInput,
    user_id: Uuid,
) -> Result<ArInvoice, AppError> {
    // 1. Fetch and validate
    let invoice = repositories::get_ar_invoice(pool, invoice_id).await?;
    if invoice.status != "OPEN" && invoice.status != "PARTIALLY_PAID" {
        return Err(AppError::Validation(
            "Invoice is not open for payment".to_string(),
        ));
    }
    if input.amount <= Decimal::ZERO {
        return Err(AppError::Validation(
            "Payment amount must be positive".to_string(),
        ));
    }
    let remaining = invoice.total_amount - invoice.paid_amount;
    if input.amount > remaining {
        return Err(AppError::Validation(format!(
            "Payment amount {} exceeds remaining balance {}",
            input.amount, remaining
        )));
    }

    // 2. Begin transaction
    let mut tx = pool.begin().await?;

    // 3. Look up accounts
    let bank_account =
        sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_accounts WHERE account_number = '1100'")
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::Internal("Bank account 1100 not found".to_string()))?
            .0;

    let ar_account =
        sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_accounts WHERE account_number = '1200'")
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::Internal("AR account 1200 not found".to_string()))?
            .0;

    let cc_id = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_company_codes LIMIT 1")
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::Internal("No company code found".to_string()))?
        .0;

    // 4. Create journal entry: DR Bank(1100), CR AR(1200)
    let je_input = CreateJournalEntry {
        company_code_id: cc_id,
        posting_date: input.payment_date,
        document_date: chrono::Utc::now().date_naive(),
        reference: Some(format!("AR-PAY:{}", invoice.document_number)),
        description: Some(format!(
            "Customer payment for invoice {}",
            invoice.document_number
        )),
        items: vec![
            CreateJournalItem {
                account_id: bank_account,
                debit_amount: input.amount,
                credit_amount: Decimal::ZERO,
                cost_center_id: None,
                description: Some("Cash received from customer".to_string()),
            },
            CreateJournalItem {
                account_id: ar_account,
                debit_amount: Decimal::ZERO,
                credit_amount: input.amount,
                cost_center_id: None,
                description: Some("AR payment received".to_string()),
            },
        ],
    };
    let je = create_journal_entry_in_tx(&mut *tx, je_input, user_id).await?;
    post_journal_entry_in_tx(&mut *tx, je.id, user_id).await?;

    // 5. Update invoice paid_amount and status
    let updated =
        repositories::update_ar_invoice_payment_on_conn(&mut *tx, invoice_id, input.amount).await?;

    // 6. Create payment document
    let pay_number = repositories::next_number_on_conn(&mut *tx, "PAY").await?;
    repositories::create_payment_document_on_conn(
        &mut *tx,
        &pay_number,
        "AR",
        invoice_id,
        input.amount,
        input.payment_date,
        Some(je.id),
        user_id,
    )
    .await?;

    tx.commit().await?;
    Ok(updated)
}

/// Record a payment against an AP invoice (vendor payment made).
/// Creates journal entry: DR AP(2100), CR Bank(1100), then updates invoice paid_amount.
pub async fn record_ap_payment(
    pool: &PgPool,
    invoice_id: Uuid,
    input: RecordPaymentInput,
    user_id: Uuid,
) -> Result<ApInvoice, AppError> {
    // 1. Fetch and validate
    let invoice = repositories::get_ap_invoice(pool, invoice_id).await?;
    if invoice.status != "OPEN" && invoice.status != "PARTIALLY_PAID" {
        return Err(AppError::Validation(
            "Invoice is not open for payment".to_string(),
        ));
    }
    if input.amount <= Decimal::ZERO {
        return Err(AppError::Validation(
            "Payment amount must be positive".to_string(),
        ));
    }
    let remaining = invoice.total_amount - invoice.paid_amount;
    if input.amount > remaining {
        return Err(AppError::Validation(format!(
            "Payment amount {} exceeds remaining balance {}",
            input.amount, remaining
        )));
    }

    // 2. Begin transaction
    let mut tx = pool.begin().await?;

    // 3. Look up accounts
    let ap_account =
        sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_accounts WHERE account_number = '2100'")
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::Internal("AP account 2100 not found".to_string()))?
            .0;

    let bank_account =
        sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_accounts WHERE account_number = '1100'")
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::Internal("Bank account 1100 not found".to_string()))?
            .0;

    let cc_id = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_company_codes LIMIT 1")
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::Internal("No company code found".to_string()))?
        .0;

    // 4. Create journal entry: DR AP(2100), CR Bank(1100)
    let je_input = CreateJournalEntry {
        company_code_id: cc_id,
        posting_date: input.payment_date,
        document_date: chrono::Utc::now().date_naive(),
        reference: Some(format!("AP-PAY:{}", invoice.document_number)),
        description: Some(format!(
            "Vendor payment for invoice {}",
            invoice.document_number
        )),
        items: vec![
            CreateJournalItem {
                account_id: ap_account,
                debit_amount: input.amount,
                credit_amount: Decimal::ZERO,
                cost_center_id: None,
                description: Some("AP payment made".to_string()),
            },
            CreateJournalItem {
                account_id: bank_account,
                debit_amount: Decimal::ZERO,
                credit_amount: input.amount,
                cost_center_id: None,
                description: Some("Cash paid to vendor".to_string()),
            },
        ],
    };
    let je = create_journal_entry_in_tx(&mut *tx, je_input, user_id).await?;
    post_journal_entry_in_tx(&mut *tx, je.id, user_id).await?;

    // 5. Update invoice paid_amount and status
    let updated =
        repositories::update_ap_invoice_payment_on_conn(&mut *tx, invoice_id, input.amount).await?;

    // 6. Create payment document
    let pay_number = repositories::next_number_on_conn(&mut *tx, "PAY").await?;
    repositories::create_payment_document_on_conn(
        &mut *tx,
        &pay_number,
        "AP",
        invoice_id,
        input.amount,
        input.payment_date,
        Some(je.id),
        user_id,
    )
    .await?;

    tx.commit().await?;
    Ok(updated)
}
