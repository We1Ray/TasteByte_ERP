use uuid::Uuid;

use crate::shared::AppError;

/// Log a data change to the audit_log table.
pub async fn log_change(
    executor: impl sqlx::PgExecutor<'_>,
    table_name: &str,
    record_id: Uuid,
    action: &str,
    old_values: Option<serde_json::Value>,
    new_values: Option<serde_json::Value>,
    changed_by: Option<Uuid>,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO audit_log (table_name, record_id, action, old_values, new_values, changed_by) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(table_name)
    .bind(record_id)
    .bind(action)
    .bind(old_values)
    .bind(new_values)
    .bind(changed_by)
    .execute(executor)
    .await?;
    Ok(())
}
