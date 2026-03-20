use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::shared::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReportDefinition {
    pub id: Uuid,
    pub report_code: String,
    pub name: String,
    pub description: Option<String>,
    pub operation_id: Option<Uuid>,
    pub data_source_sql: String,
    pub columns: serde_json::Value,
    pub filters: serde_json::Value,
    pub grouping: Option<serde_json::Value>,
    pub chart_config: Option<serde_json::Value>,
    pub default_sort: Option<String>,
    pub default_sort_dir: String,
    pub page_size: i32,
    pub is_public: bool,
    pub created_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReport {
    pub report_code: String,
    pub name: String,
    pub description: Option<String>,
    pub operation_id: Option<Uuid>,
    pub data_source_sql: String,
    pub columns: serde_json::Value,
    pub filters: Option<serde_json::Value>,
    pub chart_config: Option<serde_json::Value>,
    pub default_sort: Option<String>,
    pub page_size: Option<i32>,
}

pub async fn list_reports(pool: &PgPool) -> Result<Vec<ReportDefinition>, AppError> {
    Ok(sqlx::query_as::<_, ReportDefinition>("SELECT * FROM report_definitions ORDER BY name")
        .fetch_all(pool).await?)
}

pub async fn get_report(pool: &PgPool, id: Uuid) -> Result<ReportDefinition, AppError> {
    sqlx::query_as::<_, ReportDefinition>("SELECT * FROM report_definitions WHERE id = $1")
        .bind(id).fetch_optional(pool).await?
        .ok_or_else(|| AppError::NotFound("Report not found".into()))
}

pub async fn create_report(pool: &PgPool, input: CreateReport, created_by: Uuid) -> Result<ReportDefinition, AppError> {
    Ok(sqlx::query_as::<_, ReportDefinition>(
        "INSERT INTO report_definitions (report_code,name,description,operation_id,data_source_sql,columns,filters,chart_config,default_sort,page_size,created_by) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) RETURNING *"
    ).bind(&input.report_code).bind(&input.name).bind(&input.description).bind(input.operation_id)
    .bind(&input.data_source_sql).bind(&input.columns).bind(input.filters.unwrap_or(serde_json::json!([])))
    .bind(&input.chart_config).bind(&input.default_sort).bind(input.page_size.unwrap_or(50))
    .bind(created_by).fetch_one(pool).await?)
}

pub async fn delete_report(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM report_definitions WHERE id=$1").bind(id).execute(pool).await?;
    Ok(())
}
