use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::pagination::ListParams;
use crate::shared::{AppError, PaginatedResponse};
use crate::wm::models::*;
use crate::wm::repositories;

pub async fn list_warehouses(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Warehouse>, AppError> {
    let total = repositories::count_warehouses(pool, params).await?;
    let data = repositories::list_warehouses(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_warehouse(pool: &PgPool, id: Uuid) -> Result<Warehouse, AppError> {
    repositories::get_warehouse(pool, id).await
}

pub async fn create_warehouse(
    pool: &PgPool,
    input: CreateWarehouse,
) -> Result<Warehouse, AppError> {
    repositories::create_warehouse(pool, &input).await
}

pub async fn list_storage_bins(
    pool: &PgPool,
    warehouse_id: Uuid,
) -> Result<Vec<StorageBin>, AppError> {
    repositories::list_storage_bins(pool, warehouse_id).await
}

pub async fn create_storage_bin(
    pool: &PgPool,
    input: CreateStorageBin,
) -> Result<StorageBin, AppError> {
    repositories::create_storage_bin(pool, &input).await
}

pub async fn list_stock_transfers(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<StockTransfer>, AppError> {
    let total = repositories::count_stock_transfers(pool, params).await?;
    let data = repositories::list_stock_transfers(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn create_stock_transfer(
    pool: &PgPool,
    input: CreateStockTransfer,
    user_id: Uuid,
) -> Result<StockTransfer, AppError> {
    if input.from_warehouse_id == input.to_warehouse_id {
        return Err(AppError::Validation(
            "Source and destination warehouses must be different".to_string(),
        ));
    }
    let transfer_number = repositories::next_number(pool, "TRF").await?;
    let mvt_doc_number = repositories::next_number(pool, "MVT").await?;
    repositories::create_stock_transfer(pool, &transfer_number, &mvt_doc_number, &input, user_id)
        .await
}

pub async fn list_stock_counts(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<StockCount>, AppError> {
    let total = repositories::count_stock_counts(pool, params).await?;
    let data = repositories::list_stock_counts(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_stock_count(
    pool: &PgPool,
    id: Uuid,
) -> Result<(StockCount, Vec<StockCountItem>), AppError> {
    let count = repositories::get_stock_count(pool, id).await?;
    let items = repositories::get_stock_count_items(pool, id).await?;
    Ok((count, items))
}

pub async fn create_stock_count(
    pool: &PgPool,
    input: CreateStockCount,
) -> Result<StockCount, AppError> {
    let count_number = repositories::next_number(pool, "CNT").await?;
    repositories::create_stock_count(pool, &count_number, &input).await
}

/// Complete a stock count: reconcile counted quantities to MM plant stock.
/// Creates ADJUSTMENT material movements for items where counted_quantity
/// differs from book_quantity, and updates mm_plant_stock accordingly.
/// GAP 6: Also creates FI journal entry for inventory adjustment value.
pub async fn complete_stock_count(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<(StockCount, Vec<StockCountItem>), AppError> {
    // 1. Complete the stock count (creates adjustments in MM)
    let count = repositories::complete_stock_count(pool, id, user_id).await?;
    let items = repositories::get_stock_count_items(pool, id).await?;

    // 2. GAP 6: Calculate total adjustment value for FI posting
    // For each item with a difference, look up the material's latest price
    let mut net_adjustment_value = rust_decimal::Decimal::ZERO;
    for item in &items {
        let diff = item.counted_quantity.unwrap_or_default() - item.book_quantity;
        if diff == rust_decimal::Decimal::ZERO {
            continue;
        }
        // Get material's price (latest PO unit price or default 0)
        let price: rust_decimal::Decimal = sqlx::query_as::<_, (rust_decimal::Decimal,)>(
            "SELECT COALESCE( \
                (SELECT poi.unit_price FROM mm_purchase_order_items poi \
                 JOIN mm_purchase_orders po ON po.id = poi.purchase_order_id \
                 WHERE poi.material_id = $1 ORDER BY po.created_at DESC LIMIT 1), \
                0 \
            )",
        )
        .bind(item.material_id)
        .fetch_one(pool)
        .await
        .map(|r| r.0)
        .unwrap_or_default();

        net_adjustment_value += diff * price;
    }

    // 3. Create FI journal entry for inventory adjustments (if material)
    if net_adjustment_value != rust_decimal::Decimal::ZERO {
        let inv_account = sqlx::query_as::<_, (uuid::Uuid,)>(
            "SELECT id FROM fi_accounts WHERE account_number = '1300'",
        )
        .fetch_optional(pool)
        .await?;
        let adj_account = sqlx::query_as::<_, (uuid::Uuid,)>(
            "SELECT id FROM fi_accounts WHERE account_number = '6600'",
        )
        .fetch_optional(pool)
        .await?;
        let company_code =
            sqlx::query_as::<_, (uuid::Uuid,)>("SELECT id FROM fi_company_codes LIMIT 1")
                .fetch_optional(pool)
                .await?;

        if let (Some((inv_id,)), Some((adj_id,)), Some((cc_id,))) =
            (inv_account, adj_account, company_code)
        {
            let today = chrono::Utc::now().date_naive();
            let abs_value = net_adjustment_value.abs();

            let (debit_account, credit_account) =
                if net_adjustment_value > rust_decimal::Decimal::ZERO {
                    // Positive adjustment (counted > book): DR Inventory, CR Inv Adjustment
                    (inv_id, adj_id)
                } else {
                    // Negative adjustment (counted < book): DR Inv Adjustment, CR Inventory
                    (adj_id, inv_id)
                };

            let je_input = crate::fi::models::CreateJournalEntry {
                company_code_id: cc_id,
                posting_date: today,
                document_date: today,
                reference: Some(format!("WM-ADJ:{}", count.count_number)),
                description: Some(format!(
                    "Inventory adjustment from stock count {}",
                    count.count_number
                )),
                items: vec![
                    crate::fi::models::CreateJournalItem {
                        account_id: debit_account,
                        debit_amount: abs_value,
                        credit_amount: rust_decimal::Decimal::ZERO,
                        cost_center_id: None,
                        description: Some("Inventory adjustment".to_string()),
                    },
                    crate::fi::models::CreateJournalItem {
                        account_id: credit_account,
                        debit_amount: rust_decimal::Decimal::ZERO,
                        credit_amount: abs_value,
                        cost_center_id: None,
                        description: Some("Inventory adjustment".to_string()),
                    },
                ],
            };
            // Use the non-transactional variant since the stock count is already committed
            match crate::fi::services::create_journal_entry(pool, je_input, user_id).await {
                Ok(je) => {
                    if let Err(e) =
                        crate::fi::services::post_journal_entry(pool, je.id, user_id).await
                    {
                        tracing::warn!("Failed to post WM adjustment journal entry: {e}");
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to create WM adjustment journal entry: {e}");
                }
            }
        }
    }

    Ok((count, items))
}
