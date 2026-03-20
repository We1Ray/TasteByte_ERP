use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::fi::models::{CreateArInvoice, CreateJournalEntry, CreateJournalItem};
use crate::mm::models::CreateMaterialMovement;
use crate::sd::models::*;
use crate::sd::repositories;
use crate::shared::pagination::ListParams;
use crate::shared::status::{self, DocumentType};
use crate::shared::status_history;
use crate::shared::{AppError, PaginatedResponse};

pub async fn list_customers(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Customer>, AppError> {
    let total = repositories::count_customers(pool, params).await?;
    let data = repositories::list_customers(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_customer(pool: &PgPool, id: Uuid) -> Result<Customer, AppError> {
    repositories::get_customer(pool, id).await
}

pub async fn create_customer(pool: &PgPool, input: CreateCustomer) -> Result<Customer, AppError> {
    let cust_number = repositories::next_number(pool, "CUST").await?;
    repositories::create_customer(pool, &cust_number, &input).await
}

pub async fn update_customer(
    pool: &PgPool,
    id: Uuid,
    input: UpdateCustomer,
) -> Result<Customer, AppError> {
    repositories::update_customer(pool, id, &input).await
}

pub async fn list_sales_orders(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<SalesOrder>, AppError> {
    let total = repositories::count_sales_orders(pool, params).await?;
    let data = repositories::list_sales_orders(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

pub async fn get_sales_order(
    pool: &PgPool,
    id: Uuid,
) -> Result<(SalesOrder, Vec<SalesOrderItem>), AppError> {
    let so = repositories::get_sales_order(pool, id).await?;
    let items = repositories::get_sales_order_items(pool, id).await?;
    Ok((so, items))
}

pub async fn create_sales_order(
    pool: &PgPool,
    input: CreateSalesOrder,
    user_id: Uuid,
) -> Result<SalesOrder, AppError> {
    let order_number = repositories::next_number(pool, "SO").await?;
    repositories::create_sales_order(pool, &order_number, &input, user_id).await
}

/// Confirm a sales order: validate stock availability and reserve inventory.
pub async fn confirm_sales_order(
    pool: &PgPool,
    so_id: Uuid,
    user_id: Uuid,
) -> Result<SalesOrder, AppError> {
    let mut tx = pool.begin().await?;

    let so = repositories::get_sales_order_on_conn(&mut *tx, so_id).await?;
    status::validate_transition(DocumentType::SalesOrder, &so.status, "CONFIRMED")?;

    // Credit limit enforcement
    let customer = repositories::get_customer_on_conn(&mut *tx, so.customer_id).await?;
    if customer.credit_limit > Decimal::ZERO {
        // Sum of all open AR invoices for this customer
        let (open_ar,): (Decimal,) = sqlx::query_as(
            "SELECT COALESCE(SUM(total_amount - paid_amount), 0) \
             FROM fi_ar_invoices WHERE customer_id = $1 AND status NOT IN ('PAID', 'CANCELLED')",
        )
        .bind(customer.id)
        .fetch_one(&mut *tx)
        .await?;

        // Sum of all pending/confirmed SO total_amounts (excluding cancelled and current SO)
        let (pending_so,): (Decimal,) = sqlx::query_as(
            "SELECT COALESCE(SUM(total_amount), 0) \
             FROM sd_sales_orders WHERE customer_id = $1 AND status NOT IN ('CANCELLED', 'COMPLETED') AND id != $2",
        )
        .bind(customer.id)
        .bind(so_id)
        .fetch_one(&mut *tx)
        .await?;

        let total_exposure = open_ar + pending_so + so.total_amount;
        if total_exposure > customer.credit_limit {
            return Err(AppError::Validation(
                "Customer credit limit exceeded".to_string(),
            ));
        }
    }

    let items = repositories::get_sales_order_items_on_conn(&mut *tx, so_id).await?;

    // Reserve stock for each item
    for item in &items {
        crate::mm::repositories::reserve_stock_on_conn(
            &mut *tx,
            item.material_id,
            None,
            item.quantity,
        )
        .await?;
    }

    // Update order status
    let result =
        repositories::update_sales_order_status_on_conn(&mut *tx, so_id, "CONFIRMED").await?;
    status_history::record_transition(
        &mut *tx,
        &DocumentType::SalesOrder,
        so_id,
        Some(&so.status),
        "CONFIRMED",
        user_id,
        None,
    )
    .await?;

    tx.commit().await?;
    Ok(result)
}

pub async fn list_deliveries(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<Delivery>, AppError> {
    let total = repositories::count_deliveries(pool, params).await?;
    let data = repositories::list_deliveries(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

/// Create delivery with SD->MM integration: auto-creates GOODS_ISSUE movements,
/// updates delivered quantities, and releases reserved stock.
pub async fn create_delivery(
    pool: &PgPool,
    input: CreateDelivery,
    user_id: Uuid,
) -> Result<Delivery, AppError> {
    let mut tx = pool.begin().await?;

    let so = repositories::get_sales_order_on_conn(&mut *tx, input.sales_order_id).await?;

    // Sales order must be CONFIRMED or PARTIALLY_DELIVERED
    if so.status != "CONFIRMED" && so.status != "PARTIALLY_DELIVERED" {
        return Err(AppError::Validation(format!(
            "Cannot create delivery for order in status '{}'",
            so.status
        )));
    }

    let so_items =
        repositories::get_sales_order_items_on_conn(&mut *tx, input.sales_order_id).await?;

    // Validate delivery quantities
    for del_item in &input.items {
        let so_item = so_items
            .iter()
            .find(|i| i.id == del_item.sales_order_item_id)
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Sales order item {} not found",
                    del_item.sales_order_item_id
                ))
            })?;

        let remaining = so_item.quantity - so_item.delivered_quantity;
        if del_item.quantity > remaining {
            return Err(AppError::Validation(format!(
                "Delivery quantity {} exceeds remaining {} for item {}",
                del_item.quantity, remaining, so_item.line_number
            )));
        }
    }

    // Create the delivery record
    let delivery_number = repositories::next_number_on_conn(&mut *tx, "DO").await?;
    let delivery =
        repositories::create_delivery_on_conn(&mut *tx, &delivery_number, &input).await?;
    status_history::record_transition(
        &mut *tx,
        &DocumentType::Delivery,
        delivery.id,
        None,
        "CREATED",
        user_id,
        None,
    )
    .await?;

    // For each delivery item: create GOODS_ISSUE movement, update delivered_quantity, release reserved stock
    for del_item in &input.items {
        let so_item = so_items
            .iter()
            .find(|i| i.id == del_item.sales_order_item_id)
            .unwrap();

        // Create GOODS_ISSUE material movement (triggers inventory deduction)
        let doc_number = crate::mm::repositories::next_number_on_conn(&mut *tx, "MVT").await?;
        let movement = CreateMaterialMovement {
            movement_type: "GOODS_ISSUE".to_string(),
            material_id: so_item.material_id,
            warehouse_id: None,
            quantity: del_item.quantity,
            uom_id: so_item.uom_id,
            reference_type: Some("SALES_ORDER".to_string()),
            reference_id: Some(input.sales_order_id),
        };
        crate::mm::repositories::create_material_movement_on_conn(
            &mut *tx,
            &doc_number,
            &movement,
            user_id,
        )
        .await?;

        // GAP 8: Record warehouse_id on delivery item for WM traceability
        // Look up which warehouse the goods issue came from (highest stock)
        let warehouse_id: Option<Uuid> = sqlx::query_as::<_, (Option<Uuid>,)>(
            "SELECT warehouse_id FROM mm_plant_stock \
             WHERE material_id = $1 AND warehouse_id IS NOT NULL \
             ORDER BY quantity DESC LIMIT 1",
        )
        .bind(so_item.material_id)
        .fetch_optional(&mut *tx)
        .await?
        .and_then(|r| r.0);

        if let Some(wh_id) = warehouse_id {
            sqlx::query(
                "UPDATE sd_delivery_items SET warehouse_id = $1 \
                 WHERE delivery_id = $2 AND sales_order_item_id = $3",
            )
            .bind(wh_id)
            .bind(delivery.id)
            .bind(del_item.sales_order_item_id)
            .execute(&mut *tx)
            .await?;
        }

        // Update delivered quantity on SO item
        repositories::update_so_item_delivered_on_conn(&mut *tx, so_item.id, del_item.quantity)
            .await?;

        // Release reserved stock
        crate::mm::repositories::release_stock_on_conn(
            &mut *tx,
            so_item.material_id,
            None,
            del_item.quantity,
        )
        .await?;
    }

    // Update sales order status based on delivery completeness
    let updated_items =
        repositories::get_sales_order_items_on_conn(&mut *tx, input.sales_order_id).await?;
    let all_delivered = updated_items
        .iter()
        .all(|i| i.delivered_quantity >= i.quantity);
    let any_delivered = updated_items
        .iter()
        .any(|i| i.delivered_quantity > Decimal::ZERO);

    if all_delivered {
        status::validate_transition(DocumentType::SalesOrder, &so.status, "DELIVERED")?;
        repositories::update_sales_order_status_on_conn(
            &mut *tx,
            input.sales_order_id,
            "DELIVERED",
        )
        .await?;
        status_history::record_transition(
            &mut *tx,
            &DocumentType::SalesOrder,
            input.sales_order_id,
            Some(&so.status),
            "DELIVERED",
            user_id,
            None,
        )
        .await?;
    } else if any_delivered && so.status == "CONFIRMED" {
        status::validate_transition(DocumentType::SalesOrder, &so.status, "PARTIALLY_DELIVERED")?;
        repositories::update_sales_order_status_on_conn(
            &mut *tx,
            input.sales_order_id,
            "PARTIALLY_DELIVERED",
        )
        .await?;
        status_history::record_transition(
            &mut *tx,
            &DocumentType::SalesOrder,
            input.sales_order_id,
            Some(&so.status),
            "PARTIALLY_DELIVERED",
            user_id,
            None,
        )
        .await?;
    }

    tx.commit().await?;
    Ok(delivery)
}

pub async fn get_delivery(
    pool: &PgPool,
    id: Uuid,
) -> Result<(Delivery, Vec<DeliveryItem>), AppError> {
    let delivery = repositories::get_delivery(pool, id).await?;
    let items = repositories::get_delivery_items(pool, id).await?;
    Ok((delivery, items))
}

/// Ship a delivery: CREATED -> SHIPPED
pub async fn ship_delivery(
    pool: &PgPool,
    delivery_id: Uuid,
    user_id: Uuid,
) -> Result<Delivery, AppError> {
    let delivery = repositories::get_delivery(pool, delivery_id).await?;
    if delivery.status != "CREATED" {
        return Err(AppError::Validation(format!(
            "Cannot ship delivery in status '{}'",
            delivery.status
        )));
    }

    let result = sqlx::query_as::<_, Delivery>(
        "UPDATE sd_deliveries SET status = 'SHIPPED', shipped_by = $2, shipped_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(delivery_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn list_sd_invoices(
    pool: &PgPool,
    params: &ListParams,
) -> Result<PaginatedResponse<SdInvoice>, AppError> {
    let total = repositories::count_sd_invoices(pool, params).await?;
    let data = repositories::list_sd_invoices(pool, params).await?;
    Ok(PaginatedResponse::from_list_params(data, total, params))
}

/// Create SD invoice with SD->FI integration: auto-creates FI Journal Entry
/// (DR Accounts Receivable, CR Revenue) and FI AR Invoice.
pub async fn create_sd_invoice(
    pool: &PgPool,
    input: CreateSdInvoice,
    user_id: Uuid,
) -> Result<SdInvoice, AppError> {
    let mut tx = pool.begin().await?;

    // Validate sales order exists
    let so = repositories::get_sales_order_on_conn(&mut *tx, input.sales_order_id).await?;

    // Validate: sales order must be delivered before invoicing
    let valid_statuses = ["DELIVERED", "PARTIALLY_DELIVERED", "COMPLETED", "INVOICED"];
    if !valid_statuses.contains(&so.status.as_str()) {
        return Err(AppError::Validation(
            "Sales order must be delivered before invoicing".to_string(),
        ));
    }

    // Validate: total invoiced amount + new invoice must not exceed SO total
    let (already_invoiced,): (Decimal,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_amount), 0) FROM sd_invoices WHERE sales_order_id = $1",
    )
    .bind(input.sales_order_id)
    .fetch_one(&mut *tx)
    .await?;

    if already_invoiced + input.total_amount > so.total_amount {
        return Err(AppError::Validation(
            "Invoice amount exceeds order total".to_string(),
        ));
    }

    // Create SD invoice
    let invoice_number = repositories::next_number_on_conn(&mut *tx, "INV").await?;
    let invoice =
        repositories::create_sd_invoice_on_conn(&mut *tx, &invoice_number, &input).await?;
    status_history::record_transition(
        &mut *tx,
        &DocumentType::Invoice,
        invoice.id,
        None,
        "CREATED",
        user_id,
        None,
    )
    .await?;

    // Auto-create FI Journal Entry: DR AR (1200), CR Revenue (4100)
    let ar_account: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM fi_accounts WHERE account_number = '1200'")
            .fetch_optional(&mut *tx)
            .await?;

    let revenue_account: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM fi_accounts WHERE account_number = '4100'")
            .fetch_optional(&mut *tx)
            .await?;

    let company_code: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM fi_company_codes LIMIT 1")
        .fetch_optional(&mut *tx)
        .await?;

    // Log warnings for missing FI accounts instead of silently skipping
    if ar_account.is_none() {
        tracing::warn!(
            "SD->FI integration skipped for invoice {}: FI account 1200 (Accounts Receivable) not found",
            invoice.invoice_number
        );
    }
    if revenue_account.is_none() {
        tracing::warn!(
            "SD->FI integration skipped for invoice {}: FI account 4100 (Revenue) not found",
            invoice.invoice_number
        );
    }
    if company_code.is_none() {
        tracing::warn!(
            "SD->FI integration skipped for invoice {}: No company code configured",
            invoice.invoice_number
        );
    }

    if let (Some((ar_id,)), Some((rev_id,)), Some((cc_id,))) =
        (ar_account, revenue_account, company_code)
    {
        let today = Utc::now().date_naive();
        let je_input = CreateJournalEntry {
            company_code_id: cc_id,
            posting_date: input.invoice_date,
            document_date: today,
            reference: Some(format!("SD-INV:{}", invoice.invoice_number)),
            description: Some(format!("Sales invoice {}", invoice.invoice_number)),
            items: vec![
                CreateJournalItem {
                    account_id: ar_id,
                    debit_amount: input.total_amount,
                    credit_amount: Decimal::ZERO,
                    cost_center_id: None,
                    description: Some("Accounts receivable from sales".to_string()),
                },
                CreateJournalItem {
                    account_id: rev_id,
                    debit_amount: Decimal::ZERO,
                    credit_amount: input.total_amount,
                    cost_center_id: None,
                    description: Some("Sales revenue".to_string()),
                },
            ],
        };
        let je =
            crate::fi::services::create_journal_entry_in_tx(&mut *tx, je_input, user_id).await?;
        crate::fi::services::post_journal_entry_in_tx(&mut *tx, je.id, user_id).await?;

        // Create AR Invoice
        let ar_input = CreateArInvoice {
            customer_id: Some(input.customer_id),
            invoice_date: input.invoice_date,
            due_date: input.due_date,
            total_amount: input.total_amount,
        };
        crate::fi::services::create_ar_invoice_in_tx(&mut *tx, ar_input).await?;
    }

    // GAP 10: Check if SO should transition to INVOICED
    // Calculate total already invoiced for this SO (excluding the one we just created)
    let (already_invoiced_for_status,): (Decimal,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_amount), 0) FROM sd_invoices WHERE sales_order_id = $1 AND id != $2",
    )
    .bind(input.sales_order_id)
    .bind(invoice.id)
    .fetch_one(&mut *tx)
    .await?;

    let total_invoiced = already_invoiced_for_status + input.total_amount;
    if total_invoiced >= so.total_amount && so.status != "INVOICED" && so.status != "CLOSED" {
        // Only transition if current status allows it
        if matches!(so.status.as_str(), "DELIVERED" | "PARTIALLY_DELIVERED") {
            sqlx::query(
                "UPDATE sd_sales_orders SET status = 'INVOICED', updated_at = NOW() WHERE id = $1",
            )
            .bind(input.sales_order_id)
            .execute(&mut *tx)
            .await?;
            status_history::record_transition(
                &mut *tx,
                &DocumentType::SalesOrder,
                input.sales_order_id,
                Some(&so.status),
                "INVOICED",
                user_id,
                None,
            )
            .await?;
        }
    }

    tx.commit().await?;

    // GAP 4: SD->CO profit center posting (best-effort, after commit)
    {
        let customer = repositories::get_customer(pool, input.customer_id).await;
        if let Ok(customer) = customer {
            if let Some(pc_id) = customer.profit_center_id {
                let desc = format!(
                    "SD revenue: invoice {} for SO {}",
                    invoice.invoice_number, so.order_number
                );
                if let Err(e) = crate::co::services::auto_post_revenue_to_profit_center(
                    pool,
                    "SD",
                    invoice.id,
                    pc_id,
                    input.total_amount,
                    input.invoice_date,
                    &desc,
                )
                .await
                {
                    tracing::warn!("CO profit center posting failed: {e}");
                }
            }
        }
    }

    Ok(invoice)
}
