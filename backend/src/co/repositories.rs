use sqlx::PgPool;
use uuid::Uuid;

use crate::co::models::*;
use crate::shared::pagination::ListParams;
use crate::shared::AppError;

// --- Cost Centers ---
pub async fn list_cost_centers(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<CostCenter>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, CostCenter>(
        r#"SELECT * FROM co_cost_centers
           WHERE is_active = true
             AND ($1 = false OR (code ILIKE $2 OR name ILIKE $2))
           ORDER BY code
           LIMIT $3 OFFSET $4"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_cost_centers(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM co_cost_centers
           WHERE is_active = true
             AND ($1 = false OR (code ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_cost_center(pool: &PgPool, id: Uuid) -> Result<CostCenter, AppError> {
    sqlx::query_as::<_, CostCenter>("SELECT * FROM co_cost_centers WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Cost center not found".to_string()))
}

pub async fn create_cost_center(
    pool: &PgPool,
    input: &CreateCostCenter,
) -> Result<CostCenter, AppError> {
    let row = sqlx::query_as::<_, CostCenter>(
        "INSERT INTO co_cost_centers (code, name, description, responsible_person, valid_from, valid_to) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(&input.code).bind(&input.name).bind(&input.description).bind(input.responsible_person)
    .bind(input.valid_from).bind(input.valid_to)
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Profit Centers ---
pub async fn list_profit_centers(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<ProfitCenter>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, ProfitCenter>(
        r#"SELECT * FROM co_profit_centers
           WHERE is_active = true
             AND ($1 = false OR (code ILIKE $2 OR name ILIKE $2))
           ORDER BY code
           LIMIT $3 OFFSET $4"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_profit_centers(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM co_profit_centers
           WHERE is_active = true
             AND ($1 = false OR (code ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_profit_center(pool: &PgPool, id: Uuid) -> Result<ProfitCenter, AppError> {
    sqlx::query_as::<_, ProfitCenter>("SELECT * FROM co_profit_centers WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Profit center not found".to_string()))
}

pub async fn create_profit_center(
    pool: &PgPool,
    input: &CreateProfitCenter,
) -> Result<ProfitCenter, AppError> {
    let row = sqlx::query_as::<_, ProfitCenter>(
        "INSERT INTO co_profit_centers (code, name, description, responsible_person) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&input.code).bind(&input.name).bind(&input.description).bind(input.responsible_person)
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Internal Orders ---
pub async fn list_internal_orders(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<InternalOrder>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, InternalOrder>(
        r#"SELECT * FROM co_internal_orders
           WHERE ($1 = false OR (order_number ILIKE $2 OR name ILIKE $2))
             AND ($3 = false OR status = $4)
             AND ($5 = false OR order_type = $6)
           ORDER BY created_at DESC
           LIMIT $7 OFFSET $8"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_filter_type)
    .bind(&filter_type)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_internal_orders(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM co_internal_orders
           WHERE ($1 = false OR (order_number ILIKE $2 OR name ILIKE $2))
             AND ($3 = false OR status = $4)
             AND ($5 = false OR order_type = $6)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_filter_type)
    .bind(&filter_type)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_internal_order(pool: &PgPool, id: Uuid) -> Result<InternalOrder, AppError> {
    sqlx::query_as::<_, InternalOrder>("SELECT * FROM co_internal_orders WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Internal order not found".to_string()))
}

pub async fn create_internal_order(
    pool: &PgPool,
    order_number: &str,
    input: &CreateInternalOrder,
) -> Result<InternalOrder, AppError> {
    let row = sqlx::query_as::<_, InternalOrder>(
        "INSERT INTO co_internal_orders (order_number, name, order_type, cost_center_id, budget) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(order_number).bind(&input.name).bind(&input.order_type).bind(input.cost_center_id)
    .bind(input.budget.unwrap_or_default())
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn update_internal_order(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateInternalOrder,
) -> Result<InternalOrder, AppError> {
    let row = sqlx::query_as::<_, InternalOrder>(
        "UPDATE co_internal_orders SET name = COALESCE($2, name), status = COALESCE($3, status), budget = COALESCE($4, budget), actual_cost = COALESCE($5, actual_cost), updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id).bind(&input.name).bind(&input.status).bind(input.budget).bind(input.actual_cost)
    .fetch_optional(pool).await?
    .ok_or_else(|| AppError::NotFound("Internal order not found".to_string()))?;
    Ok(row)
}

// --- Cost Allocations ---
pub async fn list_cost_allocations(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<CostAllocation>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, CostAllocation>(
        r#"SELECT * FROM co_cost_allocations
           WHERE ($1 = false OR COALESCE(description, '') ILIKE $2)
           ORDER BY allocation_date DESC
           LIMIT $3 OFFSET $4"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_cost_allocations(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM co_cost_allocations
           WHERE ($1 = false OR COALESCE(description, '') ILIKE $2)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_cost_allocation(
    pool: &PgPool,
    input: &CreateCostAllocation,
) -> Result<CostAllocation, AppError> {
    let row = sqlx::query_as::<_, CostAllocation>(
        "INSERT INTO co_cost_allocations (from_cost_center_id, to_cost_center_id, allocation_date, amount, description) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(input.from_cost_center_id).bind(input.to_cost_center_id).bind(input.allocation_date)
    .bind(input.amount).bind(&input.description)
    .fetch_one(pool).await?;
    Ok(row)
}

/// Create a cost allocation record from an auto-posting (cross-module integration).
pub async fn create_auto_cost_allocation(
    pool: &PgPool,
    input: &AutoPostCostAllocation,
) -> Result<CostAllocation, AppError> {
    let row = sqlx::query_as::<_, CostAllocation>(
        "INSERT INTO co_cost_allocations (from_cost_center_id, to_cost_center_id, allocation_date, amount, description, source_module, reference_id, profit_center_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
    )
    .bind(input.from_cost_center_id)
    .bind(input.to_cost_center_id)
    .bind(input.allocation_date)
    .bind(input.amount)
    .bind(&input.description)
    .bind(&input.source_module)
    .bind(input.reference_id)
    .bind(input.profit_center_id)
    .fetch_one(pool).await?;
    Ok(row)
}

/// Look up a cost center by its code. Returns None if not found.
pub async fn get_cost_center_by_code(
    pool: &PgPool,
    code: &str,
) -> Result<Option<CostCenter>, AppError> {
    let row = sqlx::query_as::<_, CostCenter>(
        "SELECT * FROM co_cost_centers WHERE code = $1 AND is_active = true",
    )
    .bind(code)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn next_number(pool: &PgPool, object_type: &str) -> Result<String, AppError> {
    crate::shared::number_range::next_number(pool, object_type).await
}
