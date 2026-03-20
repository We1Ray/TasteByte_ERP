use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Webhook {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub secret: Option<String>,
    pub events: Vec<String>,
    pub headers: serde_json::Value,
    pub is_active: bool,
    pub retry_count: i32,
    pub created_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWebhook {
    pub name: String,
    pub url: String,
    pub secret: Option<String>,
    pub events: Vec<String>,
    pub headers: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct WebhookDeliveryLog {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub attempt: i32,
    pub delivered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_webhooks(pool: &PgPool) -> Result<Vec<Webhook>, AppError> {
    let hooks = sqlx::query_as::<_, Webhook>("SELECT * FROM webhooks ORDER BY name")
        .fetch_all(pool)
        .await?;
    Ok(hooks)
}

pub async fn create_webhook(
    pool: &PgPool,
    input: CreateWebhook,
    created_by: Uuid,
) -> Result<Webhook, AppError> {
    let hook = sqlx::query_as::<_, Webhook>(
        "INSERT INTO webhooks (name, url, secret, events, headers, created_by) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&input.name)
    .bind(&input.url)
    .bind(&input.secret)
    .bind(&input.events)
    .bind(input.headers.unwrap_or(serde_json::json!({})))
    .bind(created_by)
    .fetch_one(pool)
    .await?;
    Ok(hook)
}

pub async fn delete_webhook(pool: &PgPool, webhook_id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM webhooks WHERE id = $1")
        .bind(webhook_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Fire a webhook event to all matching webhooks
pub async fn fire_event(pool: &PgPool, event_type: &str, payload: &serde_json::Value) {
    let hooks: Vec<Webhook> = match sqlx::query_as(
        "SELECT * FROM webhooks WHERE is_active = true AND $1 = ANY(events)",
    )
    .bind(event_type)
    .fetch_all(pool)
    .await
    {
        Ok(h) => h,
        Err(e) => {
            tracing::warn!("Failed to fetch webhooks for event {}: {}", event_type, e);
            return;
        }
    };

    for hook in hooks {
        let url = hook.url.clone();
        let payload = payload.clone();
        let pool = pool.clone();
        let hook_id = hook.id;
        let event = event_type.to_string();

        // Spawn async delivery
        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let result = client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("X-Event-Type", &event)
                .json(&payload)
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .await;

            let (status, body) = match result {
                Ok(resp) => {
                    let s = resp.status().as_u16() as i32;
                    let b = resp.text().await.unwrap_or_default();
                    (Some(s), Some(b))
                }
                Err(e) => {
                    tracing::warn!("Webhook delivery failed to {}: {}", url, e);
                    (None, Some(e.to_string()))
                }
            };

            let _ = sqlx::query(
                "INSERT INTO webhook_delivery_log (webhook_id, event_type, payload, response_status, response_body, delivered_at) VALUES ($1, $2, $3, $4, $5, NOW())",
            )
            .bind(hook_id)
            .bind(&event)
            .bind(&payload)
            .bind(status)
            .bind(body)
            .execute(&pool)
            .await;
        });
    }
}
