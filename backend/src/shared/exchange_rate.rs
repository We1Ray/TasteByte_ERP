use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::shared::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExchangeRate {
    pub id: Uuid,
    pub from_currency: String,
    pub to_currency: String,
    pub rate: rust_decimal::Decimal,
    pub valid_from: chrono::NaiveDate,
    pub valid_until: Option<chrono::NaiveDate>,
    pub source: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateExchangeRate {
    pub from_currency: String,
    pub to_currency: String,
    pub rate: rust_decimal::Decimal,
    pub valid_from: chrono::NaiveDate,
}

pub async fn list_rates(pool: &PgPool) -> Result<Vec<ExchangeRate>, AppError> {
    Ok(sqlx::query_as::<_, ExchangeRate>("SELECT * FROM exchange_rates ORDER BY from_currency, to_currency, valid_from DESC")
        .fetch_all(pool).await?)
}

pub async fn create_rate(pool: &PgPool, input: CreateExchangeRate) -> Result<ExchangeRate, AppError> {
    Ok(sqlx::query_as::<_, ExchangeRate>(
        "INSERT INTO exchange_rates (from_currency,to_currency,rate,valid_from) VALUES ($1,$2,$3,$4) RETURNING *"
    ).bind(&input.from_currency).bind(&input.to_currency).bind(input.rate).bind(input.valid_from)
    .fetch_one(pool).await?)
}

pub async fn delete_rate(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM exchange_rates WHERE id=$1").bind(id).execute(pool).await?;
    Ok(())
}

pub async fn convert(pool: &PgPool, from: &str, to: &str, amount: rust_decimal::Decimal, date: chrono::NaiveDate) -> Result<rust_decimal::Decimal, AppError> {
    if from == to { return Ok(amount); }
    let rate = sqlx::query_as::<_, ExchangeRate>(
        "SELECT * FROM exchange_rates WHERE from_currency=$1 AND to_currency=$2 AND valid_from<=$3 ORDER BY valid_from DESC LIMIT 1"
    ).bind(from).bind(to).bind(date).fetch_optional(pool).await?
    .ok_or_else(|| AppError::NotFound(format!("No rate for {}/{}", from, to)))?;
    Ok(amount * rate.rate)
}
