use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EmailTemplate {
    pub id: Uuid,
    pub template_code: String,
    pub subject: String,
    pub body_html: String,
    pub body_text: Option<String>,
    pub variables: serde_json::Value,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EmailLog {
    pub id: Uuid,
    pub template_code: Option<String>,
    pub recipient: String,
    pub subject: String,
    pub status: String,
    pub error_message: Option<String>,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Send an email using a template
pub async fn send_template_email(
    pool: &PgPool,
    template_code: &str,
    recipient: &str,
    variables: &serde_json::Value,
) -> Result<Uuid, AppError> {
    let template = sqlx::query_as::<_, EmailTemplate>(
        "SELECT * FROM email_templates WHERE template_code = $1 AND is_active = true",
    )
    .bind(template_code)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Email template '{}' not found", template_code)))?;

    // Simple variable substitution using {{ var }}
    let mut subject = template.subject.clone();
    let mut body = template.body_html.clone();

    if let serde_json::Value::Object(vars) = variables {
        for (key, val) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = match val {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            subject = subject.replace(&placeholder, &value_str);
            body = body.replace(&placeholder, &value_str);
        }
    }

    // Log the email (actual sending would use lettre SMTP in production)
    let log_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO email_log (template_code, recipient, subject, status) VALUES ($1, $2, $3, 'QUEUED') RETURNING id",
    )
    .bind(template_code)
    .bind(recipient)
    .bind(&subject)
    .fetch_one(pool)
    .await?;

    // In production, spawn async task to actually send via SMTP
    // For now, mark as sent (SMTP config would come from env vars)
    let smtp_host = std::env::var("SMTP_HOST").ok();
    if smtp_host.is_some() {
        // Would use lettre here in production
        sqlx::query("UPDATE email_log SET status = 'SENT', sent_at = NOW() WHERE id = $1")
            .bind(log_id)
            .execute(pool)
            .await?;
    } else {
        // No SMTP configured - mark as logged only
        sqlx::query("UPDATE email_log SET status = 'LOGGED' WHERE id = $1")
            .bind(log_id)
            .execute(pool)
            .await?;
        tracing::info!(
            "Email queued (no SMTP): to={}, subject={}",
            recipient,
            subject
        );
    }

    Ok(log_id)
}

/// List email templates
pub async fn list_templates(pool: &PgPool) -> Result<Vec<EmailTemplate>, AppError> {
    let templates = sqlx::query_as::<_, EmailTemplate>(
        "SELECT * FROM email_templates ORDER BY template_code",
    )
    .fetch_all(pool)
    .await?;
    Ok(templates)
}

/// List email logs
pub async fn list_email_logs(pool: &PgPool, limit: i64) -> Result<Vec<EmailLog>, AppError> {
    let logs = sqlx::query_as::<_, EmailLog>(
        "SELECT * FROM email_log ORDER BY created_at DESC LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(logs)
}
