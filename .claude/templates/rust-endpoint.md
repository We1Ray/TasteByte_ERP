# Rust API Endpoint Template

> Use this template when adding a new API endpoint to the Rust backend.

## Directory Structure

```
backend/src/
├── handlers/
│   └── {resource}.rs       # HTTP handlers
├── models/
│   └── {resource}.rs       # Request/Response DTOs
├── db/
│   └── {resource}.rs       # Database operations
└── routes/
    └── mod.rs              # Route registration
```

## 1. Models (Request/Response DTOs)

```rust
// src/models/{resource}.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

// Database model
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct {Resource} {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Create request
#[derive(Debug, Deserialize, Validate)]
pub struct Create{Resource}Request {
    #[validate(length(min = 1, max = 255))]
    pub title: String,

    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

// Update request
#[derive(Debug, Deserialize, Validate)]
pub struct Update{Resource}Request {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

// List query params
#[derive(Debug, Deserialize)]
pub struct List{Resource}Query {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

impl Default for List{Resource}Query {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
            search: None,
        }
    }
}
```

## 2. Database Operations

```rust
// src/db/{resource}.rs
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{Resource};
use crate::error::Result;

pub struct {Resource}Repo;

impl {Resource}Repo {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<{Resource}>> {
        let result = sqlx::query_as!(
            {Resource},
            r#"
            SELECT id, user_id, title, description, created_at, updated_at
            FROM {resources}
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_user(
        pool: &PgPool,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<{Resource}>> {
        let offset = (page - 1) * per_page;

        let results = sqlx::query_as!(
            {Resource},
            r#"
            SELECT id, user_id, title, description, created_at, updated_at
            FROM {resources}
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        title: &str,
        description: Option<&str>,
    ) -> Result<{Resource}> {
        let result = sqlx::query_as!(
            {Resource},
            r#"
            INSERT INTO {resources} (user_id, title, description)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, title, description, created_at, updated_at
            "#,
            user_id,
            title,
            description
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        title: Option<&str>,
        description: Option<&str>,
    ) -> Result<Option<{Resource}>> {
        let result = sqlx::query_as!(
            {Resource},
            r#"
            UPDATE {resources}
            SET
                title = COALESCE($3, title),
                description = COALESCE($4, description),
                updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, title, description, created_at, updated_at
            "#,
            id,
            user_id,
            title,
            description
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM {resources}
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
```

## 3. Handlers

```rust
// src/handlers/{resource}.rs
use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::AuthUser,
    db::{Resource}Repo,
    error::{AppError, Result},
    models::{
        Create{Resource}Request,
        Update{Resource}Request,
        List{Resource}Query,
        {Resource}
    },
    response::ApiResponse,
    AppState,
};

/// List user's {resources}
pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<List{Resource}Query>,
) -> Result<Json<ApiResponse<Vec<{Resource}>>>> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100);

    let items = {Resource}Repo::find_by_user(
        &state.db,
        auth.user_id,
        page,
        per_page,
    ).await?;

    Ok(Json(ApiResponse::success(items)))
}

/// Get single {resource}
pub async fn get(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<{Resource}>>> {
    let item = {Resource}Repo::find_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound("{Resource} not found".into()))?;

    // Check ownership
    if item.user_id != auth.user_id {
        return Err(AppError::Forbidden);
    }

    Ok(Json(ApiResponse::success(item)))
}

/// Create new {resource}
pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<Create{Resource}Request>,
) -> Result<Json<ApiResponse<{Resource}>>> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let item = {Resource}Repo::create(
        &state.db,
        auth.user_id,
        &payload.title,
        payload.description.as_deref(),
    ).await?;

    Ok(Json(ApiResponse::success(item)))
}

/// Update {resource}
pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<Update{Resource}Request>,
) -> Result<Json<ApiResponse<{Resource}>>> {
    payload.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let item = {Resource}Repo::update(
        &state.db,
        id,
        auth.user_id,
        payload.title.as_deref(),
        payload.description.as_deref(),
    )
    .await?
    .ok_or(AppError::NotFound("{Resource} not found".into()))?;

    Ok(Json(ApiResponse::success(item)))
}

/// Delete {resource}
pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>> {
    let deleted = {Resource}Repo::delete(&state.db, id, auth.user_id).await?;

    if !deleted {
        return Err(AppError::NotFound("{Resource} not found".into()));
    }

    Ok(Json(ApiResponse::success(())))
}
```

## 4. Route Registration

```rust
// src/routes/mod.rs
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers;

pub fn {resource}_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::{resource}::list))
        .route("/", post(handlers::{resource}::create))
        .route("/:id", get(handlers::{resource}::get))
        .route("/:id", put(handlers::{resource}::update))
        .route("/:id", delete(handlers::{resource}::delete))
}

// In main router
pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/api/v1/{resources}", {resource}_routes())
        // ... other routes
}
```

## Checklist

- [ ] Created model structs with validation
- [ ] Implemented database operations
- [ ] Created handlers with proper error handling
- [ ] Registered routes
- [ ] Added ownership checks
- [ ] Input validation
- [ ] Pagination support
- [ ] Added to OpenAPI documentation (if applicable)
