use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::mm::models::CreateMaterialMovement;
use crate::pp::models::*;
use crate::pp::repositories;
use crate::shared::pagination::ListParams;
use crate::shared::status::{self, DocumentType};
use crate::shared::status_history;
use crate::shared::{AppError, PaginatedResponse};

pub async fn list_boms(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Bom>, AppError> {
    let total = repositories::count_boms(pool, params).await?;
    let data = repositories::list_boms(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_bom(pool: &PgPool, id: Uuid) -> Result<(Bom, Vec<BomItem>), AppError> {
    let bom = repositories::get_bom(pool, id).await?;
    let items = repositories::get_bom_items(pool, id).await?;
    Ok((bom, items))
}

pub async fn create_bom(pool: &PgPool, input: CreateBom) -> Result<Bom, AppError> {
    let bom_number = repositories::next_number(pool, "BOM").await?;
    repositories::create_bom(pool, &bom_number, &input).await
}

pub async fn list_routings(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Routing>, AppError> {
    let total = repositories::count_routings(pool, params).await?;
    let data = repositories::list_routings(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_routing(
    pool: &PgPool,
    id: Uuid,
) -> Result<(Routing, Vec<RoutingOperation>), AppError> {
    let routing = repositories::get_routing(pool, id).await?;
    let operations = repositories::get_routing_operations(pool, id).await?;
    Ok((routing, operations))
}

pub async fn create_routing(pool: &PgPool, input: CreateRouting) -> Result<Routing, AppError> {
    let routing_number = repositories::next_number(pool, "BOM").await?;
    repositories::create_routing(pool, &routing_number, &input).await
}

pub async fn list_production_orders(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<ProductionOrder>, AppError> {
    let total = repositories::count_production_orders(pool, params).await?;
    let data = repositories::list_production_orders(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_production_order(pool: &PgPool, id: Uuid) -> Result<ProductionOrder, AppError> {
    repositories::get_production_order(pool, id).await
}

pub async fn create_production_order(
    pool: &PgPool,
    input: CreateProductionOrder,
    user_id: Uuid,
) -> Result<ProductionOrder, AppError> {
    let order_number = repositories::next_number(pool, "PRD").await?;
    repositories::create_production_order(pool, &order_number, &input, user_id).await
}

/// Update production order status with PP->MM integration:
/// - RELEASED: check BOM component stock availability
/// - IN_PROGRESS: issue BOM components from inventory (GOODS_ISSUE)
/// - COMPLETED: receive finished goods into inventory (GOODS_RECEIPT)
pub async fn update_production_order_status(
    pool: &PgPool,
    id: Uuid,
    input: UpdateProductionOrderStatus,
    user_id: Uuid,
) -> Result<ProductionOrder, AppError> {
    let mut tx = pool.begin().await?;

    let order = repositories::get_production_order_on_conn(&mut *tx, id).await?;
    status::validate_transition(DocumentType::ProductionOrder, &order.status, &input.status)?;

    let bom_items = repositories::get_bom_items_for_order_on_conn(&mut *tx, order.bom_id).await?;

    match input.status.as_str() {
        "RELEASED" => {
            // Check stock availability for all BOM components (across all warehouses)
            for bom_item in &bom_items {
                let required_qty = bom_item.quantity * order.planned_quantity;
                let total: Option<(Decimal,)> = sqlx::query_as(
                    "SELECT COALESCE(SUM(quantity - reserved_quantity), 0) FROM mm_plant_stock WHERE material_id = $1"
                )
                .bind(bom_item.component_material_id)
                .fetch_optional(&mut *tx).await?;

                let available = total.map(|t| t.0).unwrap_or_default();
                if available < required_qty {
                    return Err(AppError::Validation(format!(
                        "Insufficient stock for component material {}: available={}, required={}",
                        bom_item.component_material_id, available, required_qty
                    )));
                }
            }
        }
        "IN_PROGRESS" => {
            // Issue BOM components from inventory
            for bom_item in &bom_items {
                let issue_qty = bom_item.quantity * order.planned_quantity;
                let doc_number =
                    crate::mm::repositories::next_number_on_conn(&mut *tx, "MVT").await?;
                let movement = CreateMaterialMovement {
                    movement_type: "GOODS_ISSUE".to_string(),
                    material_id: bom_item.component_material_id,
                    warehouse_id: None,
                    quantity: issue_qty,
                    uom_id: bom_item.uom_id,
                    reference_type: Some("PRODUCTION_ORDER".to_string()),
                    reference_id: Some(id),
                };
                crate::mm::repositories::create_material_movement_on_conn(
                    &mut *tx,
                    &doc_number,
                    &movement,
                    user_id,
                )
                .await?;
            }
        }
        "COMPLETED" => {
            // Receive finished goods into inventory
            let actual_qty = input.actual_quantity.unwrap_or(order.planned_quantity);
            let doc_number = crate::mm::repositories::next_number_on_conn(&mut *tx, "MVT").await?;
            let movement = CreateMaterialMovement {
                movement_type: "GOODS_RECEIPT".to_string(),
                material_id: order.material_id,
                warehouse_id: None,
                quantity: actual_qty,
                uom_id: order.uom_id,
                reference_type: Some("PRODUCTION_ORDER".to_string()),
                reference_id: Some(id),
            };
            crate::mm::repositories::create_material_movement_on_conn(
                &mut *tx,
                &doc_number,
                &movement,
                user_id,
            )
            .await?;

            // GAP 7: Create QM inspection lot for production output
            let inspection_input = crate::qm::models::CreateInspectionLot {
                material_id: order.material_id,
                reference_type: Some("PRODUCTION_ORDER".to_string()),
                reference_id: Some(id),
                inspection_type: Some("PRODUCTION_OUTPUT".to_string()),
                planned_quantity: actual_qty,
            };
            crate::qm::services::create_inspection_lot_in_tx(&mut *tx, inspection_input, user_id)
                .await?;

            // Apply quality hold on received quantity
            crate::mm::repositories::reserve_stock_on_conn(
                &mut *tx,
                order.material_id,
                None,
                actual_qty,
            )
            .await?;
        }
        _ => {} // CANCELLED, CLOSED - no inventory impact
    }

    // GAP 5: Calculate production cost BEFORE commit so FI entries are within the transaction
    let mut total_material_cost = Decimal::ZERO;
    let mut total_labor_cost = Decimal::ZERO;
    let mut total_production_cost = Decimal::ZERO;
    let today = Utc::now().date_naive();

    if input.status == "COMPLETED" {
        let actual_qty = input.actual_quantity.unwrap_or(order.planned_quantity);

        // Calculate material cost: sum of (BOM component qty * actual qty * latest PO unit price)
        for bom_item in &bom_items {
            let consumed_qty = bom_item.quantity * actual_qty;
            // Look up the latest purchase price for this component material
            let latest_price: Option<(Decimal,)> = sqlx::query_as(
                r#"SELECT poi.unit_price
                   FROM mm_purchase_order_items poi
                   JOIN mm_purchase_orders po ON po.id = poi.purchase_order_id
                   WHERE poi.material_id = $1 AND po.status IN ('RECEIVED', 'PARTIALLY_RECEIVED', 'RELEASED')
                   ORDER BY po.created_at DESC
                   LIMIT 1"#,
            )
            .bind(bom_item.component_material_id)
            .fetch_optional(&mut *tx)
            .await?;

            if let Some((unit_price,)) = latest_price {
                total_material_cost += consumed_qty * unit_price;
            }
        }

        // Calculate labor cost from routing operations (if routing exists)
        if let Some(routing_id) = order.routing_id {
            let operations =
                repositories::get_routing_operations_on_conn(&mut *tx, routing_id).await?;
            // Standard labor rate: 50 currency units per hour (configurable in future)
            let labor_rate_per_minute = Decimal::new(50, 0) / Decimal::new(60, 0);
            for op in &operations {
                let total_minutes = Decimal::from(op.setup_time_minutes)
                    + Decimal::from(op.run_time_minutes) * actual_qty;
                total_labor_cost += total_minutes * labor_rate_per_minute;
            }
        }

        total_production_cost = total_material_cost + total_labor_cost;

        // GAP 5: Create FI journal entry for production settlement
        if total_production_cost > Decimal::ZERO {
            let fg_account = sqlx::query_as::<_, (Uuid,)>(
                "SELECT id FROM fi_accounts WHERE account_number = '1410'",
            )
            .fetch_optional(&mut *tx)
            .await?;
            let mfg_overhead = sqlx::query_as::<_, (Uuid,)>(
                "SELECT id FROM fi_accounts WHERE account_number = '5200'",
            )
            .fetch_optional(&mut *tx)
            .await?;
            let company_code =
                sqlx::query_as::<_, (Uuid,)>("SELECT id FROM fi_company_codes LIMIT 1")
                    .fetch_optional(&mut *tx)
                    .await?;

            if let (Some((fg_id,)), Some((mfg_id,)), Some((cc_id,))) =
                (fg_account, mfg_overhead, company_code)
            {
                let je_input = crate::fi::models::CreateJournalEntry {
                    company_code_id: cc_id,
                    posting_date: today,
                    document_date: today,
                    reference: Some(format!("PP-SETTLE:{}", order.order_number)),
                    description: Some(format!("Production settlement for {}", order.order_number)),
                    items: vec![
                        crate::fi::models::CreateJournalItem {
                            account_id: fg_id,
                            debit_amount: total_production_cost,
                            credit_amount: Decimal::ZERO,
                            cost_center_id: None,
                            description: Some("Finished goods from production".to_string()),
                        },
                        crate::fi::models::CreateJournalItem {
                            account_id: mfg_id,
                            debit_amount: Decimal::ZERO,
                            credit_amount: total_production_cost,
                            cost_center_id: None,
                            description: Some("Manufacturing overhead absorbed".to_string()),
                        },
                    ],
                };
                let je =
                    crate::fi::services::create_journal_entry_in_tx(&mut *tx, je_input, user_id)
                        .await?;
                crate::fi::services::post_journal_entry_in_tx(&mut *tx, je.id, user_id).await?;
            }
        }
    }

    let result = repositories::update_production_order_status_on_conn(&mut *tx, id, &input).await?;
    status_history::record_transition(
        &mut *tx,
        &DocumentType::ProductionOrder,
        id,
        Some(&order.status),
        &input.status,
        user_id,
        None,
    )
    .await?;

    tx.commit().await?;

    // PP -> CO auto-posting + audit AFTER commit (best effort, uses pool)
    if input.status == "COMPLETED" {
        if total_production_cost > Decimal::ZERO {
            let co_description = format!(
                "PP auto-post: Production order {} completed - materials: {}, labor: {}",
                order.order_number, total_material_cost, total_labor_cost
            );

            if let Err(e) = crate::co::services::auto_post_production_cost(
                pool,
                id,
                None, // Use default CC-PRODUCTION cost center
                total_production_cost,
                today,
                &co_description,
            )
            .await
            {
                tracing::warn!(
                    "PP->CO auto-posting failed for production order {}: {}",
                    order.order_number,
                    e
                );
            }
        }

        let actual_qty = input.actual_quantity.unwrap_or(order.planned_quantity);

        // Audit log for production order completion (best effort, after commit)
        crate::shared::audit::log_change(
            pool,
            "pp_production_orders",
            id,
            "COMPLETED",
            None,
            Some(serde_json::json!({
                "order_number": &order.order_number,
                "actual_quantity": actual_qty.to_string(),
                "material_cost": total_material_cost.to_string(),
                "labor_cost": total_labor_cost.to_string(),
                "total_production_cost": total_production_cost.to_string(),
            })),
            Some(user_id),
        )
        .await
        .ok();
    }

    Ok(result)
}
