use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OutputRule {
    pub id: Uuid,
    pub name: String,
    pub operation_id: Option<Uuid>,
    pub trigger_event: String,
    pub condition_field: Option<String>,
    pub condition_operator: Option<String>,
    pub condition_value: Option<String>,
    pub output_type: String,
    pub email_template_code: Option<String>,
    pub print_layout_code: Option<String>,
    pub recipient_type: String,
    pub recipient_field: Option<String>,
    pub recipient_static: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOutputRule {
    pub name: String,
    pub operation_id: Option<Uuid>,
    pub trigger_event: Option<String>,
    pub condition_field: Option<String>,
    pub condition_operator: Option<String>,
    pub condition_value: Option<String>,
    pub output_type: String,
    pub email_template_code: Option<String>,
    pub print_layout_code: Option<String>,
    pub recipient_type: Option<String>,
    pub recipient_field: Option<String>,
    pub recipient_static: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OutputLog {
    pub id: Uuid,
    pub rule_id: Option<Uuid>,
    pub operation_id: Option<Uuid>,
    pub record_id: Option<Uuid>,
    pub output_type: Option<String>,
    pub recipient: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_rules(
    pool: &PgPool,
    operation_id: Option<Uuid>,
) -> Result<Vec<OutputRule>, AppError> {
    match operation_id {
        Some(op_id) => Ok(sqlx::query_as::<_, OutputRule>(
            "SELECT * FROM output_rules WHERE operation_id = $1 ORDER BY sort_order",
        )
        .bind(op_id)
        .fetch_all(pool)
        .await?),
        None => Ok(
            sqlx::query_as::<_, OutputRule>(
                "SELECT * FROM output_rules ORDER BY sort_order",
            )
            .fetch_all(pool)
            .await?,
        ),
    }
}

pub async fn create_rule(pool: &PgPool, input: CreateOutputRule) -> Result<OutputRule, AppError> {
    let rule = sqlx::query_as::<_, OutputRule>(
        "INSERT INTO output_rules (name, operation_id, trigger_event, condition_field, condition_operator, condition_value, output_type, email_template_code, print_layout_code, recipient_type, recipient_field, recipient_static) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12) RETURNING *",
    )
    .bind(&input.name)
    .bind(input.operation_id)
    .bind(input.trigger_event.as_deref().unwrap_or("ON_CREATE"))
    .bind(&input.condition_field)
    .bind(&input.condition_operator)
    .bind(&input.condition_value)
    .bind(&input.output_type)
    .bind(&input.email_template_code)
    .bind(&input.print_layout_code)
    .bind(input.recipient_type.as_deref().unwrap_or("FIELD"))
    .bind(&input.recipient_field)
    .bind(&input.recipient_static)
    .fetch_one(pool)
    .await?;
    Ok(rule)
}

pub async fn delete_rule(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM output_rules WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Evaluate and fire output rules for an event
pub async fn fire_outputs(
    pool: &PgPool,
    operation_id: Uuid,
    record_id: Uuid,
    event: &str,
    data: &serde_json::Value,
) {
    let rules: Vec<OutputRule> = match sqlx::query_as(
        "SELECT * FROM output_rules WHERE operation_id = $1 AND trigger_event = $2 AND is_active = true ORDER BY sort_order",
    )
    .bind(operation_id)
    .bind(event)
    .fetch_all(pool)
    .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("Failed to fetch output rules: {}", e);
            return;
        }
    };

    for rule in rules {
        // Evaluate condition
        if let (Some(ref field), Some(ref op), Some(ref val)) = (
            &rule.condition_field,
            &rule.condition_operator,
            &rule.condition_value,
        ) {
            let actual = data
                .get(field)
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let matches = match op.as_str() {
                "equals" => actual == val,
                "not_equals" => actual != val,
                "contains" => actual.contains(val.as_str()),
                _ => true,
            };
            if !matches {
                continue;
            }
        }

        // Determine recipient
        let recipient = match rule.recipient_type.as_str() {
            "FIELD" => data
                .get(rule.recipient_field.as_deref().unwrap_or(""))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            "STATIC" => rule.recipient_static.clone().unwrap_or_default(),
            _ => continue,
        };

        // Execute output
        match rule.output_type.as_str() {
            "EMAIL" => {
                if let Some(ref template_code) = rule.email_template_code {
                    let _ = crate::shared::email::send_template_email(
                        pool,
                        template_code,
                        &recipient,
                        data,
                    )
                    .await;
                }
            }
            "NOTIFICATION" => {
                let _ = sqlx::query("INSERT INTO lc_notifications (user_id, title, message, notification_type) VALUES ($1, $2, $3, 'OUTPUT')")
                    .bind(uuid::Uuid::parse_str(&recipient).ok())
                    .bind(&rule.name)
                    .bind(format!("Output triggered for record {}", record_id))
                    .execute(pool)
                    .await;
            }
            _ => {}
        }

        // Log output
        let _ = sqlx::query("INSERT INTO output_log (rule_id, operation_id, record_id, output_type, recipient, status) VALUES ($1,$2,$3,$4,$5,'EXECUTED')")
            .bind(rule.id)
            .bind(operation_id)
            .bind(record_id)
            .bind(&rule.output_type)
            .bind(&recipient)
            .execute(pool)
            .await;
    }
}

pub async fn list_logs(
    pool: &PgPool,
    operation_id: Uuid,
) -> Result<Vec<OutputLog>, AppError> {
    Ok(sqlx::query_as::<_, OutputLog>(
        "SELECT * FROM output_log WHERE operation_id = $1 ORDER BY executed_at DESC LIMIT 100",
    )
    .bind(operation_id)
    .fetch_all(pool)
    .await?)
}
