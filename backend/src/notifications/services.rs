use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::{AppError, PaginatedResponse};

use super::models::{CreateNotification, Notification, NotificationListParams};

/// Create a new notification for a user.
pub async fn create_notification(
    pool: &PgPool,
    input: CreateNotification,
) -> Result<Notification, AppError> {
    let notification_type = input.notification_type.as_deref().unwrap_or("info");
    let notification = sqlx::query_as::<_, Notification>(
        "INSERT INTO notifications (user_id, title, message, notification_type, module, reference_id) \
         VALUES ($1, $2, $3, $4, $5, $6) \
         RETURNING *",
    )
    .bind(input.user_id)
    .bind(&input.title)
    .bind(&input.message)
    .bind(notification_type)
    .bind(&input.module)
    .bind(input.reference_id)
    .fetch_one(pool)
    .await?;

    Ok(notification)
}

/// Get paginated notifications for a user, with optional filters.
pub async fn get_notifications(
    pool: &PgPool,
    user_id: Uuid,
    params: &NotificationListParams,
) -> Result<PaginatedResponse<Notification>, AppError> {
    let per_page = params.per_page();
    let offset = params.offset();

    let (total,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM notifications \
         WHERE user_id = $1 \
           AND ($2::BOOLEAN IS NULL OR is_read = $2) \
           AND ($3::TEXT IS NULL OR notification_type = $3) \
           AND ($4::TEXT IS NULL OR module = $4)",
    )
    .bind(user_id)
    .bind(params.is_read)
    .bind(&params.notification_type)
    .bind(&params.module)
    .fetch_one(pool)
    .await?;

    let items = sqlx::query_as::<_, Notification>(
        "SELECT * FROM notifications \
         WHERE user_id = $1 \
           AND ($2::BOOLEAN IS NULL OR is_read = $2) \
           AND ($3::TEXT IS NULL OR notification_type = $3) \
           AND ($4::TEXT IS NULL OR module = $4) \
         ORDER BY created_at DESC \
         LIMIT $5 OFFSET $6",
    )
    .bind(user_id)
    .bind(params.is_read)
    .bind(&params.notification_type)
    .bind(&params.module)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(PaginatedResponse::new(
        items,
        total,
        &params.to_pagination(),
    ))
}

/// Get the count of unread notifications for a user.
pub async fn get_unread_count(pool: &PgPool, user_id: Uuid) -> Result<i64, AppError> {
    let (count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND is_read = FALSE")
            .bind(user_id)
            .fetch_one(pool)
            .await?;

    Ok(count)
}

/// Mark a single notification as read. Returns true if updated.
pub async fn mark_as_read(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
    let result = sqlx::query(
        "UPDATE notifications SET is_read = TRUE WHERE id = $1 AND user_id = $2 AND is_read = FALSE",
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Mark all notifications as read for a user.
pub async fn mark_all_as_read(pool: &PgPool, user_id: Uuid) -> Result<i64, AppError> {
    let result = sqlx::query(
        "UPDATE notifications SET is_read = TRUE WHERE user_id = $1 AND is_read = FALSE",
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}

/// Delete a single notification. Returns true if deleted.
pub async fn delete_notification(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
    let result = sqlx::query("DELETE FROM notifications WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Helper: create a notification (fire-and-forget style, logs errors).
/// Used by other modules when they want to notify a user without failing the main operation.
pub async fn notify(
    pool: &PgPool,
    user_id: Uuid,
    title: &str,
    message: &str,
    notification_type: &str,
    module: Option<&str>,
    reference_id: Option<Uuid>,
) {
    let input = CreateNotification {
        user_id,
        title: title.to_string(),
        message: message.to_string(),
        notification_type: Some(notification_type.to_string()),
        module: module.map(|s| s.to_string()),
        reference_id,
    };
    if let Err(e) = create_notification(pool, input).await {
        tracing::error!("Failed to create notification: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_notification_input_defaults() {
        let input = CreateNotification {
            user_id: Uuid::new_v4(),
            title: "Test".to_string(),
            message: "Test message".to_string(),
            notification_type: None,
            module: None,
            reference_id: None,
        };
        assert!(input.notification_type.is_none());
        assert!(input.module.is_none());
        assert!(input.reference_id.is_none());
    }
}
