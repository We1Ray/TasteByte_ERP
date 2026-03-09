use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub module: Option<String>,
    pub reference_id: Option<Uuid>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNotification {
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: Option<String>,
    pub module: Option<String>,
    pub reference_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct NotificationListParams {
    pub page: Option<i64>,
    #[serde(alias = "page_size")]
    pub per_page: Option<i64>,
    pub is_read: Option<bool>,
    pub notification_type: Option<String>,
    pub module: Option<String>,
}

impl NotificationListParams {
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page();
        (page - 1) * per_page
    }

    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(20).clamp(1, 100)
    }

    pub fn to_pagination(&self) -> crate::shared::PaginationParams {
        crate::shared::PaginationParams {
            page: self.page,
            per_page: self.per_page,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UnreadCount {
    pub count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_list_params_defaults() {
        let params = NotificationListParams {
            page: None,
            per_page: None,
            is_read: None,
            notification_type: None,
            module: None,
        };
        assert_eq!(params.per_page(), 20);
        assert_eq!(params.offset(), 0);
    }

    #[test]
    fn notification_list_params_offset() {
        let params = NotificationListParams {
            page: Some(3),
            per_page: Some(10),
            is_read: None,
            notification_type: None,
            module: None,
        };
        assert_eq!(params.offset(), 20);
    }

    #[test]
    fn notification_list_params_clamp() {
        let params = NotificationListParams {
            page: Some(1),
            per_page: Some(500),
            is_read: None,
            notification_type: None,
            module: None,
        };
        assert_eq!(params.per_page(), 100);
    }
}
