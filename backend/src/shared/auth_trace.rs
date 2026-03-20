use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AuthTraceEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub action: String,
    pub result: String,
    pub reason: Option<String>,
    pub checked_roles: Option<Vec<String>>,
    pub checked_permissions: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code, clippy::too_many_arguments)]
pub async fn log_auth_check(
    pool: &PgPool,
    user_id: Uuid,
    resource_type: &str,
    resource_id: Option<Uuid>,
    action: &str,
    result: &str,
    reason: Option<&str>,
    checked_roles: Option<&[String]>,
    checked_permissions: Option<&serde_json::Value>,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO auth_trace_log (user_id, resource_type, resource_id, action, result, reason, checked_roles, checked_permissions) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
    )
    .bind(user_id)
    .bind(resource_type)
    .bind(resource_id)
    .bind(action)
    .bind(result)
    .bind(reason)
    .bind(checked_roles.map(|r| r.to_vec()))
    .bind(checked_permissions)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_user_trace(
    pool: &PgPool,
    user_id: Uuid,
    limit: i64,
) -> Result<Vec<AuthTraceEntry>, AppError> {
    Ok(sqlx::query_as::<_, AuthTraceEntry>(
        "SELECT * FROM auth_trace_log WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2",
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?)
}

pub async fn get_recent_denials(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<AuthTraceEntry>, AppError> {
    Ok(sqlx::query_as::<_, AuthTraceEntry>(
        "SELECT * FROM auth_trace_log WHERE user_id = $1 AND result = 'DENIED' ORDER BY created_at DESC LIMIT 20",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}
