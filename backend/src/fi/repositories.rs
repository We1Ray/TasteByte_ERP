use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::fi::models::*;
use crate::shared::pagination::ListParams;
use crate::shared::AppError;

// Functions with `&PgPool` are used by handlers. Functions with `_on_conn` suffix
// accept `&mut PgConnection` and are used from within transactions in service functions.

// --- Accounts ---
pub async fn list_accounts(pool: &PgPool, params: &ListParams) -> Result<Vec<Account>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, Account>(
        r#"SELECT * FROM fi_accounts
           WHERE is_active = true
             AND ($1 = false OR (account_number ILIKE $2 OR name ILIKE $2))
           ORDER BY account_number
           LIMIT $3 OFFSET $4"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_accounts(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM fi_accounts
           WHERE is_active = true
             AND ($1 = false OR (account_number ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_account(pool: &PgPool, id: Uuid) -> Result<Account, AppError> {
    sqlx::query_as::<_, Account>("SELECT * FROM fi_accounts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Account not found".to_string()))
}

pub async fn create_account(pool: &PgPool, input: &CreateAccount) -> Result<Account, AppError> {
    let row = sqlx::query_as::<_, Account>(
        "INSERT INTO fi_accounts (account_number, name, account_group_id, account_type, is_reconciliation) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(&input.account_number)
    .bind(&input.name)
    .bind(input.account_group_id)
    .bind(&input.account_type)
    .bind(input.is_reconciliation.unwrap_or(false))
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn update_account(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateAccount,
) -> Result<Account, AppError> {
    let existing = get_account(pool, id).await?;
    let name = input.name.as_deref().unwrap_or(&existing.name);
    let account_group_id = input.account_group_id.or(existing.account_group_id);
    let is_active = input.is_active.unwrap_or(existing.is_active);

    let row = sqlx::query_as::<_, Account>(
        "UPDATE fi_accounts SET name = $2, account_group_id = $3, is_active = $4, updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .bind(name)
    .bind(account_group_id)
    .bind(is_active)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

// --- Account Groups ---
pub async fn list_account_groups(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<AccountGroup>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, AccountGroup>(
        r#"SELECT * FROM fi_account_groups
           WHERE ($1 = false OR (code ILIKE $2 OR name ILIKE $2))
           ORDER BY code
           LIMIT $3 OFFSET $4"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_account_groups(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM fi_account_groups
           WHERE ($1 = false OR (code ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_account_group(
    pool: &PgPool,
    input: &CreateAccountGroup,
) -> Result<AccountGroup, AppError> {
    let row = sqlx::query_as::<_, AccountGroup>(
        "INSERT INTO fi_account_groups (code, name, account_type) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&input.code)
    .bind(&input.name)
    .bind(&input.account_type)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

// --- Company Codes ---
pub async fn list_company_codes(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<CompanyCode>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, CompanyCode>(
        r#"SELECT * FROM fi_company_codes
           WHERE ($1 = false OR (code ILIKE $2 OR name ILIKE $2))
           ORDER BY code
           LIMIT $3 OFFSET $4"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_company_codes(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM fi_company_codes
           WHERE ($1 = false OR (code ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_company_code(
    pool: &PgPool,
    input: &CreateCompanyCode,
) -> Result<CompanyCode, AppError> {
    let row = sqlx::query_as::<_, CompanyCode>(
        "INSERT INTO fi_company_codes (code, name, currency, country) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&input.code)
    .bind(&input.name)
    .bind(input.currency.as_deref().unwrap_or("TWD"))
    .bind(input.country.as_deref().unwrap_or("TW"))
    .fetch_one(pool)
    .await?;
    Ok(row)
}

// --- Fiscal Years ---
pub async fn list_fiscal_years(
    pool: &PgPool,
    company_code_id: Uuid,
) -> Result<Vec<FiscalYear>, AppError> {
    let rows = sqlx::query_as::<_, FiscalYear>(
        "SELECT * FROM fi_fiscal_years WHERE company_code_id = $1 ORDER BY year DESC",
    )
    .bind(company_code_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_fiscal_year(
    pool: &PgPool,
    input: &CreateFiscalYear,
) -> Result<FiscalYear, AppError> {
    let row = sqlx::query_as::<_, FiscalYear>(
        "INSERT INTO fi_fiscal_years (company_code_id, year, start_date, end_date) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(input.company_code_id)
    .bind(input.year)
    .bind(input.start_date)
    .bind(input.end_date)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

// --- Journal Entries ---
pub async fn list_journal_entries(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<JournalEntry>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let rows = sqlx::query_as::<_, JournalEntry>(
        r#"SELECT * FROM fi_journal_entries
           WHERE ($1 = false OR (document_number ILIKE $2 OR COALESCE(description, '') ILIKE $2 OR COALESCE(reference, '') ILIKE $2))
             AND ($3 = false OR status = $4)
             AND ($5 = false OR posting_date >= $6)
             AND ($7 = false OR posting_date <= $8)
           ORDER BY created_at DESC
           LIMIT $9 OFFSET $10"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_journal_entries(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM fi_journal_entries
           WHERE ($1 = false OR (document_number ILIKE $2 OR COALESCE(description, '') ILIKE $2 OR COALESCE(reference, '') ILIKE $2))
             AND ($3 = false OR status = $4)
             AND ($5 = false OR posting_date >= $6)
             AND ($7 = false OR posting_date <= $8)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_journal_entry(pool: &PgPool, id: Uuid) -> Result<JournalEntry, AppError> {
    sqlx::query_as::<_, JournalEntry>("SELECT * FROM fi_journal_entries WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Journal entry not found".to_string()))
}

pub async fn get_journal_items(
    pool: &PgPool,
    journal_entry_id: Uuid,
) -> Result<Vec<JournalItem>, AppError> {
    let rows = sqlx::query_as::<_, JournalItem>(
        "SELECT * FROM fi_journal_items WHERE journal_entry_id = $1 ORDER BY line_number",
    )
    .bind(journal_entry_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_journal_entry(
    pool: &PgPool,
    doc_number: &str,
    input: &CreateJournalEntry,
    user_id: Uuid,
    fiscal_year: i32,
    fiscal_period: i32,
) -> Result<JournalEntry, AppError> {
    let mut tx = pool.begin().await?;

    let entry = sqlx::query_as::<_, JournalEntry>(
        "INSERT INTO fi_journal_entries (document_number, company_code_id, fiscal_year, fiscal_period, posting_date, document_date, reference, description, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(doc_number)
    .bind(input.company_code_id)
    .bind(fiscal_year)
    .bind(fiscal_period)
    .bind(input.posting_date)
    .bind(input.document_date)
    .bind(&input.reference)
    .bind(&input.description)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    for (i, item) in input.items.iter().enumerate() {
        sqlx::query(
            "INSERT INTO fi_journal_items (journal_entry_id, line_number, account_id, debit_amount, credit_amount, cost_center_id, description) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(entry.id)
        .bind((i + 1) as i32)
        .bind(item.account_id)
        .bind(item.debit_amount)
        .bind(item.credit_amount)
        .bind(item.cost_center_id)
        .bind(&item.description)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(entry)
}

pub async fn create_journal_entry_on_conn(
    conn: &mut sqlx::PgConnection,
    doc_number: &str,
    input: &CreateJournalEntry,
    user_id: Uuid,
    fiscal_year: i32,
    fiscal_period: i32,
) -> Result<JournalEntry, AppError> {
    let entry = sqlx::query_as::<_, JournalEntry>(
        "INSERT INTO fi_journal_entries (document_number, company_code_id, fiscal_year, fiscal_period, posting_date, document_date, reference, description, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(doc_number)
    .bind(input.company_code_id)
    .bind(fiscal_year)
    .bind(fiscal_period)
    .bind(input.posting_date)
    .bind(input.document_date)
    .bind(&input.reference)
    .bind(&input.description)
    .bind(user_id)
    .fetch_one(&mut *conn)
    .await?;

    for (i, item) in input.items.iter().enumerate() {
        sqlx::query(
            "INSERT INTO fi_journal_items (journal_entry_id, line_number, account_id, debit_amount, credit_amount, cost_center_id, description) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(entry.id)
        .bind((i + 1) as i32)
        .bind(item.account_id)
        .bind(item.debit_amount)
        .bind(item.credit_amount)
        .bind(item.cost_center_id)
        .bind(&item.description)
        .execute(&mut *conn)
        .await?;
    }

    Ok(entry)
}

pub async fn post_journal_entry(pool: &PgPool, id: Uuid) -> Result<JournalEntry, AppError> {
    sqlx::query_as::<_, JournalEntry>(
        "UPDATE fi_journal_entries SET status = 'POSTED', updated_at = NOW() WHERE id = $1 AND status = 'DRAFT' RETURNING *"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::Validation("Journal entry not found or already posted".to_string()))
}

pub async fn post_journal_entry_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
) -> Result<JournalEntry, AppError> {
    sqlx::query_as::<_, JournalEntry>(
        "UPDATE fi_journal_entries SET status = 'POSTED', updated_at = NOW() WHERE id = $1 AND status = 'DRAFT' RETURNING *"
    )
    .bind(id)
    .fetch_optional(&mut *conn)
    .await?
    .ok_or_else(|| AppError::Validation("Journal entry not found or already posted".to_string()))
}

// --- AR Invoices ---
pub async fn list_ar_invoices(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<ArInvoice>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let rows = sqlx::query_as::<_, ArInvoice>(
        r#"SELECT * FROM fi_ar_invoices
           WHERE ($1 = false OR document_number ILIKE $2)
             AND ($3 = false OR status = $4)
             AND ($5 = false OR invoice_date >= $6)
             AND ($7 = false OR invoice_date <= $8)
           ORDER BY created_at DESC
           LIMIT $9 OFFSET $10"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_ar_invoices(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM fi_ar_invoices
           WHERE ($1 = false OR document_number ILIKE $2)
             AND ($3 = false OR status = $4)
             AND ($5 = false OR invoice_date >= $6)
             AND ($7 = false OR invoice_date <= $8)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_ar_invoice(
    pool: &PgPool,
    doc_number: &str,
    input: &CreateArInvoice,
) -> Result<ArInvoice, AppError> {
    let row = sqlx::query_as::<_, ArInvoice>(
        "INSERT INTO fi_ar_invoices (document_number, customer_id, invoice_date, due_date, total_amount) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(doc_number)
    .bind(input.customer_id)
    .bind(input.invoice_date)
    .bind(input.due_date)
    .bind(input.total_amount)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn create_ar_invoice_on_conn(
    conn: &mut sqlx::PgConnection,
    doc_number: &str,
    input: &CreateArInvoice,
) -> Result<ArInvoice, AppError> {
    let row = sqlx::query_as::<_, ArInvoice>(
        "INSERT INTO fi_ar_invoices (document_number, customer_id, invoice_date, due_date, total_amount) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(doc_number)
    .bind(input.customer_id)
    .bind(input.invoice_date)
    .bind(input.due_date)
    .bind(input.total_amount)
    .fetch_one(&mut *conn)
    .await?;
    Ok(row)
}

// --- AP Invoices ---
pub async fn list_ap_invoices(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<ApInvoice>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let rows = sqlx::query_as::<_, ApInvoice>(
        r#"SELECT * FROM fi_ap_invoices
           WHERE ($1 = false OR document_number ILIKE $2)
             AND ($3 = false OR status = $4)
             AND ($5 = false OR invoice_date >= $6)
             AND ($7 = false OR invoice_date <= $8)
           ORDER BY created_at DESC
           LIMIT $9 OFFSET $10"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_ap_invoices(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM fi_ap_invoices
           WHERE ($1 = false OR document_number ILIKE $2)
             AND ($3 = false OR status = $4)
             AND ($5 = false OR invoice_date >= $6)
             AND ($7 = false OR invoice_date <= $8)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_ap_invoice(
    pool: &PgPool,
    doc_number: &str,
    input: &CreateApInvoice,
) -> Result<ApInvoice, AppError> {
    let row = sqlx::query_as::<_, ApInvoice>(
        "INSERT INTO fi_ap_invoices (document_number, vendor_id, invoice_date, due_date, total_amount) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(doc_number)
    .bind(input.vendor_id)
    .bind(input.invoice_date)
    .bind(input.due_date)
    .bind(input.total_amount)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn create_ap_invoice_on_conn(
    conn: &mut sqlx::PgConnection,
    doc_number: &str,
    input: &CreateApInvoice,
) -> Result<ApInvoice, AppError> {
    let row = sqlx::query_as::<_, ApInvoice>(
        "INSERT INTO fi_ap_invoices (document_number, vendor_id, invoice_date, due_date, total_amount) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(doc_number)
    .bind(input.vendor_id)
    .bind(input.invoice_date)
    .bind(input.due_date)
    .bind(input.total_amount)
    .fetch_one(&mut *conn)
    .await?;
    Ok(row)
}

// --- AR/AP Invoice Lookups (for payment processing) ---
pub async fn get_ar_invoice(pool: &PgPool, id: Uuid) -> Result<ArInvoice, AppError> {
    sqlx::query_as::<_, ArInvoice>("SELECT * FROM fi_ar_invoices WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("AR invoice not found".to_string()))
}

pub async fn get_ap_invoice(pool: &PgPool, id: Uuid) -> Result<ApInvoice, AppError> {
    sqlx::query_as::<_, ApInvoice>("SELECT * FROM fi_ap_invoices WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("AP invoice not found".to_string()))
}

// --- Payment Processing (transactional) ---
pub async fn update_ar_invoice_payment_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
    additional_amount: Decimal,
) -> Result<ArInvoice, AppError> {
    sqlx::query_as::<_, ArInvoice>(
        "UPDATE fi_ar_invoices SET paid_amount = paid_amount + $2, \
         status = CASE WHEN paid_amount + $2 >= total_amount THEN 'PAID' ELSE 'PARTIALLY_PAID' END, \
         updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(additional_amount)
    .fetch_one(&mut *conn)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to update AR invoice payment: {e}")))
}

pub async fn update_ap_invoice_payment_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
    additional_amount: Decimal,
) -> Result<ApInvoice, AppError> {
    sqlx::query_as::<_, ApInvoice>(
        "UPDATE fi_ap_invoices SET paid_amount = paid_amount + $2, \
         status = CASE WHEN paid_amount + $2 >= total_amount THEN 'PAID' ELSE 'PARTIALLY_PAID' END, \
         updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(additional_amount)
    .fetch_one(&mut *conn)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to update AP invoice payment: {e}")))
}

pub async fn create_payment_document_on_conn(
    conn: &mut sqlx::PgConnection,
    doc_number: &str,
    payment_type: &str,
    invoice_id: Uuid,
    amount: Decimal,
    payment_date: NaiveDate,
    journal_entry_id: Option<Uuid>,
    user_id: Uuid,
) -> Result<PaymentDocument, AppError> {
    sqlx::query_as::<_, PaymentDocument>(
        "INSERT INTO fi_payment_documents (document_number, payment_type, invoice_id, amount, payment_date, journal_entry_id, created_by) \
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(doc_number)
    .bind(payment_type)
    .bind(invoice_id)
    .bind(amount)
    .bind(payment_date)
    .bind(journal_entry_id)
    .bind(user_id)
    .fetch_one(&mut *conn)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to create payment document: {e}")))
}

// --- Number Range Helper ---
pub async fn next_number(pool: &PgPool, object_type: &str) -> Result<String, AppError> {
    crate::shared::number_range::next_number(pool, object_type).await
}

pub async fn next_number_on_conn(
    conn: &mut sqlx::PgConnection,
    object_type: &str,
) -> Result<String, AppError> {
    crate::shared::number_range::next_number_on_conn(conn, object_type).await
}
