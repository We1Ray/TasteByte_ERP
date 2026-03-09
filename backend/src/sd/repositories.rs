use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::sd::models::*;
use crate::shared::pagination::ListParams;
use crate::shared::AppError;

// --- Customers ---
pub async fn list_customers(pool: &PgPool, params: &ListParams) -> Result<Vec<Customer>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, Customer>(
        r#"SELECT * FROM sd_customers
           WHERE is_active = true
             AND ($1 = false OR (customer_number ILIKE $2 OR name ILIKE $2 OR COALESCE(email, '') ILIKE $2))
           ORDER BY customer_number
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

pub async fn count_customers(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM sd_customers
           WHERE is_active = true
             AND ($1 = false OR (customer_number ILIKE $2 OR name ILIKE $2 OR COALESCE(email, '') ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_customer(pool: &PgPool, id: Uuid) -> Result<Customer, AppError> {
    sqlx::query_as::<_, Customer>("SELECT * FROM sd_customers WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))
}

pub async fn get_customer_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
) -> Result<Customer, AppError> {
    sqlx::query_as::<_, Customer>("SELECT * FROM sd_customers WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))
}

pub async fn create_customer(
    pool: &PgPool,
    cust_number: &str,
    input: &CreateCustomer,
) -> Result<Customer, AppError> {
    let row = sqlx::query_as::<_, Customer>(
        "INSERT INTO sd_customers (customer_number, name, contact_person, email, phone, address, payment_terms, credit_limit, profit_center_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(cust_number).bind(&input.name).bind(&input.contact_person).bind(&input.email)
    .bind(&input.phone).bind(&input.address).bind(input.payment_terms.unwrap_or(30))
    .bind(input.credit_limit.unwrap_or_default()).bind(input.profit_center_id)
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn update_customer(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateCustomer,
) -> Result<Customer, AppError> {
    let existing = get_customer(pool, id).await?;
    let row = sqlx::query_as::<_, Customer>(
        "UPDATE sd_customers SET name = COALESCE($2, name), contact_person = COALESCE($3, contact_person), email = COALESCE($4, email), phone = COALESCE($5, phone), address = COALESCE($6, address), payment_terms = COALESCE($7, payment_terms), credit_limit = COALESCE($8, credit_limit), profit_center_id = COALESCE($9, profit_center_id), is_active = $10, updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id).bind(&input.name).bind(&input.contact_person).bind(&input.email)
    .bind(&input.phone).bind(&input.address).bind(input.payment_terms)
    .bind(input.credit_limit).bind(input.profit_center_id)
    .bind(input.is_active.unwrap_or(existing.is_active))
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Sales Orders ---
pub async fn list_sales_orders(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<SalesOrder>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let rows = sqlx::query_as::<_, SalesOrder>(
        r#"SELECT * FROM sd_sales_orders
           WHERE ($1 = false OR (order_number ILIKE $2 OR COALESCE(notes, '') ILIKE $2))
             AND ($3 = false OR status = $4)
             AND ($5 = false OR order_date >= $6)
             AND ($7 = false OR order_date <= $8)
           ORDER BY created_at DESC
           LIMIT $9 OFFSET $10"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_sales_orders(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let date_from = params
        .date_from
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let date_to = params
        .date_to
        .as_ref()
        .and_then(|d| d.parse::<NaiveDate>().ok());
    let has_date_from = date_from.is_some();
    let has_date_to = date_to.is_some();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM sd_sales_orders
           WHERE ($1 = false OR (order_number ILIKE $2 OR COALESCE(notes, '') ILIKE $2))
             AND ($3 = false OR status = $4)
             AND ($5 = false OR order_date >= $6)
             AND ($7 = false OR order_date <= $8)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_date_from)
    .bind(date_from)
    .bind(has_date_to)
    .bind(date_to)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_sales_order(pool: &PgPool, id: Uuid) -> Result<SalesOrder, AppError> {
    sqlx::query_as::<_, SalesOrder>("SELECT * FROM sd_sales_orders WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Sales order not found".to_string()))
}

pub async fn get_sales_order_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
) -> Result<SalesOrder, AppError> {
    sqlx::query_as::<_, SalesOrder>("SELECT * FROM sd_sales_orders WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| AppError::NotFound("Sales order not found".to_string()))
}

pub async fn get_sales_order_items(
    pool: &PgPool,
    so_id: Uuid,
) -> Result<Vec<SalesOrderItem>, AppError> {
    let rows = sqlx::query_as::<_, SalesOrderItem>(
        "SELECT * FROM sd_sales_order_items WHERE sales_order_id = $1 ORDER BY line_number",
    )
    .bind(so_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_sales_order_items_on_conn(
    conn: &mut sqlx::PgConnection,
    so_id: Uuid,
) -> Result<Vec<SalesOrderItem>, AppError> {
    let rows = sqlx::query_as::<_, SalesOrderItem>(
        "SELECT * FROM sd_sales_order_items WHERE sales_order_id = $1 ORDER BY line_number",
    )
    .bind(so_id)
    .fetch_all(&mut *conn)
    .await?;
    Ok(rows)
}

pub async fn create_sales_order(
    pool: &PgPool,
    order_number: &str,
    input: &CreateSalesOrder,
    user_id: Uuid,
) -> Result<SalesOrder, AppError> {
    let mut tx = pool.begin().await?;

    let total: rust_decimal::Decimal = input.items.iter().map(|i| i.quantity * i.unit_price).sum();

    let so = sqlx::query_as::<_, SalesOrder>(
        "INSERT INTO sd_sales_orders (order_number, customer_id, order_date, requested_delivery_date, total_amount, notes, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(order_number).bind(input.customer_id).bind(input.order_date).bind(input.requested_delivery_date)
    .bind(total).bind(&input.notes).bind(user_id)
    .fetch_one(&mut *tx).await?;

    for (i, item) in input.items.iter().enumerate() {
        let total_price = item.quantity * item.unit_price;
        sqlx::query(
            "INSERT INTO sd_sales_order_items (sales_order_id, line_number, material_id, quantity, unit_price, total_price, uom_id) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(so.id).bind((i + 1) as i32).bind(item.material_id).bind(item.quantity)
        .bind(item.unit_price).bind(total_price).bind(item.uom_id)
        .execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(so)
}

// --- Deliveries ---
pub async fn list_deliveries(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<Delivery>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, Delivery>(
        r#"SELECT * FROM sd_deliveries
           WHERE ($1 = false OR delivery_number ILIKE $2)
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

pub async fn count_deliveries(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM sd_deliveries
           WHERE ($1 = false OR delivery_number ILIKE $2)
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

pub async fn create_delivery(
    pool: &PgPool,
    delivery_number: &str,
    input: &CreateDelivery,
) -> Result<Delivery, AppError> {
    let mut tx = pool.begin().await?;
    let result = create_delivery_inner(&mut *tx, delivery_number, input).await?;
    tx.commit().await?;
    Ok(result)
}

pub async fn create_delivery_on_conn(
    conn: &mut sqlx::PgConnection,
    delivery_number: &str,
    input: &CreateDelivery,
) -> Result<Delivery, AppError> {
    create_delivery_inner(conn, delivery_number, input).await
}

async fn create_delivery_inner(
    conn: &mut sqlx::PgConnection,
    delivery_number: &str,
    input: &CreateDelivery,
) -> Result<Delivery, AppError> {
    let delivery = sqlx::query_as::<_, Delivery>(
        "INSERT INTO sd_deliveries (delivery_number, sales_order_id, delivery_date) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(delivery_number).bind(input.sales_order_id).bind(input.delivery_date)
    .fetch_one(&mut *conn).await?;

    for item in &input.items {
        sqlx::query(
            "INSERT INTO sd_delivery_items (delivery_id, sales_order_item_id, quantity) VALUES ($1, $2, $3)"
        )
        .bind(delivery.id).bind(item.sales_order_item_id).bind(item.quantity)
        .execute(&mut *conn).await?;
    }

    Ok(delivery)
}

// --- SD Invoices ---
pub async fn list_sd_invoices(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<SdInvoice>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, SdInvoice>(
        r#"SELECT * FROM sd_invoices
           WHERE ($1 = false OR invoice_number ILIKE $2)
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

pub async fn count_sd_invoices(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM sd_invoices
           WHERE ($1 = false OR invoice_number ILIKE $2)
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

pub async fn create_sd_invoice(
    pool: &PgPool,
    invoice_number: &str,
    input: &CreateSdInvoice,
) -> Result<SdInvoice, AppError> {
    let row = sqlx::query_as::<_, SdInvoice>(
        "INSERT INTO sd_invoices (invoice_number, sales_order_id, delivery_id, customer_id, invoice_date, due_date, total_amount) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(invoice_number).bind(input.sales_order_id).bind(input.delivery_id)
    .bind(input.customer_id).bind(input.invoice_date).bind(input.due_date).bind(input.total_amount)
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn create_sd_invoice_on_conn(
    conn: &mut sqlx::PgConnection,
    invoice_number: &str,
    input: &CreateSdInvoice,
) -> Result<SdInvoice, AppError> {
    let row = sqlx::query_as::<_, SdInvoice>(
        "INSERT INTO sd_invoices (invoice_number, sales_order_id, delivery_id, customer_id, invoice_date, due_date, total_amount) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(invoice_number).bind(input.sales_order_id).bind(input.delivery_id)
    .bind(input.customer_id).bind(input.invoice_date).bind(input.due_date).bind(input.total_amount)
    .fetch_one(&mut *conn).await?;
    Ok(row)
}

/// Update sales order status
pub async fn update_sales_order_status(
    pool: &PgPool,
    so_id: Uuid,
    status: &str,
) -> Result<SalesOrder, AppError> {
    sqlx::query_as::<_, SalesOrder>(
        "UPDATE sd_sales_orders SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(so_id)
    .bind(status)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Sales order not found".to_string()))
}

pub async fn update_sales_order_status_on_conn(
    conn: &mut sqlx::PgConnection,
    so_id: Uuid,
    status: &str,
) -> Result<SalesOrder, AppError> {
    sqlx::query_as::<_, SalesOrder>(
        "UPDATE sd_sales_orders SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(so_id)
    .bind(status)
    .fetch_optional(&mut *conn)
    .await?
    .ok_or_else(|| AppError::NotFound("Sales order not found".to_string()))
}

/// Update delivered quantity on a sales order item
pub async fn update_so_item_delivered(
    pool: &PgPool,
    item_id: Uuid,
    delivered_qty: rust_decimal::Decimal,
) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE sd_sales_order_items SET delivered_quantity = delivered_quantity + $2 WHERE id = $1"
    )
    .bind(item_id).bind(delivered_qty)
    .execute(pool).await?;
    Ok(())
}

pub async fn update_so_item_delivered_on_conn(
    conn: &mut sqlx::PgConnection,
    item_id: Uuid,
    delivered_qty: rust_decimal::Decimal,
) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE sd_sales_order_items SET delivered_quantity = delivered_quantity + $2 WHERE id = $1"
    )
    .bind(item_id).bind(delivered_qty)
    .execute(&mut *conn).await?;
    Ok(())
}

pub async fn next_number(pool: &PgPool, object_type: &str) -> Result<String, AppError> {
    crate::shared::number_range::next_number(pool, object_type).await
}

pub async fn next_number_on_conn(
    conn: &mut sqlx::PgConnection,
    object_type: &str,
) -> Result<String, AppError> {
    crate::shared::number_range::next_number_on_conn(conn, object_type).await
}
