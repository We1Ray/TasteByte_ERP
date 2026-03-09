# Rust 錯誤處理指南

## 目錄
1. [自訂錯誤類型](#自訂錯誤類型)
2. [Axum 錯誤回應](#axum-錯誤回應)
3. [錯誤轉換](#錯誤轉換)
4. [錯誤日誌](#錯誤日誌)

## 自訂錯誤類型

### 使用 thiserror
```rust
use thiserror::Error;
use axum::http::StatusCode;

#[derive(Debug, Error)]
pub enum AppError {
    // 認證錯誤
    #[error("未授權：{0}")]
    Unauthorized(String),

    #[error("禁止存取：{0}")]
    Forbidden(String),

    // 資源錯誤
    #[error("找不到資源")]
    NotFound,

    #[error("資源已存在：{0}")]
    Conflict(String),

    // 驗證錯誤
    #[error("驗證失敗：{0}")]
    Validation(String),

    #[error("請求格式錯誤：{0}")]
    BadRequest(String),

    // 內部錯誤
    #[error("資料庫錯誤：{0}")]
    Database(#[from] sqlx::Error),

    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    #[error("JSON 序列化錯誤：{0}")]
    Serialization(#[from] serde_json::Error),

    #[error("JWT 錯誤：{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("內部錯誤：{0}")]
    Internal(String),

    // Rate Limiting
    #[error("請求過於頻繁")]
    TooManyRequests,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Validation(_) | Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::Database(_) | Self::Redis(_) | Self::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::HttpRequest(_) => StatusCode::BAD_GATEWAY,
            Self::Serialization(_) | Self::Jwt(_) => StatusCode::BAD_REQUEST,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::NotFound => "NOT_FOUND",
            Self::Conflict(_) => "CONFLICT",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::TooManyRequests => "TOO_MANY_REQUESTS",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Redis(_) => "CACHE_ERROR",
            Self::HttpRequest(_) => "EXTERNAL_SERVICE_ERROR",
            Self::Serialization(_) => "SERIALIZATION_ERROR",
            Self::Jwt(_) => "JWT_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }
}
```

## Axum 錯誤回應

### IntoResponse 實作
```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_code = self.error_code();
        let message = self.to_string();

        // 記錄錯誤
        if status.is_server_error() {
            tracing::error!(
                error_code = %error_code,
                message = %message,
                "Server error occurred"
            );
        } else {
            tracing::warn!(
                error_code = %error_code,
                message = %message,
                "Client error occurred"
            );
        }

        let body = Json(json!({
            "success": false,
            "data": null,
            "error": {
                "code": error_code,
                "message": if status.is_server_error() {
                    "內部伺服器錯誤".to_string()
                } else {
                    message
                }
            }
        }));

        (status, body).into_response()
    }
}
```

### 帶詳細資訊的錯誤
```rust
#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
    pub fields: HashMap<String, Vec<String>>,
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "success": false,
            "data": null,
            "error": {
                "code": "VALIDATION_ERROR",
                "message": self.message,
                "details": {
                    "fields": self.fields
                }
            }
        }));

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
```

## 錯誤轉換

### From Trait 實作
```rust
impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errs)| {
                errs.iter().map(move |e| {
                    format!("{}: {}", field, e.message.as_ref().unwrap_or(&std::borrow::Cow::Borrowed("invalid")))
                })
            })
            .collect();

        AppError::Validation(messages.join(", "))
    }
}

// 從 sqlx 錯誤轉換（已透過 #[from] 自動實作）
// 但可以自訂更細緻的轉換
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(db_err) => {
                // PostgreSQL unique violation
                if db_err.code().map(|c| c == "23505").unwrap_or(false) {
                    return AppError::Conflict("資源已存在".to_string());
                }
                // PostgreSQL foreign key violation
                if db_err.code().map(|c| c == "23503").unwrap_or(false) {
                    return AppError::BadRequest("關聯資源不存在".to_string());
                }
                AppError::Database(err)
            }
            _ => AppError::Database(err),
        }
    }
}
```

### Result 擴展
```rust
pub trait ResultExt<T> {
    fn not_found(self) -> Result<T, AppError>;
    fn bad_request(self, msg: &str) -> Result<T, AppError>;
}

impl<T> ResultExt<T> for Option<T> {
    fn not_found(self) -> Result<T, AppError> {
        self.ok_or(AppError::NotFound)
    }

    fn bad_request(self, msg: &str) -> Result<T, AppError> {
        self.ok_or_else(|| AppError::BadRequest(msg.to_string()))
    }
}

// 使用範例
async fn get_sales_order(id: Uuid) -> Result<SalesOrder, AppError> {
    let order = repository.find_by_id(id).await?.not_found()?;
    Ok(order)
}
```

## 錯誤日誌

### 結構化錯誤日誌
```rust
use tracing::{error, warn, info, instrument};

#[instrument(skip(pool), err)]
pub async fn create_sales_order(
    pool: &PgPool,
    user_id: Uuid,
    data: CreateSalesOrderRequest,
) -> Result<SalesOrder, AppError> {
    info!(user_id = %user_id, "Creating new sales order");

    let order = sqlx::query_as::<_, SalesOrder>(...)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!(
                error = %e,
                user_id = %user_id,
                "Failed to create sales order"
            );
            AppError::Database(e)
        })?;

    info!(
        order_id = %order.id,
        user_id = %user_id,
        "Sales order created successfully"
    );

    Ok(order)
}
```

### 錯誤上下文
```rust
use anyhow::Context;

pub async fn process_order(id: Uuid) -> Result<(), anyhow::Error> {
    let order = fetch_order(id)
        .await
        .context("Failed to fetch order")?;

    let items = fetch_order_items(id)
        .await
        .context("Failed to fetch order items")?;

    validate_inventory(&order, &items)
        .context("Failed to validate inventory availability")?;

    Ok(())
}
```

### Panic 處理
```rust
use std::panic;
use tracing::error;

pub fn setup_panic_handler() {
    panic::set_hook(Box::new(|info| {
        let location = info.location().map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()));
        let message = info.payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "Unknown panic".to_string());

        error!(
            message = %message,
            location = ?location,
            "Application panicked"
        );
    }));
}
```
