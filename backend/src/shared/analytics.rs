use crate::shared::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct TrackEvent {
    pub event_type: String,
    pub operation_id: Option<Uuid>,
    pub event_data: Option<serde_json::Value>,
    pub page_url: Option<String>,
    pub duration_ms: Option<i32>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UsageSummary {
    pub event_type: String,
    pub count: i64,
    pub unique_users: i64,
}

pub async fn track_event(pool: &PgPool, user_id: Uuid, input: TrackEvent) -> Result<(), AppError> {
    sqlx::query("INSERT INTO usage_analytics (user_id,operation_id,event_type,event_data,page_url,duration_ms) VALUES ($1,$2,$3,$4,$5,$6)")
        .bind(user_id).bind(input.operation_id).bind(&input.event_type)
        .bind(input.event_data.unwrap_or(serde_json::json!({}))).bind(&input.page_url).bind(input.duration_ms)
        .execute(pool).await?;
    Ok(())
}

pub async fn get_summary(pool: &PgPool, days: i32) -> Result<Vec<UsageSummary>, AppError> {
    Ok(sqlx::query_as::<_, UsageSummary>(
        "SELECT event_type, COUNT(*) as count, COUNT(DISTINCT user_id) as unique_users FROM usage_analytics WHERE created_at > NOW() - make_interval(days => $1) GROUP BY event_type ORDER BY count DESC"
    ).bind(days).fetch_all(pool).await?)
}

pub async fn get_operation_stats(
    pool: &PgPool,
    operation_id: Uuid,
) -> Result<Vec<UsageSummary>, AppError> {
    Ok(sqlx::query_as::<_, UsageSummary>(
        "SELECT event_type, COUNT(*) as count, COUNT(DISTINCT user_id) as unique_users FROM usage_analytics WHERE operation_id = $1 GROUP BY event_type ORDER BY count DESC"
    ).bind(operation_id).fetch_all(pool).await?)
}
