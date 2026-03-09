# Rust Axum API 開發指南

## 目錄
1. [應用程式結構](#應用程式結構)
2. [路由定義](#路由定義)
3. [Handler 模式](#handler-模式)
4. [狀態管理](#狀態管理)
5. [回應格式](#回應格式)

## 應用程式結構

### 基礎 App 配置
```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
    Extension,
};
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
};
use std::sync::Arc;

pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        // API 路由
        .nest("/api/v1", api_routes())
        // 健康檢查
        .route("/health", get(health_check))
        // 中間件
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        // 共享狀態
        .with_state(state)
}

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/fi", fi_routes())
        .nest("/mm", mm_routes())
        .nest("/sd", sd_routes())
}
```

### 模組組織
```
src/
├── main.rs
├── lib.rs
├── config/
│   ├── mod.rs
│   └── settings.rs
├── routes/
│   ├── mod.rs
│   ├── auth.rs
│   └── erp/          # ERP module routes
├── handlers/
│   ├── mod.rs
│   ├── auth.rs
│   └── erp/
├── models/
│   ├── mod.rs
│   └── erp/
├── services/
│   ├── mod.rs
│   └── erp/
├── middleware/
│   ├── mod.rs
│   └── auth.rs
└── error.rs
```

## 路由定義

### RESTful 路由
```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};

pub fn sd_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/sales-orders", get(list_sales_orders).post(create_sales_order))
        .route("/sales-orders/:id", get(get_sales_order).put(update_sales_order))
        .route("/customers", get(list_customers).post(create_customer))
        .route("/customers/:id", get(get_customer).put(update_customer))
}
```

### 路由分組與中間件
```rust
use crate::middleware::auth::auth_middleware;

pub fn protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/profile", get(get_profile).put(update_profile))
        .route("/sales-orders", post(create_sales_order))
        // 套用認證中間件
        .layer(axum::middleware::from_fn(auth_middleware))
}
```

## Handler 模式

### 基礎 Handler
```rust
use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

// GET /sd/sales-orders/:id
pub async fn get_sales_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SalesOrder>>, AppError> {
    let order = state.sd_service.get_by_id(id).await?;

    Ok(Json(ApiResponse::success(order)))
}

// GET /sd/sales-orders?page=1&limit=20
pub async fn list_sales_orders(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SalesOrder>>>, AppError> {
    let orders = state.sd_service
        .list_sales_orders(params.page, params.limit)
        .await?;

    Ok(Json(ApiResponse::success(orders)))
}

// POST /sd/sales-orders
pub async fn create_sales_order(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Json(payload): Json<CreateSalesOrderRequest>,
) -> Result<Json<ApiResponse<SalesOrder>>, AppError> {
    // 驗證輸入
    payload.validate()?;

    let order = state.sd_service
        .create_sales_order(user.id, payload)
        .await?;

    Ok(Json(ApiResponse::success(order)))
}
```

### 請求驗證
```rust
use validator::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSalesOrderRequest {
    pub customer_id: Uuid,

    pub order_date: chrono::NaiveDate,

    #[validate(length(min = 1))]
    pub items: Vec<SalesOrderItemInput>,

    #[validate(length(max = 2000))]
    pub notes: Option<String>,
}
```

## 狀態管理

### AppState 定義
```rust
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub db: PgPool,
    pub config: AppConfig,
    // ERP 服務
    pub fi_service: FiService,
    pub mm_service: MmService,
    pub sd_service: SdService,
    pub pp_service: PpService,
    pub hr_service: HrService,
    pub wm_service: WmService,
    pub qm_service: QmService,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Arc<Self>, anyhow::Error> {
        // 建立資料庫連線池 (postgres://localhost:5432/TastyByte)
        let db = PgPool::connect(&config.database_url).await?;

        // 初始化 ERP 服務
        let fi_service = FiService::new(db.clone());
        let mm_service = MmService::new(db.clone());
        let sd_service = SdService::new(db.clone());
        let pp_service = PpService::new(db.clone());
        let hr_service = HrService::new(db.clone());
        let wm_service = WmService::new(db.clone());
        let qm_service = QmService::new(db.clone());

        Ok(Arc::new(Self {
            db,
            config,
            fi_service,
            mm_service,
            sd_service,
            pp_service,
            hr_service,
            wm_service,
            qm_service,
        }))
    }
}
```

## 回應格式

### 統一 API 回應
```rust
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub meta: ResponseMeta,
}

#[derive(Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct ResponseMeta {
    pub timestamp: DateTime<Utc>,
    pub request_id: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: ResponseMeta {
                timestamp: Utc::now(),
                request_id: uuid::Uuid::new_v4().to_string(),
            },
        }
    }

    pub fn error(code: impl Into<String>, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                details: None,
            }),
            meta: ResponseMeta {
                timestamp: Utc::now(),
                request_id: uuid::Uuid::new_v4().to_string(),
            },
        }
    }
}
```

### 分頁回應
```rust
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: i32,
    pub per_page: i32,
    pub total: i64,
    pub total_pages: i32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, page: i32, per_page: i32, total: i64) -> Self {
        Self {
            items,
            page,
            per_page,
            total,
            total_pages: ((total as f64) / (per_page as f64)).ceil() as i32,
        }
    }
}
```
