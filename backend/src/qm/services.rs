use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::qm::models::*;
use crate::qm::repositories;
use crate::shared::pagination::ListParams;
use crate::shared::{AppError, PaginatedResponse};

pub async fn list_inspection_lots(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<InspectionLot>, AppError> {
    let total = repositories::count_inspection_lots(pool, params).await?;
    let data = repositories::list_inspection_lots(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_inspection_lot(pool: &PgPool, id: Uuid) -> Result<InspectionLot, AppError> {
    repositories::get_inspection_lot(pool, id).await
}

pub async fn create_inspection_lot(
    pool: &PgPool,
    input: CreateInspectionLot,
    user_id: Uuid,
) -> Result<InspectionLot, AppError> {
    let lot_number = repositories::next_number(pool, "QIL").await?;
    repositories::create_inspection_lot(pool, &lot_number, &input, user_id).await
}

/// Create an inspection lot within an existing transaction.
pub async fn create_inspection_lot_in_tx(
    tx: &mut sqlx::PgConnection,
    input: CreateInspectionLot,
    user_id: Uuid,
) -> Result<InspectionLot, AppError> {
    let lot_number = repositories::next_number_on_conn(&mut *tx, "QIL").await?;
    repositories::create_inspection_lot_on_conn(&mut *tx, &lot_number, &input, user_id).await
}

pub async fn list_inspection_results(
    pool: &PgPool,
    lot_id: Uuid,
) -> Result<Vec<InspectionResult>, AppError> {
    repositories::list_inspection_results(pool, lot_id).await
}

pub async fn create_inspection_result(
    pool: &PgPool,
    input: CreateInspectionResult,
    user_id: Uuid,
) -> Result<InspectionResult, AppError> {
    repositories::create_inspection_result(pool, &input, user_id).await
}

// --- Inspection Result Update/Delete ---

async fn ensure_lot_created_status(pool: &PgPool, lot_id: Uuid) -> Result<(), AppError> {
    let lot = repositories::get_inspection_lot(pool, lot_id).await?;
    if lot.status != "CREATED" {
        return Err(AppError::Validation(format!(
            "Inspection lot is in status '{}', must be CREATED to modify results",
            lot.status
        )));
    }
    Ok(())
}

pub async fn update_inspection_result(
    pool: &PgPool,
    result_id: Uuid,
    input: UpdateInspectionResult,
) -> Result<InspectionResult, AppError> {
    let result = repositories::get_inspection_result(pool, result_id).await?;
    ensure_lot_created_status(pool, result.inspection_lot_id).await?;
    repositories::update_inspection_result(pool, result_id, &input).await
}

pub async fn delete_inspection_result(
    pool: &PgPool,
    result_id: Uuid,
) -> Result<(), AppError> {
    let result = repositories::get_inspection_result(pool, result_id).await?;
    ensure_lot_created_status(pool, result.inspection_lot_id).await?;
    repositories::delete_inspection_result(pool, result_id).await
}

pub async fn list_quality_notifications(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<QualityNotification>, AppError> {
    let total = repositories::count_quality_notifications(pool, params).await?;
    let data = repositories::list_quality_notifications(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_quality_notification(
    pool: &PgPool,
    id: Uuid,
) -> Result<QualityNotification, AppError> {
    repositories::get_quality_notification(pool, id).await
}

pub async fn create_quality_notification(
    pool: &PgPool,
    input: CreateQualityNotification,
    user_id: Uuid,
) -> Result<QualityNotification, AppError> {
    let notif_number = repositories::next_number(pool, "QN").await?;
    repositories::create_quality_notification(pool, &notif_number, &input, user_id).await
}

pub async fn update_quality_notification(
    pool: &PgPool,
    id: Uuid,
    input: UpdateQualityNotification,
) -> Result<QualityNotification, AppError> {
    repositories::update_quality_notification(pool, id, &input).await
}

/// Complete an inspection lot created during goods receipt.
///
/// If `passed` is true:
///   - Set lot status to COMPLETED
///   - Release the reserved (quality-held) quantity in mm_plant_stock
///
/// If `passed` is false:
///   - Set lot status to COMPLETED
///   - Create a quality notification for the failed inspection
///   - Keep the reserved quantity (stock remains on hold)
pub async fn complete_inspection_lot(
    pool: &PgPool,
    lot_id: Uuid,
    input: CompleteInspectionLot,
    user_id: Uuid,
) -> Result<InspectionLot, AppError> {
    let lot = repositories::get_inspection_lot(pool, lot_id).await?;

    // Only lots in CREATED status can be completed
    if lot.status != "CREATED" {
        return Err(AppError::Validation(format!(
            "Inspection lot is already in status '{}', cannot complete",
            lot.status
        )));
    }

    // Update lot status to COMPLETED
    let updated_lot = repositories::update_inspection_lot_status(pool, lot_id, "COMPLETED").await?;

    if input.passed {
        // Quality passed: release the reserved quantity held for QM inspection.
        // Look up the warehouse_id from the material movement that was created during goods receipt.
        // The inspection lot stores reference_type=PURCHASE_ORDER and reference_id=po_id.
        // We find the corresponding plant stock entries and release the held quantity.
        release_qm_hold(pool, &lot).await?;
    } else {
        // Quality failed: create a quality notification. Stock remains on hold.
        let notif_input = CreateQualityNotification {
            notification_type: "QM_REJECT".to_string(),
            material_id: Some(lot.material_id),
            description: input.notes.unwrap_or_else(|| {
                format!(
                    "Quality inspection failed for lot {}, material held",
                    lot.lot_number
                )
            }),
            priority: Some("HIGH".to_string()),
            assigned_to: None,
        };
        create_quality_notification(pool, notif_input, user_id).await?;
    }

    Ok(updated_lot)
}

/// Release quality-hold reserved quantity for a given inspection lot.
/// Finds the warehouse from the goods-receipt material movement linked to the PO,
/// then decreases reserved_quantity on mm_plant_stock.
async fn release_qm_hold(pool: &PgPool, lot: &InspectionLot) -> Result<(), AppError> {
    // The inspection lot was created with reference_type=PURCHASE_ORDER, reference_id=po_id.
    // The goods-receipt movements for this PO+material carry the warehouse_id.
    if lot.reference_type.as_deref() != Some("PURCHASE_ORDER") || lot.reference_id.is_none() {
        // Not linked to a PO, release across all warehouses for this material
        sqlx::query(
            "UPDATE mm_plant_stock SET reserved_quantity = GREATEST(reserved_quantity - $2, 0), updated_at = NOW() \
             WHERE material_id = $1 AND reserved_quantity > 0",
        )
        .bind(lot.material_id)
        .bind(lot.planned_quantity)
        .execute(pool)
        .await?;
        return Ok(());
    }

    let po_id = lot.reference_id.unwrap();

    // Find all goods-receipt movements for this PO + material to get warehouse_id(s)
    let movements: Vec<(Option<Uuid>, Decimal)> = sqlx::query_as(
        "SELECT warehouse_id, quantity FROM mm_material_movements \
         WHERE reference_type = 'PURCHASE_ORDER' AND reference_id = $1 \
           AND material_id = $2 AND movement_type = 'GOODS_RECEIPT' \
         ORDER BY posted_at DESC",
    )
    .bind(po_id)
    .bind(lot.material_id)
    .fetch_all(pool)
    .await?;

    // Release reserved quantity per warehouse
    let mut remaining = lot.planned_quantity;
    for (warehouse_id, qty) in &movements {
        if remaining <= Decimal::ZERO {
            break;
        }
        let release_qty = if *qty <= remaining { *qty } else { remaining };
        crate::mm::repositories::release_stock(pool, lot.material_id, *warehouse_id, release_qty)
            .await?;
        remaining -= release_qty;
    }

    Ok(())
}
