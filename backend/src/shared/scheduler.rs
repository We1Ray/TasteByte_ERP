use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScheduledJob {
    pub id: Uuid,
    pub job_name: String,
    pub job_type: String,
    pub cron_expression: String,
    pub handler: String,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub last_run_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_status: Option<String>,
    pub last_error: Option<String>,
    pub next_run_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduledJob {
    pub job_name: String,
    pub job_type: String,
    pub cron_expression: String,
    pub handler: String,
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct JobExecutionLog {
    pub id: Uuid,
    pub job_id: Uuid,
    pub status: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_ms: Option<i64>,
    pub result: Option<serde_json::Value>,
    pub error_message: Option<String>,
}

pub async fn list_jobs(pool: &PgPool) -> Result<Vec<ScheduledJob>, AppError> {
    let jobs = sqlx::query_as::<_, ScheduledJob>("SELECT * FROM scheduled_jobs ORDER BY job_name")
        .fetch_all(pool)
        .await?;
    Ok(jobs)
}

pub async fn create_job(
    pool: &PgPool,
    input: CreateScheduledJob,
    created_by: Uuid,
) -> Result<ScheduledJob, AppError> {
    let job = sqlx::query_as::<_, ScheduledJob>(
        "INSERT INTO scheduled_jobs (job_name, job_type, cron_expression, handler, config, created_by) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&input.job_name)
    .bind(&input.job_type)
    .bind(&input.cron_expression)
    .bind(&input.handler)
    .bind(input.config.unwrap_or(serde_json::json!({})))
    .bind(created_by)
    .fetch_one(pool)
    .await?;
    Ok(job)
}

pub async fn toggle_job(
    pool: &PgPool,
    job_id: Uuid,
    is_active: bool,
) -> Result<ScheduledJob, AppError> {
    let job = sqlx::query_as::<_, ScheduledJob>(
        "UPDATE scheduled_jobs SET is_active = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(job_id)
    .bind(is_active)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Job not found".to_string()))?;
    Ok(job)
}

pub async fn delete_job(pool: &PgPool, job_id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM scheduled_jobs WHERE id = $1")
        .bind(job_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn log_execution(
    pool: &PgPool,
    job_id: Uuid,
    status: &str,
    started_at: chrono::DateTime<chrono::Utc>,
    duration_ms: Option<i64>,
    result: Option<serde_json::Value>,
    error: Option<&str>,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO job_execution_log (job_id, status, started_at, finished_at, duration_ms, result, error_message) VALUES ($1, $2, $3, NOW(), $4, $5, $6)",
    )
    .bind(job_id)
    .bind(status)
    .bind(started_at)
    .bind(duration_ms)
    .bind(result)
    .bind(error)
    .execute(pool)
    .await?;

    // Update job's last run info
    sqlx::query(
        "UPDATE scheduled_jobs SET last_run_at = $2, last_status = $3, last_error = $4, updated_at = NOW() WHERE id = $1",
    )
    .bind(job_id)
    .bind(started_at)
    .bind(status)
    .bind(error)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_execution_logs(
    pool: &PgPool,
    job_id: Uuid,
    limit: i64,
) -> Result<Vec<JobExecutionLog>, AppError> {
    let logs = sqlx::query_as::<_, JobExecutionLog>(
        "SELECT * FROM job_execution_log WHERE job_id = $1 ORDER BY started_at DESC LIMIT $2",
    )
    .bind(job_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(logs)
}
