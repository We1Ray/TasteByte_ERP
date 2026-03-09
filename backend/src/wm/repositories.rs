use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::mm::models::CreateMaterialMovement;
use crate::shared::pagination::ListParams;
use crate::shared::AppError;
use crate::wm::models::*;

// --- Warehouses ---
pub async fn list_warehouses(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<Warehouse>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, Warehouse>(
        r#"SELECT * FROM wm_warehouses
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

pub async fn count_warehouses(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM wm_warehouses
           WHERE is_active = true
             AND ($1 = false OR (code ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_warehouse(pool: &PgPool, id: Uuid) -> Result<Warehouse, AppError> {
    sqlx::query_as::<_, Warehouse>("SELECT * FROM wm_warehouses WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Warehouse not found".to_string()))
}

pub async fn create_warehouse(
    pool: &PgPool,
    input: &CreateWarehouse,
) -> Result<Warehouse, AppError> {
    let row = sqlx::query_as::<_, Warehouse>(
        "INSERT INTO wm_warehouses (code, name, address, warehouse_type) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&input.code).bind(&input.name).bind(&input.address).bind(input.warehouse_type.as_deref().unwrap_or("STANDARD"))
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Storage Bins ---
pub async fn list_storage_bins(
    pool: &PgPool,
    warehouse_id: Uuid,
) -> Result<Vec<StorageBin>, AppError> {
    let rows = sqlx::query_as::<_, StorageBin>(
        "SELECT * FROM wm_storage_bins WHERE warehouse_id = $1 AND is_active = true ORDER BY bin_code"
    ).bind(warehouse_id).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn create_storage_bin(
    pool: &PgPool,
    input: &CreateStorageBin,
) -> Result<StorageBin, AppError> {
    let row = sqlx::query_as::<_, StorageBin>(
        "INSERT INTO wm_storage_bins (warehouse_id, bin_code, zone, aisle, rack, level, max_weight) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(input.warehouse_id).bind(&input.bin_code).bind(&input.zone).bind(&input.aisle)
    .bind(&input.rack).bind(&input.level).bind(input.max_weight)
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Stock Transfers ---
pub async fn list_stock_transfers(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<StockTransfer>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, StockTransfer>(
        r#"SELECT * FROM wm_stock_transfers
           WHERE ($1 = false OR transfer_number ILIKE $2)
             AND ($3 = false OR status = $4)
           ORDER BY created_at DESC
           LIMIT $5 OFFSET $6"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_stock_transfers(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM wm_stock_transfers
           WHERE ($1 = false OR transfer_number ILIKE $2)
             AND ($3 = false OR status = $4)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_stock_transfer(
    pool: &PgPool,
    transfer_number: &str,
    mvt_doc_number: &str,
    input: &CreateStockTransfer,
    user_id: Uuid,
) -> Result<StockTransfer, AppError> {
    let mut tx = pool.begin().await?;

    // 1. Validate sufficient stock at source warehouse
    let stock: Option<(Decimal,)> = sqlx::query_as(
        "SELECT quantity FROM mm_plant_stock WHERE material_id = $1 AND warehouse_id = $2",
    )
    .bind(input.material_id)
    .bind(input.from_warehouse_id)
    .fetch_optional(&mut *tx)
    .await?;

    let current_qty = stock.map(|s| s.0).unwrap_or_default();
    if current_qty < input.quantity {
        return Err(AppError::Validation(format!(
            "Insufficient stock at source warehouse: available={}, requested={}",
            current_qty, input.quantity
        )));
    }

    // 2. Create WM stock transfer record
    let transfer = sqlx::query_as::<_, StockTransfer>(
        "INSERT INTO wm_stock_transfers (transfer_number, from_warehouse_id, to_warehouse_id, material_id, quantity, uom_id, status, requested_by, completed_at) \
         VALUES ($1, $2, $3, $4, $5, $6, 'COMPLETED', $7, NOW()) RETURNING *"
    )
    .bind(transfer_number)
    .bind(input.from_warehouse_id)
    .bind(input.to_warehouse_id)
    .bind(input.material_id)
    .bind(input.quantity)
    .bind(input.uom_id)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    // 3. Create MM material movement (TRANSFER) to sync plant stock
    let movement = CreateMaterialMovement {
        movement_type: "TRANSFER".to_string(),
        material_id: input.material_id,
        warehouse_id: Some(input.from_warehouse_id),
        quantity: input.quantity,
        uom_id: input.uom_id,
        reference_type: Some("WM_STOCK_TRANSFER".to_string()),
        reference_id: Some(transfer.id),
    };

    // Insert material movement
    sqlx::query(
        "INSERT INTO mm_material_movements (document_number, movement_type, material_id, warehouse_id, quantity, uom_id, reference_type, reference_id, posted_by) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(mvt_doc_number)
    .bind(&movement.movement_type)
    .bind(movement.material_id)
    .bind(movement.warehouse_id)
    .bind(movement.quantity)
    .bind(movement.uom_id)
    .bind(&movement.reference_type)
    .bind(movement.reference_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    // 4. Decrement stock at source warehouse
    sqlx::query(
        "UPDATE mm_plant_stock SET quantity = quantity - $3, updated_at = NOW() \
         WHERE material_id = $1 AND warehouse_id = $2",
    )
    .bind(input.material_id)
    .bind(input.from_warehouse_id)
    .bind(input.quantity)
    .execute(&mut *tx)
    .await?;

    // 5. Increment stock at destination warehouse (upsert)
    sqlx::query(
        "INSERT INTO mm_plant_stock (material_id, warehouse_id, quantity, uom_id, updated_at) \
         VALUES ($1, $2, $3, $4, NOW()) \
         ON CONFLICT (material_id, warehouse_id) DO UPDATE SET \
         quantity = mm_plant_stock.quantity + $3, updated_at = NOW()",
    )
    .bind(input.material_id)
    .bind(input.to_warehouse_id)
    .bind(input.quantity)
    .bind(input.uom_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(transfer)
}

// --- Stock Counts ---
pub async fn list_stock_counts(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<StockCount>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, StockCount>(
        r#"SELECT * FROM wm_stock_counts
           WHERE ($1 = false OR count_number ILIKE $2)
             AND ($3 = false OR status = $4)
           ORDER BY created_at DESC
           LIMIT $5 OFFSET $6"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_stock_counts(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM wm_stock_counts
           WHERE ($1 = false OR count_number ILIKE $2)
             AND ($3 = false OR status = $4)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_stock_count(pool: &PgPool, id: Uuid) -> Result<StockCount, AppError> {
    sqlx::query_as::<_, StockCount>("SELECT * FROM wm_stock_counts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Stock count not found".to_string()))
}

pub async fn get_stock_count_items(
    pool: &PgPool,
    count_id: Uuid,
) -> Result<Vec<StockCountItem>, AppError> {
    let rows = sqlx::query_as::<_, StockCountItem>(
        "SELECT * FROM wm_stock_count_items WHERE stock_count_id = $1",
    )
    .bind(count_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_stock_count(
    pool: &PgPool,
    count_number: &str,
    input: &CreateStockCount,
) -> Result<StockCount, AppError> {
    let mut tx = pool.begin().await?;

    let count = sqlx::query_as::<_, StockCount>(
        "INSERT INTO wm_stock_counts (count_number, warehouse_id, count_date) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(count_number).bind(input.warehouse_id).bind(input.count_date)
    .fetch_one(&mut *tx).await?;

    for item in &input.items {
        let difference = item.counted_quantity.map(|c| c - item.book_quantity);
        sqlx::query(
            "INSERT INTO wm_stock_count_items (stock_count_id, material_id, storage_bin_id, book_quantity, counted_quantity, difference) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(count.id).bind(item.material_id).bind(item.storage_bin_id).bind(item.book_quantity)
        .bind(item.counted_quantity).bind(difference)
        .execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(count)
}

/// Complete a stock count: transitions status to COMPLETED, creates MM ADJUSTMENT
/// movements for each item where counted_quantity differs from book_quantity,
/// and updates mm_plant_stock to match the counted quantities.
pub async fn complete_stock_count(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<StockCount, AppError> {
    let count = get_stock_count(pool, id).await?;

    if count.status != "PLANNED" {
        return Err(AppError::Validation(format!(
            "Cannot complete stock count in status '{}', must be 'PLANNED'",
            count.status
        )));
    }

    let items = get_stock_count_items(pool, id).await?;
    if items.is_empty() {
        return Err(AppError::Validation("Stock count has no items".to_string()));
    }

    // Ensure all items have counted quantities
    for item in &items {
        if item.counted_quantity.is_none() {
            return Err(AppError::Validation(format!(
                "Stock count item {} has no counted quantity",
                item.id
            )));
        }
    }

    let mut tx = pool.begin().await?;

    // For each item with a difference, create an ADJUSTMENT movement and update plant stock
    for item in &items {
        let counted = item.counted_quantity.unwrap(); // safe: validated above
        let difference = counted - item.book_quantity;

        if difference == Decimal::ZERO {
            continue;
        }

        // Generate a movement document number
        let mvt_doc = next_number_in_tx(&mut tx, "MVT").await?;

        // Create ADJUSTMENT movement referencing this stock count
        sqlx::query(
            "INSERT INTO mm_material_movements (document_number, movement_type, material_id, warehouse_id, quantity, uom_id, reference_type, reference_id, posted_by) \
             VALUES ($1, 'ADJUSTMENT', $2, $3, $4, NULL, 'WM_STOCK_COUNT', $5, $6)"
        )
        .bind(&mvt_doc)
        .bind(item.material_id)
        .bind(count.warehouse_id)
        .bind(counted) // ADJUSTMENT sets absolute quantity
        .bind(id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        // Update mm_plant_stock to the counted quantity (absolute set)
        sqlx::query(
            "INSERT INTO mm_plant_stock (material_id, warehouse_id, quantity, updated_at, last_count_date) \
             VALUES ($1, $2, $3, NOW(), $4) \
             ON CONFLICT (material_id, warehouse_id) DO UPDATE SET \
             quantity = $3, updated_at = NOW(), last_count_date = $4"
        )
        .bind(item.material_id)
        .bind(count.warehouse_id)
        .bind(counted)
        .bind(count.count_date)
        .execute(&mut *tx)
        .await?;
    }

    // Update stock count status to COMPLETED
    let completed = sqlx::query_as::<_, StockCount>(
        "UPDATE wm_stock_counts SET status = 'COMPLETED', counted_by = $2, completed_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(completed)
}

/// Generate next number within an existing transaction
async fn next_number_in_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    object_type: &str,
) -> Result<String, AppError> {
    let row: (String, i64, i32) = sqlx::query_as(
        "UPDATE number_ranges SET current_number = current_number + 1 WHERE object_type = $1 \
         RETURNING prefix, current_number, pad_length",
    )
    .bind(object_type)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| AppError::Internal(format!("Number range not found for {}", object_type)))?;

    let num_str = format!("{:0>width$}", row.1, width = row.2 as usize);
    Ok(format!("{}{}", row.0, num_str))
}

pub async fn next_number(pool: &PgPool, object_type: &str) -> Result<String, AppError> {
    crate::shared::number_range::next_number(pool, object_type).await
}
