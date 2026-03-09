# Rust SQLx 資料庫整合指南

## 目錄
1. [連線池配置](#連線池配置)
2. [查詢模式](#查詢模式)
3. [交易處理](#交易處理)
4. [Repository 模式](#repository-模式)

## 連線池配置

### 基礎配置
```rust
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

// DATABASE_URL=postgres://localhost:5432/TastyByte
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
}
```

### 健康檢查
```rust
pub async fn check_database_health(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .is_ok()
}
```

## 查詢模式

### 基礎 CRUD
```rust
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, FromRow)]
pub struct SalesOrder {
    pub id: Uuid,
    pub document_number: String,
    pub customer_id: Uuid,
    pub status: String,
    pub total_amount: Decimal,
    pub currency: String,
    pub fiscal_year: i32,
    pub posting_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// SELECT by ID
pub async fn get_sales_order_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<SalesOrder>, sqlx::Error> {
    sqlx::query_as::<_, SalesOrder>(
        r#"
        SELECT id, document_number, customer_id, status,
               total_amount, currency, fiscal_year, posting_date,
               notes, created_at, updated_at
        FROM sd_sales_orders
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

// SELECT with pagination
pub async fn list_sales_orders(
    pool: &PgPool,
    status: Option<&str>,
    page: i32,
    per_page: i32,
) -> Result<Vec<SalesOrder>, sqlx::Error> {
    let offset = (page - 1) * per_page;

    sqlx::query_as::<_, SalesOrder>(
        r#"
        SELECT id, document_number, customer_id, status,
               total_amount, currency, fiscal_year, posting_date,
               notes, created_at, updated_at
        FROM sd_sales_orders
        WHERE ($1::TEXT IS NULL OR status = $1)
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(status)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await
}

// INSERT
pub async fn create_sales_order(
    pool: &PgPool,
    document_number: &str,
    customer_id: Uuid,
    posting_date: NaiveDate,
    fiscal_year: i32,
    created_by: Uuid,
) -> Result<SalesOrder, sqlx::Error> {
    sqlx::query_as::<_, SalesOrder>(
        r#"
        INSERT INTO sd_sales_orders
            (document_number, customer_id, posting_date, fiscal_year, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(document_number)
    .bind(customer_id)
    .bind(posting_date)
    .bind(fiscal_year)
    .bind(created_by)
    .fetch_one(pool)
    .await
}

// UPDATE status
pub async fn update_sales_order_status(
    pool: &PgPool,
    id: Uuid,
    new_status: &str,
    updated_by: Uuid,
) -> Result<SalesOrder, sqlx::Error> {
    sqlx::query_as::<_, SalesOrder>(
        r#"
        UPDATE sd_sales_orders
        SET status = $2, updated_at = NOW(), updated_by = $3
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .bind(new_status)
    .bind(updated_by)
    .fetch_one(pool)
    .await
}
```

### 複雜查詢
```rust
// JOIN 查詢 - 銷售訂單含明細數
#[derive(Debug, FromRow)]
pub struct SalesOrderSummary {
    pub id: Uuid,
    pub document_number: String,
    pub customer_name: String,
    pub item_count: i64,
    pub total_amount: Decimal,
}

pub async fn get_sales_order_summaries(
    pool: &PgPool,
) -> Result<Vec<SalesOrderSummary>, sqlx::Error> {
    sqlx::query_as::<_, SalesOrderSummary>(
        r#"
        SELECT so.id, so.document_number, c.name as customer_name,
               COUNT(soi.id) as item_count, so.total_amount
        FROM sd_sales_orders so
        JOIN sd_customers c ON so.customer_id = c.id
        LEFT JOIN sd_sales_order_items soi ON so.id = soi.sales_order_id
        GROUP BY so.id, so.document_number, c.name, so.total_amount
        ORDER BY so.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

// 全文搜尋
pub async fn search_materials(
    pool: &PgPool,
    query: &str,
    limit: i32,
) -> Result<Vec<Material>, sqlx::Error> {
    sqlx::query_as::<_, Material>(
        r#"
        SELECT *
        FROM mm_materials
        WHERE is_active = true
          AND (
            material_number ILIKE '%' || $1 || '%'
            OR description ILIKE '%' || $1 || '%'
          )
        ORDER BY material_number
        LIMIT $2
        "#
    )
    .bind(query)
    .bind(limit)
    .fetch_all(pool)
    .await
}
```

## 交易處理

### 基礎交易 - 建立銷售訂單含明細
```rust
pub async fn create_sales_order_with_items(
    pool: &PgPool,
    user_id: Uuid,
    order_data: CreateSalesOrderRequest,
) -> Result<SalesOrder, sqlx::Error> {
    let mut tx = pool.begin().await?;

    // 產生文件編號
    let doc_number = generate_document_number(&mut *tx, "SO").await?;

    // 建立訂單主檔
    let order = sqlx::query_as::<_, SalesOrder>(
        r#"
        INSERT INTO sd_sales_orders
            (document_number, customer_id, posting_date, fiscal_year, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(&doc_number)
    .bind(order_data.customer_id)
    .bind(order_data.order_date)
    .bind(order_data.order_date.year())
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    // 建立訂單明細
    let mut total = Decimal::ZERO;
    for (idx, item) in order_data.items.iter().enumerate() {
        let amount = item.quantity * item.unit_price;
        total += amount;

        sqlx::query(
            r#"
            INSERT INTO sd_sales_order_items
                (sales_order_id, item_number, material_id, quantity, unit_price, amount)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(order.id)
        .bind((idx + 1) as i32)
        .bind(item.material_id)
        .bind(item.quantity)
        .bind(item.unit_price)
        .bind(amount)
        .execute(&mut *tx)
        .await?;
    }

    // 更新訂單總金額
    sqlx::query("UPDATE sd_sales_orders SET total_amount = $1 WHERE id = $2")
        .bind(total)
        .bind(order.id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(order)
}
```

### 跨模組交易 - 訂單確認觸發庫存預留
```rust
pub async fn confirm_sales_order(
    pool: &PgPool,
    order_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await?;

    // 更新訂單狀態
    sqlx::query("UPDATE sd_sales_orders SET status = 'CONFIRMED' WHERE id = $1")
        .bind(order_id)
        .execute(&mut *tx)
        .await?;

    // 預留庫存 (MM 模組)
    let items = sqlx::query_as::<_, SalesOrderItem>(
        "SELECT * FROM sd_sales_order_items WHERE sales_order_id = $1"
    )
    .bind(order_id)
    .fetch_all(&mut *tx)
    .await?;

    for item in &items {
        sqlx::query(
            "INSERT INTO wm_inventory_reservations (material_id, quantity, reference_doc) VALUES ($1, $2, $3)"
        )
        .bind(item.material_id)
        .bind(item.quantity)
        .bind(order_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
```

## Repository 模式

### Repository Trait
```rust
use async_trait::async_trait;

#[async_trait]
pub trait SdRepository: Send + Sync {
    async fn find_sales_order_by_id(&self, id: Uuid) -> Result<Option<SalesOrder>, AppError>;
    async fn list_sales_orders(&self, status: Option<&str>, page: i32, per_page: i32) -> Result<Vec<SalesOrder>, AppError>;
    async fn create_sales_order(&self, user_id: Uuid, data: CreateSalesOrderRequest) -> Result<SalesOrder, AppError>;
    async fn update_status(&self, id: Uuid, status: &str, user_id: Uuid) -> Result<SalesOrder, AppError>;
    async fn count_by_status(&self, status: &str) -> Result<i64, AppError>;
}
```

### Repository 實作
```rust
pub struct PgSdRepository {
    pool: PgPool,
}

impl PgSdRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SdRepository for PgSdRepository {
    async fn find_sales_order_by_id(&self, id: Uuid) -> Result<Option<SalesOrder>, AppError> {
        get_sales_order_by_id(&self.pool, id)
            .await
            .map_err(AppError::Database)
    }

    async fn create_sales_order(&self, user_id: Uuid, data: CreateSalesOrderRequest) -> Result<SalesOrder, AppError> {
        create_sales_order_with_items(&self.pool, user_id, data)
            .await
            .map_err(AppError::Database)
    }

    async fn count_by_status(&self, status: &str) -> Result<i64, AppError> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sd_sales_orders WHERE status = $1"
        )
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.0)
    }

    // ... 其他方法實作
}
```
