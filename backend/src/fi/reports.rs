use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::rbac::{FiRead, RequireRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

// --- Trial Balance ---
#[derive(Serialize, sqlx::FromRow)]
pub struct TrialBalanceRow {
    pub account_id: Uuid,
    pub account_number: String,
    pub account_name: String,
    pub account_type: String,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub balance: Decimal,
}

#[derive(Deserialize)]
pub struct DateRangeQuery {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

pub async fn trial_balance(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<Vec<TrialBalanceRow>>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let rows = sqlx::query_as::<_, TrialBalanceRow>(
        "SELECT a.id AS account_id, a.account_number, a.name AS account_name, a.account_type, \
         COALESCE(SUM(ji.debit_amount), 0) AS total_debit, \
         COALESCE(SUM(ji.credit_amount), 0) AS total_credit, \
         COALESCE(SUM(ji.debit_amount), 0) - COALESCE(SUM(ji.credit_amount), 0) AS balance \
         FROM fi_accounts a \
         LEFT JOIN fi_journal_items ji ON ji.account_id = a.id \
         LEFT JOIN fi_journal_entries je ON je.id = ji.journal_entry_id \
             AND je.status = 'POSTED' AND je.posting_date BETWEEN $1 AND $2 \
         WHERE a.is_active = true \
         GROUP BY a.id, a.account_number, a.name, a.account_type \
         HAVING COALESCE(SUM(ji.debit_amount), 0) != 0 \
             OR COALESCE(SUM(ji.credit_amount), 0) != 0 \
         ORDER BY a.account_number",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}

// --- Income Statement ---
#[derive(Serialize)]
pub struct IncomeStatement {
    pub total_revenue: Decimal,
    pub total_expenses: Decimal,
    pub net_income: Decimal,
    pub revenue_items: Vec<AccountBalance>,
    pub expense_items: Vec<AccountBalance>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AccountBalance {
    pub account_number: String,
    pub account_name: String,
    pub balance: Decimal,
}

pub async fn income_statement(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<IncomeStatement>>, AppError> {
    let from = query
        .from_date
        .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let revenue_items = sqlx::query_as::<_, AccountBalance>(
        "SELECT a.account_number, a.name AS account_name, \
         COALESCE(SUM(ji.credit_amount), 0) - COALESCE(SUM(ji.debit_amount), 0) AS balance \
         FROM fi_accounts a \
         JOIN fi_journal_items ji ON ji.account_id = a.id \
         JOIN fi_journal_entries je ON je.id = ji.journal_entry_id \
             AND je.status = 'POSTED' AND je.posting_date BETWEEN $1 AND $2 \
         WHERE a.account_type = 'REVENUE' AND a.is_active = true \
         GROUP BY a.account_number, a.name \
         ORDER BY a.account_number",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    let expense_items = sqlx::query_as::<_, AccountBalance>(
        "SELECT a.account_number, a.name AS account_name, \
         COALESCE(SUM(ji.debit_amount), 0) - COALESCE(SUM(ji.credit_amount), 0) AS balance \
         FROM fi_accounts a \
         JOIN fi_journal_items ji ON ji.account_id = a.id \
         JOIN fi_journal_entries je ON je.id = ji.journal_entry_id \
             AND je.status = 'POSTED' AND je.posting_date BETWEEN $1 AND $2 \
         WHERE a.account_type = 'EXPENSE' AND a.is_active = true \
         GROUP BY a.account_number, a.name \
         ORDER BY a.account_number",
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await?;

    let total_revenue: Decimal = revenue_items.iter().map(|r| r.balance).sum();
    let total_expenses: Decimal = expense_items.iter().map(|r| r.balance).sum();

    Ok(Json(ApiResponse::success(IncomeStatement {
        total_revenue,
        total_expenses,
        net_income: total_revenue - total_expenses,
        revenue_items,
        expense_items,
    })))
}

// --- Balance Sheet ---
#[derive(Serialize)]
pub struct BalanceSheet {
    pub total_assets: Decimal,
    pub total_liabilities: Decimal,
    pub total_equity: Decimal,
    pub assets: Vec<AccountBalance>,
    pub liabilities: Vec<AccountBalance>,
    pub equity: Vec<AccountBalance>,
}

pub async fn balance_sheet(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<BalanceSheet>>, AppError> {
    let to = query
        .to_date
        .unwrap_or(NaiveDate::from_ymd_opt(2099, 12, 31).unwrap());

    let fetch_balances = |account_type: &'static str, debit_positive: bool| {
        let pool = state.pool.clone();
        async move {
            sqlx::query_as::<_, AccountBalance>(&format!(
                "SELECT a.account_number, a.name AS account_name, \
                     {} AS balance \
                     FROM fi_accounts a \
                     JOIN fi_journal_items ji ON ji.account_id = a.id \
                     JOIN fi_journal_entries je ON je.id = ji.journal_entry_id \
                         AND je.status = 'POSTED' AND je.posting_date <= $1 \
                     WHERE a.account_type = '{}' AND a.is_active = true \
                     GROUP BY a.account_number, a.name \
                     ORDER BY a.account_number",
                if debit_positive {
                    "COALESCE(SUM(ji.debit_amount), 0) - COALESCE(SUM(ji.credit_amount), 0)"
                } else {
                    "COALESCE(SUM(ji.credit_amount), 0) - COALESCE(SUM(ji.debit_amount), 0)"
                },
                account_type
            ))
            .bind(to)
            .fetch_all(&pool)
            .await
        }
    };

    let assets: Vec<AccountBalance> = fetch_balances("ASSET", true).await?;
    let liabilities: Vec<AccountBalance> = fetch_balances("LIABILITY", false).await?;
    let equity: Vec<AccountBalance> = fetch_balances("EQUITY", false).await?;

    let total_assets: Decimal = assets.iter().map(|r| r.balance).sum();
    let total_liabilities: Decimal = liabilities.iter().map(|r| r.balance).sum();
    let total_equity: Decimal = equity.iter().map(|r| r.balance).sum();

    Ok(Json(ApiResponse::success(BalanceSheet {
        total_assets,
        total_liabilities,
        total_equity,
        assets,
        liabilities,
        equity,
    })))
}

// --- AR Aging ---
#[derive(Serialize, sqlx::FromRow)]
pub struct AgingRow {
    pub customer_id: Option<Uuid>,
    pub total_amount: Decimal,
    pub paid_amount: Decimal,
    pub outstanding: Decimal,
    pub current_amount: Decimal,
    pub days_30: Decimal,
    pub days_60: Decimal,
    pub days_90: Decimal,
    pub over_90: Decimal,
}

pub async fn ar_aging(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
) -> Result<Json<ApiResponse<Vec<AgingRow>>>, AppError> {
    let rows = sqlx::query_as::<_, AgingRow>(
        "SELECT customer_id, \
         SUM(total_amount) AS total_amount, SUM(paid_amount) AS paid_amount, \
         SUM(total_amount - paid_amount) AS outstanding, \
         SUM(CASE WHEN CURRENT_DATE - due_date <= 0 THEN total_amount - paid_amount ELSE 0 END) AS current_amount, \
         SUM(CASE WHEN CURRENT_DATE - due_date BETWEEN 1 AND 30 THEN total_amount - paid_amount ELSE 0 END) AS days_30, \
         SUM(CASE WHEN CURRENT_DATE - due_date BETWEEN 31 AND 60 THEN total_amount - paid_amount ELSE 0 END) AS days_60, \
         SUM(CASE WHEN CURRENT_DATE - due_date BETWEEN 61 AND 90 THEN total_amount - paid_amount ELSE 0 END) AS days_90, \
         SUM(CASE WHEN CURRENT_DATE - due_date > 90 THEN total_amount - paid_amount ELSE 0 END) AS over_90 \
         FROM fi_ar_invoices WHERE status != 'CANCELLED' \
         GROUP BY customer_id ORDER BY outstanding DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}

// --- AP Aging ---
#[derive(Serialize, sqlx::FromRow)]
pub struct ApAgingRow {
    pub vendor_id: Option<Uuid>,
    pub total_amount: Decimal,
    pub paid_amount: Decimal,
    pub outstanding: Decimal,
    pub current_amount: Decimal,
    pub days_30: Decimal,
    pub days_60: Decimal,
    pub days_90: Decimal,
    pub over_90: Decimal,
}

pub async fn ap_aging(
    State(state): State<AppState>,
    _role: RequireRole<FiRead>,
) -> Result<Json<ApiResponse<Vec<ApAgingRow>>>, AppError> {
    let rows = sqlx::query_as::<_, ApAgingRow>(
        "SELECT vendor_id, \
         SUM(total_amount) AS total_amount, SUM(paid_amount) AS paid_amount, \
         SUM(total_amount - paid_amount) AS outstanding, \
         SUM(CASE WHEN CURRENT_DATE - due_date <= 0 THEN total_amount - paid_amount ELSE 0 END) AS current_amount, \
         SUM(CASE WHEN CURRENT_DATE - due_date BETWEEN 1 AND 30 THEN total_amount - paid_amount ELSE 0 END) AS days_30, \
         SUM(CASE WHEN CURRENT_DATE - due_date BETWEEN 31 AND 60 THEN total_amount - paid_amount ELSE 0 END) AS days_60, \
         SUM(CASE WHEN CURRENT_DATE - due_date BETWEEN 61 AND 90 THEN total_amount - paid_amount ELSE 0 END) AS days_90, \
         SUM(CASE WHEN CURRENT_DATE - due_date > 90 THEN total_amount - paid_amount ELSE 0 END) AS over_90 \
         FROM fi_ap_invoices WHERE status != 'CANCELLED' \
         GROUP BY vendor_id ORDER BY outstanding DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(rows)))
}
