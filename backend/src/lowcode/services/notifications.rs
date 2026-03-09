use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

/// Insert a single notification into `lc_notifications`.
pub async fn create_notification(
    pool: &PgPool,
    user_id: Uuid,
    title: &str,
    message: &str,
    notification_type: &str,
    reference_type: Option<&str>,
    reference_id: Option<Uuid>,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO lc_notifications (user_id, title, message, notification_type, reference_type, reference_id) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(user_id)
    .bind(title)
    .bind(message)
    .bind(notification_type)
    .bind(reference_type)
    .bind(reference_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to create notification: {e}")))?;
    Ok(())
}

/// Notify all developers of a project (via operation_id → project_id → lc_project_developers).
pub async fn notify_project_developers(
    pool: &PgPool,
    operation_id: Uuid,
    title: &str,
    message: &str,
    notification_type: &str,
    reference_type: Option<&str>,
    reference_id: Option<Uuid>,
) -> Result<(), AppError> {
    let developer_ids: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT pd.user_id FROM lc_project_developers pd \
         JOIN lc_operations o ON o.project_id = pd.project_id \
         WHERE o.id = $1",
    )
    .bind(operation_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to query project developers: {e}")))?;

    for (uid,) in developer_ids {
        // Best-effort: log but do not fail the parent operation if one notification fails.
        if let Err(e) = create_notification(
            pool,
            uid,
            title,
            message,
            notification_type,
            reference_type,
            reference_id,
        )
        .await
        {
            tracing::warn!("Failed to notify developer {uid}: {e}");
        }
    }
    Ok(())
}

/// Notify all users with the PLATFORM_ADMIN role.
pub async fn notify_platform_admins(
    pool: &PgPool,
    title: &str,
    message: &str,
    notification_type: &str,
    reference_type: Option<&str>,
    reference_id: Option<Uuid>,
) -> Result<(), AppError> {
    let admin_ids: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT upr.user_id FROM lc_user_platform_roles upr \
         JOIN lc_platform_roles r ON r.id = upr.role_id \
         WHERE r.role_name = 'PLATFORM_ADMIN'",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to query platform admins: {e}")))?;

    for (uid,) in admin_ids {
        if let Err(e) = create_notification(
            pool,
            uid,
            title,
            message,
            notification_type,
            reference_type,
            reference_id,
        )
        .await
        {
            tracing::warn!("Failed to notify admin {uid}: {e}");
        }
    }
    Ok(())
}
