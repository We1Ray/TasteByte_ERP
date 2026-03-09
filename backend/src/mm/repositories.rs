use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::mm::models::*;
use crate::shared::pagination::ListParams;
use crate::shared::AppError;

// --- UOM ---
pub async fn list_uoms(pool: &PgPool, params: &ListParams) -> Result<Vec<Uom>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, Uom>(
        r#"SELECT * FROM mm_uom
           WHERE ($1 = false OR (code ILIKE $2 OR name ILIKE $2))
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

pub async fn count_uoms(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM mm_uom
           WHERE ($1 = false OR (code ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_uom(pool: &PgPool, input: &CreateUom) -> Result<Uom, AppError> {
    let row = sqlx::query_as::<_, Uom>(
        "INSERT INTO mm_uom (code, name, is_base) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&input.code)
    .bind(&input.name)
    .bind(input.is_base.unwrap_or(false))
    .fetch_one(pool)
    .await?;
    Ok(row)
}

// --- Material Groups ---
pub async fn list_material_groups(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<MaterialGroup>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, MaterialGroup>(
        r#"SELECT * FROM mm_material_groups
           WHERE ($1 = false OR (code ILIKE $2 OR name ILIKE $2))
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

pub async fn count_material_groups(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM mm_material_groups
           WHERE ($1 = false OR (code ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_material_group(
    pool: &PgPool,
    input: &CreateMaterialGroup,
) -> Result<MaterialGroup, AppError> {
    let row = sqlx::query_as::<_, MaterialGroup>(
        "INSERT INTO mm_material_groups (code, name, description) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&input.code)
    .bind(&input.name)
    .bind(&input.description)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

// --- Materials ---
pub async fn list_materials(pool: &PgPool, params: &ListParams) -> Result<Vec<Material>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, Material>(
        r#"SELECT * FROM mm_materials
           WHERE is_active = true
             AND ($1 = false OR (material_number ILIKE $2 OR name ILIKE $2))
             AND ($3 = false OR material_type = $4)
           ORDER BY material_number
           LIMIT $5 OFFSET $6"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_filter_type)
    .bind(&filter_type)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_materials(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM mm_materials
           WHERE is_active = true
             AND ($1 = false OR (material_number ILIKE $2 OR name ILIKE $2))
             AND ($3 = false OR material_type = $4)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_filter_type)
    .bind(&filter_type)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_material(pool: &PgPool, id: Uuid) -> Result<Material, AppError> {
    sqlx::query_as::<_, Material>("SELECT * FROM mm_materials WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Material not found".to_string()))
}

pub async fn create_material(
    pool: &PgPool,
    mat_number: &str,
    input: &CreateMaterial,
) -> Result<Material, AppError> {
    let row = sqlx::query_as::<_, Material>(
        "INSERT INTO mm_materials (material_number, name, description, material_group_id, base_uom_id, material_type, weight, weight_uom) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
    )
    .bind(mat_number).bind(&input.name).bind(&input.description).bind(input.material_group_id)
    .bind(input.base_uom_id).bind(input.material_type.as_deref().unwrap_or("RAW"))
    .bind(input.weight).bind(&input.weight_uom)
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn update_material(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateMaterial,
) -> Result<Material, AppError> {
    let existing = get_material(pool, id).await?;
    let row = sqlx::query_as::<_, Material>(
        "UPDATE mm_materials SET name = COALESCE($2, name), description = COALESCE($3, description), material_group_id = COALESCE($4, material_group_id), base_uom_id = COALESCE($5, base_uom_id), weight = COALESCE($6, weight), weight_uom = COALESCE($7, weight_uom), is_active = $8, updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id).bind(&input.name).bind(&input.description).bind(input.material_group_id)
    .bind(input.base_uom_id).bind(input.weight).bind(&input.weight_uom)
    .bind(input.is_active.unwrap_or(existing.is_active))
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Vendors ---
pub async fn list_vendors(pool: &PgPool, params: &ListParams) -> Result<Vec<Vendor>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, Vendor>(
        r#"SELECT * FROM mm_vendors
           WHERE is_active = true
             AND ($1 = false OR (vendor_number ILIKE $2 OR name ILIKE $2))
           ORDER BY vendor_number
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

pub async fn count_vendors(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM mm_vendors
           WHERE is_active = true
             AND ($1 = false OR (vendor_number ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_vendor(pool: &PgPool, id: Uuid) -> Result<Vendor, AppError> {
    sqlx::query_as::<_, Vendor>("SELECT * FROM mm_vendors WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Vendor not found".to_string()))
}

pub async fn create_vendor(
    pool: &PgPool,
    vendor_number: &str,
    input: &CreateVendor,
) -> Result<Vendor, AppError> {
    let row = sqlx::query_as::<_, Vendor>(
        "INSERT INTO mm_vendors (vendor_number, name, contact_person, email, phone, address, payment_terms) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(vendor_number).bind(&input.name).bind(&input.contact_person).bind(&input.email)
    .bind(&input.phone).bind(&input.address).bind(input.payment_terms.unwrap_or(30))
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn update_vendor(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateVendor,
) -> Result<Vendor, AppError> {
    let existing = get_vendor(pool, id).await?;
    let row = sqlx::query_as::<_, Vendor>(
        "UPDATE mm_vendors SET name = COALESCE($2, name), contact_person = COALESCE($3, contact_person), email = COALESCE($4, email), phone = COALESCE($5, phone), address = COALESCE($6, address), payment_terms = COALESCE($7, payment_terms), is_active = $8, updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id).bind(&input.name).bind(&input.contact_person).bind(&input.email)
    .bind(&input.phone).bind(&input.address).bind(input.payment_terms)
    .bind(input.is_active.unwrap_or(existing.is_active))
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Plant Stock ---
pub async fn list_plant_stock(
    pool: &PgPool,
    warehouse_id: Option<Uuid>,
    params: &ListParams,
) -> Result<Vec<PlantStock>, AppError> {
    let has_warehouse = warehouse_id.is_some();
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, PlantStock>(
        r#"SELECT ps.* FROM mm_plant_stock ps
           LEFT JOIN mm_materials m ON ps.material_id = m.id
           WHERE ($1 = false OR ps.warehouse_id = $2)
             AND ($3 = false OR (m.material_number ILIKE $4 OR m.name ILIKE $4))
           ORDER BY ps.material_id"#,
    )
    .bind(has_warehouse)
    .bind(warehouse_id)
    .bind(has_search)
    .bind(&pattern)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

// --- Material Movements ---
pub async fn list_material_movements(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<MaterialMovement>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, MaterialMovement>(
        r#"SELECT * FROM mm_material_movements
           WHERE ($1 = false OR document_number ILIKE $2)
             AND ($3 = false OR movement_type = $4)
           ORDER BY posted_at DESC
           LIMIT $5 OFFSET $6"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_filter_type)
    .bind(&filter_type)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_material_movements(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM mm_material_movements
           WHERE ($1 = false OR document_number ILIKE $2)
             AND ($3 = false OR movement_type = $4)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_filter_type)
    .bind(&filter_type)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn create_material_movement(
    pool: &PgPool,
    doc_number: &str,
    input: &CreateMaterialMovement,
    user_id: Uuid,
) -> Result<MaterialMovement, AppError> {
    let mut tx = pool.begin().await?;
    let result = create_material_movement_inner(&mut *tx, doc_number, input, user_id).await?;
    tx.commit().await?;
    Ok(result)
}

pub async fn create_material_movement_on_conn(
    conn: &mut sqlx::PgConnection,
    doc_number: &str,
    input: &CreateMaterialMovement,
    user_id: Uuid,
) -> Result<MaterialMovement, AppError> {
    create_material_movement_inner(conn, doc_number, input, user_id).await
}

async fn create_material_movement_inner(
    conn: &mut sqlx::PgConnection,
    doc_number: &str,
    input: &CreateMaterialMovement,
    user_id: Uuid,
) -> Result<MaterialMovement, AppError> {
    let row = sqlx::query_as::<_, MaterialMovement>(
        "INSERT INTO mm_material_movements (document_number, movement_type, material_id, warehouse_id, quantity, uom_id, reference_type, reference_id, posted_by) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(doc_number).bind(&input.movement_type).bind(input.material_id).bind(input.warehouse_id)
    .bind(input.quantity).bind(input.uom_id).bind(&input.reference_type).bind(input.reference_id).bind(user_id)
    .fetch_one(&mut *conn).await?;

    // Update plant stock based on movement type
    match input.movement_type.as_str() {
        "GOODS_RECEIPT" => {
            sqlx::query(
                "INSERT INTO mm_plant_stock (material_id, warehouse_id, quantity, uom_id, updated_at) \
                 VALUES ($1, $2, $3, $4, NOW()) \
                 ON CONFLICT (material_id, warehouse_id) DO UPDATE SET \
                 quantity = mm_plant_stock.quantity + $3, updated_at = NOW()"
            )
            .bind(input.material_id).bind(input.warehouse_id).bind(input.quantity).bind(input.uom_id)
            .execute(&mut *conn).await?;
        }
        "GOODS_ISSUE" => {
            if input.warehouse_id.is_some() {
                let stock: Option<(rust_decimal::Decimal,)> = sqlx::query_as(
                    "SELECT quantity FROM mm_plant_stock WHERE material_id = $1 AND warehouse_id = $2"
                )
                .bind(input.material_id).bind(input.warehouse_id)
                .fetch_optional(&mut *conn).await?;

                let current_qty = stock.map(|s| s.0).unwrap_or_default();
                if current_qty < input.quantity {
                    return Err(AppError::Validation(format!(
                        "Insufficient stock: available={}, requested={}",
                        current_qty, input.quantity
                    )));
                }

                sqlx::query(
                    "UPDATE mm_plant_stock SET quantity = quantity - $3, updated_at = NOW() \
                     WHERE material_id = $1 AND warehouse_id = $2",
                )
                .bind(input.material_id)
                .bind(input.warehouse_id)
                .bind(input.quantity)
                .execute(&mut *conn)
                .await?;
            } else {
                let total: Option<(rust_decimal::Decimal,)> = sqlx::query_as(
                    "SELECT COALESCE(SUM(quantity), 0) FROM mm_plant_stock WHERE material_id = $1",
                )
                .bind(input.material_id)
                .fetch_optional(&mut *conn)
                .await?;

                let current_qty = total.map(|s| s.0).unwrap_or_default();
                if current_qty < input.quantity {
                    return Err(AppError::Validation(format!(
                        "Insufficient stock: available={}, requested={}",
                        current_qty, input.quantity
                    )));
                }

                sqlx::query(
                    "UPDATE mm_plant_stock SET quantity = quantity - $2, updated_at = NOW() \
                     WHERE id = (SELECT id FROM mm_plant_stock WHERE material_id = $1 AND quantity > 0 ORDER BY quantity DESC LIMIT 1)"
                )
                .bind(input.material_id).bind(input.quantity)
                .execute(&mut *conn).await?;
            }
        }
        "TRANSFER" => {
            let target_warehouse_id = input.reference_id;

            let stock: Option<(rust_decimal::Decimal,)> = sqlx::query_as(
                "SELECT quantity FROM mm_plant_stock WHERE material_id = $1 AND warehouse_id IS NOT DISTINCT FROM $2"
            )
            .bind(input.material_id).bind(input.warehouse_id)
            .fetch_optional(&mut *conn).await?;

            let current_qty = stock.map(|s| s.0).unwrap_or_default();
            if current_qty < input.quantity {
                return Err(AppError::Validation(format!(
                    "Insufficient stock for transfer: available={}, requested={}",
                    current_qty, input.quantity
                )));
            }

            sqlx::query(
                "UPDATE mm_plant_stock SET quantity = quantity - $3, updated_at = NOW() \
                 WHERE material_id = $1 AND warehouse_id IS NOT DISTINCT FROM $2",
            )
            .bind(input.material_id)
            .bind(input.warehouse_id)
            .bind(input.quantity)
            .execute(&mut *conn)
            .await?;

            sqlx::query(
                "INSERT INTO mm_plant_stock (material_id, warehouse_id, quantity, uom_id, updated_at) \
                 VALUES ($1, $2, $3, $4, NOW()) \
                 ON CONFLICT (material_id, warehouse_id) DO UPDATE SET \
                 quantity = mm_plant_stock.quantity + $3, updated_at = NOW()"
            )
            .bind(input.material_id).bind(target_warehouse_id).bind(input.quantity).bind(input.uom_id)
            .execute(&mut *conn).await?;
        }
        "ADJUSTMENT" => {
            sqlx::query(
                "INSERT INTO mm_plant_stock (material_id, warehouse_id, quantity, uom_id, updated_at) \
                 VALUES ($1, $2, $3, $4, NOW()) \
                 ON CONFLICT (material_id, warehouse_id) DO UPDATE SET \
                 quantity = $3, updated_at = NOW()"
            )
            .bind(input.material_id).bind(input.warehouse_id).bind(input.quantity).bind(input.uom_id)
            .execute(&mut *conn).await?;
        }
        _ => {
            return Err(AppError::Validation(format!(
                "Unknown movement type: {}",
                input.movement_type
            )));
        }
    }

    Ok(row)
}

/// Reserve stock for a sales order confirmation
pub async fn reserve_stock(
    pool: &PgPool,
    material_id: Uuid,
    warehouse_id: Option<Uuid>,
    quantity: rust_decimal::Decimal,
) -> Result<(), AppError> {
    let mut conn = pool.acquire().await?;
    reserve_stock_inner(&mut *conn, material_id, warehouse_id, quantity).await
}

pub async fn reserve_stock_on_conn(
    conn: &mut sqlx::PgConnection,
    material_id: Uuid,
    warehouse_id: Option<Uuid>,
    quantity: rust_decimal::Decimal,
) -> Result<(), AppError> {
    reserve_stock_inner(conn, material_id, warehouse_id, quantity).await
}

async fn reserve_stock_inner(
    conn: &mut sqlx::PgConnection,
    material_id: Uuid,
    warehouse_id: Option<Uuid>,
    quantity: rust_decimal::Decimal,
) -> Result<(), AppError> {
    if let Some(wh_id) = warehouse_id {
        let stock: Option<(rust_decimal::Decimal, rust_decimal::Decimal)> = sqlx::query_as(
            "SELECT quantity, reserved_quantity FROM mm_plant_stock WHERE material_id = $1 AND warehouse_id = $2"
        )
        .bind(material_id).bind(wh_id)
        .fetch_optional(&mut *conn).await?;

        let (current_qty, reserved_qty) = stock.unwrap_or_default();
        let available = current_qty - reserved_qty;
        if available < quantity {
            return Err(AppError::Validation(format!(
                "Insufficient available stock for reservation: available={}, requested={}",
                available, quantity
            )));
        }

        sqlx::query(
            "UPDATE mm_plant_stock SET reserved_quantity = reserved_quantity + $3, updated_at = NOW() \
             WHERE material_id = $1 AND warehouse_id = $2"
        )
        .bind(material_id).bind(wh_id).bind(quantity)
        .execute(&mut *conn).await?;
    } else {
        let total: Option<(rust_decimal::Decimal,)> = sqlx::query_as(
            "SELECT COALESCE(SUM(quantity - reserved_quantity), 0) FROM mm_plant_stock WHERE material_id = $1"
        )
        .bind(material_id)
        .fetch_optional(&mut *conn).await?;

        let available = total.map(|t| t.0).unwrap_or_default();
        if available < quantity {
            return Err(AppError::Validation(format!(
                "Insufficient available stock for reservation: available={}, requested={}",
                available, quantity
            )));
        }

        let first_wh: Option<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM mm_plant_stock WHERE material_id = $1 AND (quantity - reserved_quantity) > 0 ORDER BY quantity DESC LIMIT 1"
        )
        .bind(material_id)
        .fetch_optional(&mut *conn).await?;

        if let Some((wh_stock_id,)) = first_wh {
            sqlx::query(
                "UPDATE mm_plant_stock SET reserved_quantity = reserved_quantity + $2, updated_at = NOW() WHERE id = $1"
            )
            .bind(wh_stock_id).bind(quantity)
            .execute(&mut *conn).await?;
        }
    }
    Ok(())
}

/// Release reserved stock (e.g., when order is cancelled or delivered)
pub async fn release_stock(
    pool: &PgPool,
    material_id: Uuid,
    warehouse_id: Option<Uuid>,
    quantity: rust_decimal::Decimal,
) -> Result<(), AppError> {
    let mut conn = pool.acquire().await?;
    release_stock_inner(&mut *conn, material_id, warehouse_id, quantity).await
}

pub async fn release_stock_on_conn(
    conn: &mut sqlx::PgConnection,
    material_id: Uuid,
    warehouse_id: Option<Uuid>,
    quantity: rust_decimal::Decimal,
) -> Result<(), AppError> {
    release_stock_inner(conn, material_id, warehouse_id, quantity).await
}

async fn release_stock_inner(
    conn: &mut sqlx::PgConnection,
    material_id: Uuid,
    warehouse_id: Option<Uuid>,
    quantity: rust_decimal::Decimal,
) -> Result<(), AppError> {
    if let Some(wh_id) = warehouse_id {
        sqlx::query(
            "UPDATE mm_plant_stock SET reserved_quantity = GREATEST(reserved_quantity - $3, 0), updated_at = NOW() \
             WHERE material_id = $1 AND warehouse_id = $2"
        )
        .bind(material_id).bind(wh_id).bind(quantity)
        .execute(&mut *conn).await?;
    } else {
        sqlx::query(
            "UPDATE mm_plant_stock SET reserved_quantity = GREATEST(reserved_quantity - $2, 0), updated_at = NOW() \
             WHERE id = (SELECT id FROM mm_plant_stock WHERE material_id = $1 AND reserved_quantity > 0 LIMIT 1)"
        )
        .bind(material_id).bind(quantity)
        .execute(&mut *conn).await?;
    }
    Ok(())
}

/// Update PO item received quantity
pub async fn update_po_item_received(
    pool: &PgPool,
    item_id: Uuid,
    received_qty: rust_decimal::Decimal,
) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE mm_purchase_order_items SET received_quantity = received_quantity + $2 WHERE id = $1"
    )
    .bind(item_id).bind(received_qty)
    .execute(pool).await?;
    Ok(())
}

pub async fn update_po_item_received_on_conn(
    conn: &mut sqlx::PgConnection,
    item_id: Uuid,
    received_qty: rust_decimal::Decimal,
) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE mm_purchase_order_items SET received_quantity = received_quantity + $2 WHERE id = $1"
    )
    .bind(item_id).bind(received_qty)
    .execute(&mut *conn).await?;
    Ok(())
}

/// Update PO status
pub async fn update_po_status(
    pool: &PgPool,
    po_id: Uuid,
    status: &str,
) -> Result<PurchaseOrder, AppError> {
    sqlx::query_as::<_, PurchaseOrder>(
        "UPDATE mm_purchase_orders SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(po_id)
    .bind(status)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))
}

pub async fn update_po_status_on_conn(
    conn: &mut sqlx::PgConnection,
    po_id: Uuid,
    status: &str,
) -> Result<PurchaseOrder, AppError> {
    sqlx::query_as::<_, PurchaseOrder>(
        "UPDATE mm_purchase_orders SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(po_id)
    .bind(status)
    .fetch_optional(&mut *conn)
    .await?
    .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))
}

// --- Purchase Orders ---
pub async fn list_purchase_orders(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<PurchaseOrder>, AppError> {
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

    let rows = sqlx::query_as::<_, PurchaseOrder>(
        r#"SELECT * FROM mm_purchase_orders
           WHERE ($1 = false OR (po_number ILIKE $2 OR COALESCE(notes, '') ILIKE $2))
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

pub async fn count_purchase_orders(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
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
        r#"SELECT COUNT(*) FROM mm_purchase_orders
           WHERE ($1 = false OR (po_number ILIKE $2 OR COALESCE(notes, '') ILIKE $2))
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

pub async fn get_purchase_order(pool: &PgPool, id: Uuid) -> Result<PurchaseOrder, AppError> {
    sqlx::query_as::<_, PurchaseOrder>("SELECT * FROM mm_purchase_orders WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))
}

pub async fn get_purchase_order_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
) -> Result<PurchaseOrder, AppError> {
    sqlx::query_as::<_, PurchaseOrder>("SELECT * FROM mm_purchase_orders WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| AppError::NotFound("Purchase order not found".to_string()))
}

pub async fn get_purchase_order_items(
    pool: &PgPool,
    po_id: Uuid,
) -> Result<Vec<PurchaseOrderItem>, AppError> {
    let rows = sqlx::query_as::<_, PurchaseOrderItem>(
        "SELECT * FROM mm_purchase_order_items WHERE purchase_order_id = $1 ORDER BY line_number",
    )
    .bind(po_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_purchase_order_items_on_conn(
    conn: &mut sqlx::PgConnection,
    po_id: Uuid,
) -> Result<Vec<PurchaseOrderItem>, AppError> {
    let rows = sqlx::query_as::<_, PurchaseOrderItem>(
        "SELECT * FROM mm_purchase_order_items WHERE purchase_order_id = $1 ORDER BY line_number",
    )
    .bind(po_id)
    .fetch_all(&mut *conn)
    .await?;
    Ok(rows)
}

pub async fn create_purchase_order(
    pool: &PgPool,
    po_number: &str,
    input: &CreatePurchaseOrder,
    user_id: Uuid,
) -> Result<PurchaseOrder, AppError> {
    let mut tx = pool.begin().await?;

    let total: rust_decimal::Decimal = input.items.iter().map(|i| i.quantity * i.unit_price).sum();

    let po = sqlx::query_as::<_, PurchaseOrder>(
        "INSERT INTO mm_purchase_orders (po_number, vendor_id, order_date, delivery_date, total_amount, notes, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(po_number).bind(input.vendor_id).bind(input.order_date).bind(input.delivery_date)
    .bind(total).bind(&input.notes).bind(user_id)
    .fetch_one(&mut *tx).await?;

    for (i, item) in input.items.iter().enumerate() {
        let total_price = item.quantity * item.unit_price;
        sqlx::query(
            "INSERT INTO mm_purchase_order_items (purchase_order_id, line_number, material_id, quantity, unit_price, total_price, uom_id, delivery_date) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(po.id).bind((i + 1) as i32).bind(item.material_id).bind(item.quantity)
        .bind(item.unit_price).bind(total_price).bind(item.uom_id).bind(item.delivery_date)
        .execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(po)
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
