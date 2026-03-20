use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::co::models::*;
use crate::co::repositories;
use crate::shared::pagination::ListParams;
use crate::shared::{AppError, PaginatedResponse};

pub async fn list_cost_centers(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<CostCenter>, AppError> {
    let total = repositories::count_cost_centers(pool, params).await?;
    let data = repositories::list_cost_centers(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_cost_center(pool: &PgPool, id: Uuid) -> Result<CostCenter, AppError> {
    repositories::get_cost_center(pool, id).await
}

pub async fn create_cost_center(
    pool: &PgPool,
    input: CreateCostCenter,
) -> Result<CostCenter, AppError> {
    repositories::create_cost_center(pool, &input).await
}

pub async fn list_profit_centers(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<ProfitCenter>, AppError> {
    let total = repositories::count_profit_centers(pool, params).await?;
    let data = repositories::list_profit_centers(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_profit_center(pool: &PgPool, id: Uuid) -> Result<ProfitCenter, AppError> {
    repositories::get_profit_center(pool, id).await
}

pub async fn create_profit_center(
    pool: &PgPool,
    input: CreateProfitCenter,
) -> Result<ProfitCenter, AppError> {
    repositories::create_profit_center(pool, &input).await
}

pub async fn list_internal_orders(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<InternalOrder>, AppError> {
    let total = repositories::count_internal_orders(pool, params).await?;
    let data = repositories::list_internal_orders(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_internal_order(pool: &PgPool, id: Uuid) -> Result<InternalOrder, AppError> {
    repositories::get_internal_order(pool, id).await
}

pub async fn create_internal_order(
    pool: &PgPool,
    input: CreateInternalOrder,
) -> Result<InternalOrder, AppError> {
    let order_number = repositories::next_number(pool, "IO").await?;
    repositories::create_internal_order(pool, &order_number, &input).await
}

pub async fn update_internal_order(
    pool: &PgPool,
    id: Uuid,
    input: UpdateInternalOrder,
) -> Result<InternalOrder, AppError> {
    repositories::update_internal_order(pool, id, &input).await
}

pub async fn list_cost_allocations(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<CostAllocation>, AppError> {
    let total = repositories::count_cost_allocations(pool, params).await?;
    let data = repositories::list_cost_allocations(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn create_cost_allocation(
    pool: &PgPool,
    input: CreateCostAllocation,
) -> Result<CostAllocation, AppError> {
    repositories::create_cost_allocation(pool, &input).await
}

pub async fn update_cost_allocation(
    pool: &PgPool,
    id: Uuid,
    input: UpdateCostAllocation,
) -> Result<CostAllocation, AppError> {
    repositories::update_cost_allocation(pool, id, &input).await
}

pub async fn delete_cost_allocation(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    repositories::delete_cost_allocation(pool, id).await
}

/// Auto-post a cost allocation from another module (FI, MM, PP).
///
/// This is called by cross-module integration hooks to automatically create
/// CO cost allocation records when relevant business events occur.
///
/// The `from` cost center is resolved as CC-GENERAL (general overhead).
/// The `to` cost center is the cost center associated with the transaction.
///
/// If the required cost centers do not exist, the auto-posting is silently
/// skipped (logged as a warning) rather than failing the parent transaction.
pub async fn auto_post_cost_allocation(
    pool: &PgPool,
    source_module: &str,
    reference_id: Uuid,
    cost_center_id: Uuid,
    amount: Decimal,
    allocation_date: NaiveDate,
    description: &str,
) -> Result<Option<CostAllocation>, AppError> {
    // Skip zero-amount allocations
    if amount == Decimal::ZERO {
        return Ok(None);
    }

    // Look up the CC-GENERAL cost center as the source (from) side
    let from_cc = repositories::get_cost_center_by_code(pool, "CC-GENERAL").await?;
    let from_cc = match from_cc {
        Some(cc) => cc,
        None => {
            tracing::warn!(
                "CO auto-posting skipped: CC-GENERAL cost center not found. \
                 source_module={}, reference_id={}",
                source_module,
                reference_id
            );
            return Ok(None);
        }
    };

    // Verify the target cost center exists
    let to_cc = repositories::get_cost_center(pool, cost_center_id).await;
    if to_cc.is_err() {
        tracing::warn!(
            "CO auto-posting skipped: target cost center {} not found. \
             source_module={}, reference_id={}",
            cost_center_id,
            source_module,
            reference_id
        );
        return Ok(None);
    }

    let input = AutoPostCostAllocation {
        from_cost_center_id: from_cc.id,
        to_cost_center_id: cost_center_id,
        allocation_date,
        amount,
        description: description.to_string(),
        source_module: source_module.to_string(),
        reference_id,
        profit_center_id: None,
    };
    let allocation = repositories::create_auto_cost_allocation(pool, &input).await?;

    tracing::info!(
        "CO auto-posted allocation: {} {} -> cost_center={}, amount={}, ref={}",
        source_module,
        allocation.id,
        cost_center_id,
        amount,
        reference_id
    );

    Ok(Some(allocation))
}

/// Auto-post a cost allocation for procurement (MM module).
/// Uses CC-PROCUREMENT as the target cost center if no specific one is provided.
pub async fn auto_post_procurement_cost(
    pool: &PgPool,
    reference_id: Uuid,
    cost_center_id: Option<Uuid>,
    amount: Decimal,
    allocation_date: NaiveDate,
    description: &str,
) -> Result<Option<CostAllocation>, AppError> {
    // Resolve the cost center: use provided one, or fall back to CC-PROCUREMENT
    let target_cc_id = match cost_center_id {
        Some(id) => id,
        None => {
            let cc = repositories::get_cost_center_by_code(pool, "CC-PROCUREMENT").await?;
            match cc {
                Some(cc) => cc.id,
                None => {
                    tracing::warn!(
                        "CO auto-posting skipped: CC-PROCUREMENT cost center not found. reference_id={}",
                        reference_id
                    );
                    return Ok(None);
                }
            }
        }
    };

    auto_post_cost_allocation(
        pool,
        "MM",
        reference_id,
        target_cc_id,
        amount,
        allocation_date,
        description,
    )
    .await
}

/// Auto-post a cost allocation for production (PP module).
/// Uses CC-PRODUCTION as the target cost center if no specific one is provided.
pub async fn auto_post_production_cost(
    pool: &PgPool,
    reference_id: Uuid,
    cost_center_id: Option<Uuid>,
    amount: Decimal,
    allocation_date: NaiveDate,
    description: &str,
) -> Result<Option<CostAllocation>, AppError> {
    let target_cc_id = match cost_center_id {
        Some(id) => id,
        None => {
            let cc = repositories::get_cost_center_by_code(pool, "CC-PRODUCTION").await?;
            match cc {
                Some(cc) => cc.id,
                None => {
                    tracing::warn!(
                        "CO auto-posting skipped: CC-PRODUCTION cost center not found. reference_id={}",
                        reference_id
                    );
                    return Ok(None);
                }
            }
        }
    };

    auto_post_cost_allocation(
        pool,
        "PP",
        reference_id,
        target_cc_id,
        amount,
        allocation_date,
        description,
    )
    .await
}

/// Auto-post revenue to a profit center (called from SD invoice flow).
///
/// This creates a CO cost allocation record tagged with a profit center,
/// enabling profit center reporting for sales revenue. Uses CC-GENERAL as
/// the cost center (profit center is a parallel accounting dimension).
pub async fn auto_post_revenue_to_profit_center(
    pool: &PgPool,
    source_module: &str,
    reference_id: Uuid,
    profit_center_id: Uuid,
    amount: Decimal,
    allocation_date: NaiveDate,
    description: &str,
) -> Result<Option<CostAllocation>, AppError> {
    if amount == Decimal::ZERO {
        return Ok(None);
    }

    // Use CC-GENERAL as from/to (profit center is a parallel dimension)
    let general_cc = repositories::get_cost_center_by_code(pool, "CC-GENERAL").await?;
    let general_cc = match general_cc {
        Some(cc) => cc,
        None => {
            tracing::warn!(
                "CO profit center posting skipped: CC-GENERAL cost center not found. \
                 source_module={}, reference_id={}",
                source_module,
                reference_id
            );
            return Ok(None);
        }
    };

    let input = AutoPostCostAllocation {
        from_cost_center_id: general_cc.id,
        to_cost_center_id: general_cc.id,
        allocation_date,
        amount,
        description: description.to_string(),
        source_module: source_module.to_string(),
        reference_id,
        profit_center_id: Some(profit_center_id),
    };
    let allocation = repositories::create_auto_cost_allocation(pool, &input).await?;

    tracing::info!(
        "CO profit center posting: {} {} for ref {}",
        source_module,
        amount,
        reference_id
    );

    Ok(Some(allocation))
}
