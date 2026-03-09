# ERP Module Template

> Use this template when implementing a new ERP module in the Rust backend.

## Module Structure

```
backend/src/erp/{module}/
├── mod.rs                    # Module exports
├── models.rs                 # Data models (struct definitions)
├── service.rs                # Business logic
├── repository.rs             # Database queries (SQLx)
├── handlers.rs               # Axum HTTP handlers
├── routes.rs                 # Route definitions
└── tests.rs                  # Unit tests
```

## 1. Models (models.rs)

```rust
use chrono::{NaiveDate, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Master Data Model
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct {Resource} {
    pub id: Uuid,
    pub {resource}_number: String,
    pub description: String,
    pub {resource}_type: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// Transaction Document Model
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct {Document} {
    pub id: Uuid,
    pub document_number: String,
    pub status: String,
    pub fiscal_year: i32,
    pub posting_date: NaiveDate,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create Request DTO
#[derive(Debug, Deserialize, Validate)]
pub struct Create{Document}Request {
    pub {related}_id: Uuid,

    pub order_date: NaiveDate,

    #[validate(length(min = 1))]
    pub items: Vec<{Document}ItemInput>,

    #[validate(length(max = 2000))]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct {Document}ItemInput {
    pub material_id: Uuid,
    #[validate(range(min = 0.001))]
    pub quantity: Decimal,
    #[validate(range(min = 0.0))]
    pub unit_price: Decimal,
}
```

## 2. Repository (repository.rs)

```rust
use sqlx::PgPool;
use uuid::Uuid;

pub struct {Module}Repository {
    pool: PgPool,
}

impl {Module}Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<{Resource}>, sqlx::Error> {
        sqlx::query_as!(
            {Resource},
            r#"SELECT * FROM {module}_{resources} WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<{Resource}>, sqlx::Error> {
        let offset = (page - 1) * per_page;
        sqlx::query_as!(
            {Resource},
            r#"
            SELECT * FROM {module}_{resources}
            WHERE is_active = true
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            per_page as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create(&self, data: Create{Resource}Request, user_id: Uuid) -> Result<{Resource}, sqlx::Error> {
        sqlx::query_as!(
            {Resource},
            r#"
            INSERT INTO {module}_{resources} ({resource}_number, description, {resource}_type, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            data.{resource}_number,
            data.description,
            data.{resource}_type,
            user_id,
        )
        .fetch_one(&self.pool)
        .await
    }
}
```

## 3. Service (service.rs)

```rust
use crate::error::AppError;

pub struct {Module}Service {
    repo: {Module}Repository,
}

impl {Module}Service {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: {Module}Repository::new(pool),
        }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<{Resource}, AppError> {
        self.repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("{Resource} not found: {}", id)))
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        data: Create{Resource}Request,
    ) -> Result<{Resource}, AppError> {
        data.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Business logic validation
        // ...

        self.repo.create(data, user_id).await
            .map_err(AppError::from)
    }
}
```

## 4. Handlers (handlers.rs)

```rust
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use uuid::Uuid;

pub async fn list_{resources}(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<{Resource}>>>, AppError> {
    user.check_auth_object("S_{MODULE}_{RESOURCE}", "READ")?;

    let result = state.{module}_service
        .list(&params)
        .await?;

    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_{resource}(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<{Resource}>>, AppError> {
    user.check_auth_object("S_{MODULE}_{RESOURCE}", "READ")?;

    let item = state.{module}_service.get_by_id(id).await?;

    Ok(Json(ApiResponse::success(item)))
}

pub async fn create_{resource}(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Json(payload): Json<Create{Resource}Request>,
) -> Result<Json<ApiResponse<{Resource}>>, AppError> {
    user.check_auth_object("S_{MODULE}_{RESOURCE}", "CREATE")?;

    let item = state.{module}_service
        .create(user.id, payload)
        .await?;

    Ok(Json(ApiResponse::success(item)))
}
```

## 5. Routes (routes.rs)

```rust
use axum::{routing::{get, post, put}, Router};

pub fn {module}_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{resources}", get(list_{resources}).post(create_{resource}))
        .route("/{resources}/:id", get(get_{resource}).put(update_{resource}))
}
```

## 6. Register in Main Router

```rust
// routes/mod.rs
fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/{module}", {module}_routes())
        // ... other modules
}
```

## Checklist

- [ ] Models use `Decimal` for monetary fields
- [ ] All handlers check authorization objects
- [ ] Transaction documents have document_number generation
- [ ] Status transitions are validated in service layer
- [ ] Repository uses parameterized queries (SQLx)
- [ ] Error handling uses `AppError` enum
- [ ] Unit tests in tests.rs
