use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::fi::models::{CreateApInvoice, CreateJournalEntry, CreateJournalItem};
use crate::mm::models::*;
use crate::mm::repositories;
use crate::qm::models::CreateInspectionLot;
use crate::shared::pagination::ListParams;
use crate::shared::status::{self, DocumentType};
use crate::shared::status_history;
use crate::shared::{AppError, PaginatedResponse};

pub async fn list_uoms(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Uom>, AppError> {
    let total = repositories::count_uoms(pool, params).await?;
    let data = repositories::list_uoms(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn create_uom(pool: &PgPool, input: CreateUom) -> Result<Uom, AppError> {
    repositories::create_uom(pool, &input).await
}

pub async fn list_material_groups(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<MaterialGroup>, AppError> {
    let total = repositories::count_material_groups(pool, params).await?;
    let data = repositories::list_material_groups(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn create_material_group(
    pool: &PgPool,
    input: CreateMaterialGroup,
) -> Result<MaterialGroup, AppError> {
    repositories::create_material_group(pool, &input).await
}

pub async fn list_materials(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Material>, AppError> {
    let total = repositories::count_materials(pool, params).await?;
    let data = repositories::list_materials(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_material(pool: &PgPool, id: Uuid) -> Result<Material, AppError> {
    repositories::get_material(pool, id).await
}

pub async fn create_material(pool: &PgPool, input: CreateMaterial) -> Result<Material, AppError> {
    let mat_number = repositories::next_number(pool, "MAT").await?;
    repositories::create_material(pool, &mat_number, &input).await
}

pub async fn update_material(
    pool: &PgPool,
    id: Uuid,
    input: UpdateMaterial,
) -> Result<Material, AppError> {
    repositories::update_material(pool, id, &input).await
}

pub async fn list_vendors(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Vendor>, AppError> {
    let total = repositories::count_vendors(pool, params).await?;
    let data = repositories::list_vendors(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_vendor(pool: &PgPool, id: Uuid) -> Result<Vendor, AppError> {
    repositories::get_vendor(pool, id).await
}

pub async fn create_vendor(pool: &PgPool, input: CreateVendor) -> Result<Vendor, AppError> {
    let vendor_number = repositories::next_number(pool, "VND").await?;
    repositories::create_vendor(pool, &vendor_number, &input).await
}

pub async fn update_vendor(
    pool: &PgPool,
    id: Uuid,
    input: UpdateVendor,
) -> Result<Vendor, AppError> {
    repositories::update_vendor(pool, id, &input).await
}

pub async fn list_plant_stock(
    pool: &PgPool,
    warehouse_id: Option<Uuid>,
    params: &ListParams,
) -> Result<Vec<PlantStock>, AppError> {
    repositories::list_plant_stock(pool, warehouse_id, params).await
}

pub async fn list_material_movements(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<MaterialMovement>, AppError> {
    let total = repositories::count_material_movements(pool, params).await?;
    let data = repositories::list_material_movements(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn create_material_movement(
    pool: &PgPool,
    input: CreateMaterialMovement,
    user_id: Uuid,
) -> Result<MaterialMovement, AppError> {
    let doc_number = repositories::next_number(pool, "MVT").await?;
    repositories::create_material_movement(pool, &doc_number, &input, user_id).await
}

// --- Goods Receipts (GRN) ---
pub async fn list_goods_receipts(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<GoodsReceipt>, AppError> {
    let total = repositories::count_goods_receipts(pool, params).await?;
    let data = repositories::list_goods_receipts(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_goods_receipt(
    pool: &PgPool,
    id: Uuid,
) -> Result<(GoodsReceipt, Vec<GoodsReceiptItem>), AppError> {
    let grn = repositories::get_goods_receipt(pool, id).await?;
    let items = repositories::get_goods_receipt_items(pool, id).await?;
    Ok((grn, items))
}

pub async fn create_goods_receipt(
    pool: &PgPool,
    input: CreateGoodsReceipt,
    user_id: Uuid,
) -> Result<GoodsReceipt, AppError> {
    let grn_number = repositories::next_number(pool, "GRN").await?;
    repositories::create_goods_receipt(pool, &grn_number, &input, user_id).await
}

/// Confirm a GRN: DRAFT -> CONFIRMED. Updates plant stock and PO received quantities.
pub async fn confirm_goods_receipt(
    pool: &PgPool,
    grn_id: Uuid,
    user_id: Uuid,
) -> Result<GoodsReceipt, AppError> {
    let mut tx = pool.begin().await?;

    let grn = repositories::get_goods_receipt_on_conn(&mut *tx, grn_id).await?;
    if grn.status != "DRAFT" {
        return Err(AppError::Validation(format!(
            "Cannot confirm GRN in status '{}'",
            grn.status
        )));
    }

    let items = repositories::get_goods_receipt_items_on_conn(&mut *tx, grn_id).await?;

    for item in &items {
        let accepted_qty = item.received_quantity - item.rejected_quantity.unwrap_or(Decimal::ZERO);
        if accepted_qty <= Decimal::ZERO {
            continue;
        }

        // Create GOODS_RECEIPT movement
        let doc_number = repositories::next_number_on_conn(&mut *tx, "MVT").await?;
        let movement = CreateMaterialMovement {
            movement_type: "GOODS_RECEIPT".to_string(),
            material_id: item.material_id,
            warehouse_id: grn.warehouse_id,
            quantity: accepted_qty,
            uom_id: item.uom_id,
            reference_type: Some("GRN".to_string()),
            reference_id: Some(grn_id),
        };
        repositories::create_material_movement_on_conn(&mut *tx, &doc_number, &movement, user_id)
            .await?;

        // Record in unified stock movements ledger
        sqlx::query(
            "INSERT INTO mm_stock_movements (material_id, warehouse_id, movement_type, quantity, reference_type, reference_id, batch_number, notes, created_by) \
             VALUES ($1, $2, 'GOODS_RECEIPT', $3, 'GRN', $4, $5, $6, $7)"
        )
        .bind(item.material_id)
        .bind(grn.warehouse_id)
        .bind(accepted_qty)
        .bind(grn_id)
        .bind(&item.batch_number)
        .bind(&item.notes)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        // Update PO item received quantity if linked to a PO
        if let Some(po_item_id) = item.po_item_id {
            repositories::update_po_item_received_on_conn(&mut *tx, po_item_id, accepted_qty)
                .await?;
        }
    }

    // Update GRN status
    let confirmed = sqlx::query_as::<_, GoodsReceipt>(
        "UPDATE mm_goods_receipts SET status = 'CONFIRMED', updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(grn_id)
    .fetch_one(&mut *tx)
    .await?;

    // If linked to a PO, update PO status based on received quantities
    if let Some(po_id) = grn.purchase_order_id {
        let po_items = repositories::get_purchase_order_items_on_conn(&mut *tx, po_id).await?;
        let all_received = po_items.iter().all(|i| i.received_quantity >= i.quantity);
        let any_received = po_items.iter().any(|i| i.received_quantity > Decimal::ZERO);

        if all_received {
            let po = repositories::get_purchase_order_on_conn(&mut *tx, po_id).await?;
            if po.status == "RELEASED" || po.status == "PARTIALLY_RECEIVED" {
                repositories::update_po_status_on_conn(&mut *tx, po_id, "RECEIVED").await?;
            }
        } else if any_received {
            let po = repositories::get_purchase_order_on_conn(&mut *tx, po_id).await?;
            if po.status == "RELEASED" {
                repositories::update_po_status_on_conn(&mut *tx, po_id, "PARTIALLY_RECEIVED")
                    .await?;
            }
        }
    }

    tx.commit().await?;
    Ok(confirmed)
}

// --- Stock Reservations ---
pub async fn list_stock_reservations(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<StockReservation>, AppError> {
    let total = repositories::count_stock_reservations(pool, params).await?;
    let data = repositories::list_stock_reservations(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

// --- Stock Movements ---
pub async fn list_stock_movements(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<StockMovement>, AppError> {
    let total = repositories::count_stock_movements(pool, params).await?;
    let data = repositories::list_stock_movements(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn list_purchase_orders(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<PurchaseOrder>, AppError> {
    let total = repositories::count_purchase_orders(pool, params).await?;
    let data = repositories::list_purchase_orders(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_purchase_order(
    pool: &PgPool,
    id: Uuid,
) -> Result<(PurchaseOrder, Vec<PurchaseOrderItem>), AppError> {
    let po = repositories::get_purchase_order(pool, id).await?;
    let items = repositories::get_purchase_order_items(pool, id).await?;
    Ok((po, items))
}

pub async fn create_purchase_order(
    pool: &PgPool,
    input: CreatePurchaseOrder,
    user_id: Uuid,
) -> Result<PurchaseOrder, AppError> {
    let po_number = repositories::next_number(pool, "PO").await?;
    repositories::create_purchase_order(pool, &po_number, &input, user_id).await
}

/// Release a purchase order: DRAFT -> RELEASED
pub async fn release_purchase_order(
    pool: &PgPool,
    po_id: Uuid,
    user_id: Uuid,
) -> Result<PurchaseOrder, AppError> {
    let po = repositories::get_purchase_order(pool, po_id).await?;
    status::validate_transition(DocumentType::PurchaseOrder, &po.status, "RELEASED")?;
    let result = repositories::update_po_status(pool, po_id, "RELEASED").await?;
    status_history::record_transition(
        pool,
        &DocumentType::PurchaseOrder,
        po_id,
        Some(&po.status),
        "RELEASED",
        user_id,
        None,
    )
    .await?;
    Ok(result)
}

/// Receive goods for a purchase order (MM->FI integration).
/// Creates GOODS_RECEIPT movements, updates PO item received quantities,
/// auto-creates FI Journal Entry (DR Inventory, CR AP) and FI AP Invoice.
pub async fn receive_purchase_order(
    pool: &PgPool,
    po_id: Uuid,
    input: ReceivePurchaseOrder,
    user_id: Uuid,
) -> Result<PurchaseOrder, AppError> {
    let mut tx = pool.begin().await?;

    let po = repositories::get_purchase_order_on_conn(&mut *tx, po_id).await?;

    // Validate PO status allows receiving
    if po.status != "RELEASED" && po.status != "PARTIALLY_RECEIVED" {
        return Err(AppError::Validation(format!(
            "Cannot receive goods for PO in status '{}'",
            po.status
        )));
    }

    let po_items = repositories::get_purchase_order_items_on_conn(&mut *tx, po_id).await?;
    let mut total_received_value = Decimal::ZERO;

    // Track received items per material for QM inspection lot creation
    struct ReceivedMaterialInfo {
        material_id: Uuid,
        warehouse_id: Option<Uuid>,
        quantity: Decimal,
    }
    let mut received_materials: Vec<ReceivedMaterialInfo> = Vec::new();

    for recv_item in &input.items {
        // Find matching PO item
        let po_item = po_items
            .iter()
            .find(|i| i.id == recv_item.po_item_id)
            .ok_or_else(|| {
                AppError::NotFound(format!("PO item {} not found", recv_item.po_item_id))
            })?;

        // Check receive quantity doesn't exceed ordered
        let remaining = po_item.quantity - po_item.received_quantity;
        if recv_item.quantity > remaining {
            return Err(AppError::Validation(format!(
                "Receive quantity {} exceeds remaining {} for item {}",
                recv_item.quantity, remaining, po_item.line_number
            )));
        }

        // Create GOODS_RECEIPT movement (triggers inventory update)
        let doc_number = repositories::next_number_on_conn(&mut *tx, "MVT").await?;
        let movement = CreateMaterialMovement {
            movement_type: "GOODS_RECEIPT".to_string(),
            material_id: po_item.material_id,
            warehouse_id: recv_item.warehouse_id,
            quantity: recv_item.quantity,
            uom_id: po_item.uom_id,
            reference_type: Some("PURCHASE_ORDER".to_string()),
            reference_id: Some(po_id),
        };
        repositories::create_material_movement_on_conn(&mut *tx, &doc_number, &movement, user_id)
            .await?;

        // Update PO item received quantity
        repositories::update_po_item_received_on_conn(&mut *tx, po_item.id, recv_item.quantity)
            .await?;

        total_received_value += recv_item.quantity * po_item.unit_price;

        // Track for QM quality hold
        received_materials.push(ReceivedMaterialInfo {
            material_id: po_item.material_id,
            warehouse_id: recv_item.warehouse_id,
            quantity: recv_item.quantity,
        });
    }

    // --- QM Integration: Auto-create inspection lots and apply quality hold ---
    // For each received material, create an inspection lot and reserve the received
    // quantity in mm_plant_stock so it cannot be consumed until QM completes inspection.
    for rm in &received_materials {
        // Create QM inspection lot linked to this PO
        let inspection_input = CreateInspectionLot {
            material_id: rm.material_id,
            reference_type: Some("PURCHASE_ORDER".to_string()),
            reference_id: Some(po_id),
            inspection_type: Some("INCOMING".to_string()),
            planned_quantity: rm.quantity,
        };
        crate::qm::services::create_inspection_lot_in_tx(&mut *tx, inspection_input, user_id)
            .await?;

        // Apply quality hold: reserve the received quantity so it is not available
        // for consumption until the inspection lot is completed and passed.
        repositories::reserve_stock_on_conn(&mut *tx, rm.material_id, rm.warehouse_id, rm.quantity)
            .await?;
    }

    // Determine new PO status
    let updated_items = repositories::get_purchase_order_items_on_conn(&mut *tx, po_id).await?;
    let all_received = updated_items
        .iter()
        .all(|i| i.received_quantity >= i.quantity);
    let any_received = updated_items
        .iter()
        .any(|i| i.received_quantity > Decimal::ZERO);

    let new_status = if all_received {
        "RECEIVED"
    } else if any_received {
        "PARTIALLY_RECEIVED"
    } else {
        &po.status
    };

    let updated_po = if new_status != po.status {
        status::validate_transition(DocumentType::PurchaseOrder, &po.status, new_status)?;
        let result = repositories::update_po_status_on_conn(&mut *tx, po_id, new_status).await?;
        status_history::record_transition(
            &mut *tx,
            &DocumentType::PurchaseOrder,
            po_id,
            Some(&po.status),
            new_status,
            user_id,
            None,
        )
        .await?;
        result
    } else {
        po
    };

    // Auto-create FI Journal Entry: DR Inventory (1300), CR Accounts Payable (2100)
    // Look up account IDs by account_number
    let inventory_account: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM fi_accounts WHERE account_number = '1300'")
            .fetch_optional(&mut *tx)
            .await?;

    let ap_account: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM fi_accounts WHERE account_number = '2100'")
            .fetch_optional(&mut *tx)
            .await?;

    let company_code: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM fi_company_codes LIMIT 1")
        .fetch_optional(&mut *tx)
        .await?;

    if let (Some((inv_id,)), Some((ap_id,)), Some((cc_id,))) =
        (inventory_account, ap_account, company_code)
    {
        let today = Utc::now().date_naive();
        let je_input = CreateJournalEntry {
            company_code_id: cc_id,
            posting_date: today,
            document_date: today,
            reference: Some(format!("PO-RECV:{}", updated_po.po_number)),
            description: Some(format!("Goods receipt for PO {}", updated_po.po_number)),
            items: vec![
                CreateJournalItem {
                    account_id: inv_id,
                    debit_amount: total_received_value,
                    credit_amount: Decimal::ZERO,
                    cost_center_id: None,
                    description: Some("Inventory increase from PO receipt".to_string()),
                },
                CreateJournalItem {
                    account_id: ap_id,
                    debit_amount: Decimal::ZERO,
                    credit_amount: total_received_value,
                    cost_center_id: None,
                    description: Some("AP liability from PO receipt".to_string()),
                },
            ],
        };
        let je =
            crate::fi::services::create_journal_entry_in_tx(&mut *tx, je_input, user_id).await?;
        crate::fi::services::post_journal_entry_in_tx(&mut *tx, je.id, user_id).await?;

        // Create AP Invoice
        let ap_input = CreateApInvoice {
            vendor_id: Some(updated_po.vendor_id),
            invoice_date: today,
            due_date: today + chrono::Duration::days(30),
            total_amount: total_received_value,
        };
        crate::fi::services::create_ap_invoice_in_tx(&mut *tx, ap_input).await?;
    }

    // Audit log inside transaction
    crate::shared::audit::log_change(
        &mut *tx,
        "mm_purchase_orders",
        po_id,
        "GOODS_RECEIPT",
        None,
        Some(serde_json::json!({
            "po_number": &updated_po.po_number,
            "status": &updated_po.status,
            "total_received_value": total_received_value.to_string(),
        })),
        Some(user_id),
    )
    .await
    .ok();

    tx.commit().await?;

    // MM -> CO auto-posting AFTER commit (best effort, uses pool)
    let today = Utc::now().date_naive();
    let co_description = format!(
        "MM auto-post: Goods receipt for PO {} - total value {}",
        updated_po.po_number, total_received_value
    );
    if let Err(e) = crate::co::services::auto_post_procurement_cost(
        pool,
        po_id,
        None, // Use default CC-PROCUREMENT cost center
        total_received_value,
        today,
        &co_description,
    )
    .await
    {
        tracing::warn!(
            "MM->CO auto-posting failed for PO {}: {}",
            updated_po.po_number,
            e
        );
    }

    Ok(updated_po)
}
