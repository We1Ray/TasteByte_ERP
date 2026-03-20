use sqlx::PgPool;
use uuid::Uuid;

use crate::pp::models::*;
use crate::shared::pagination::ListParams;
use crate::shared::AppError;

// --- BOMs ---
pub async fn list_boms(pool: &PgPool, params: &ListParams) -> Result<Vec<Bom>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, Bom>(
        r#"SELECT * FROM pp_boms
           WHERE ($1 = false OR (bom_number ILIKE $2 OR name ILIKE $2))
           ORDER BY bom_number
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

pub async fn count_boms(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM pp_boms
           WHERE ($1 = false OR (bom_number ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_bom(pool: &PgPool, id: Uuid) -> Result<Bom, AppError> {
    sqlx::query_as::<_, Bom>("SELECT * FROM pp_boms WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("BOM not found".to_string()))
}

pub async fn get_bom_items(pool: &PgPool, bom_id: Uuid) -> Result<Vec<BomItem>, AppError> {
    let rows = sqlx::query_as::<_, BomItem>(
        "SELECT * FROM pp_bom_items WHERE bom_id = $1 ORDER BY line_number",
    )
    .bind(bom_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_bom(
    pool: &PgPool,
    bom_number: &str,
    input: &CreateBom,
) -> Result<Bom, AppError> {
    let mut tx = pool.begin().await?;

    let bom = sqlx::query_as::<_, Bom>(
        "INSERT INTO pp_boms (bom_number, material_id, name, valid_from, valid_to) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(bom_number).bind(input.material_id).bind(&input.name).bind(input.valid_from).bind(input.valid_to)
    .fetch_one(&mut *tx).await?;

    for (i, item) in input.items.iter().enumerate() {
        sqlx::query(
            "INSERT INTO pp_bom_items (bom_id, line_number, component_material_id, quantity, uom_id, scrap_percentage) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(bom.id).bind((i + 1) as i32).bind(item.component_material_id).bind(item.quantity)
        .bind(item.uom_id).bind(item.scrap_percentage.unwrap_or_default())
        .execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(bom)
}

// --- Routings ---
pub async fn list_routings(pool: &PgPool, params: &ListParams) -> Result<Vec<Routing>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let rows = sqlx::query_as::<_, Routing>(
        r#"SELECT * FROM pp_routings
           WHERE ($1 = false OR (routing_number ILIKE $2 OR name ILIKE $2))
           ORDER BY routing_number
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

pub async fn count_routings(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM pp_routings
           WHERE ($1 = false OR (routing_number ILIKE $2 OR name ILIKE $2))"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_routing(pool: &PgPool, id: Uuid) -> Result<Routing, AppError> {
    sqlx::query_as::<_, Routing>("SELECT * FROM pp_routings WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Routing not found".to_string()))
}

pub async fn get_routing_operations(
    pool: &PgPool,
    routing_id: Uuid,
) -> Result<Vec<RoutingOperation>, AppError> {
    let rows = sqlx::query_as::<_, RoutingOperation>(
        "SELECT * FROM pp_routing_operations WHERE routing_id = $1 ORDER BY operation_number",
    )
    .bind(routing_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_routing_operations_on_conn(
    conn: &mut sqlx::PgConnection,
    routing_id: Uuid,
) -> Result<Vec<RoutingOperation>, AppError> {
    let rows = sqlx::query_as::<_, RoutingOperation>(
        "SELECT * FROM pp_routing_operations WHERE routing_id = $1 ORDER BY operation_number",
    )
    .bind(routing_id)
    .fetch_all(&mut *conn)
    .await?;
    Ok(rows)
}

pub async fn create_routing(
    pool: &PgPool,
    routing_number: &str,
    input: &CreateRouting,
) -> Result<Routing, AppError> {
    let mut tx = pool.begin().await?;

    let routing = sqlx::query_as::<_, Routing>(
        "INSERT INTO pp_routings (routing_number, material_id, name) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(routing_number).bind(input.material_id).bind(&input.name)
    .fetch_one(&mut *tx).await?;

    for op in &input.operations {
        sqlx::query(
            "INSERT INTO pp_routing_operations (routing_id, operation_number, work_center, description, setup_time_minutes, run_time_minutes) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(routing.id).bind(op.operation_number).bind(&op.work_center).bind(&op.description)
        .bind(op.setup_time_minutes.unwrap_or(0)).bind(op.run_time_minutes.unwrap_or(0))
        .execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(routing)
}

// --- Production Orders ---
pub async fn list_production_orders(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<ProductionOrder>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, ProductionOrder>(
        r#"SELECT * FROM pp_production_orders
           WHERE ($1 = false OR order_number ILIKE $2)
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

pub async fn count_production_orders(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM pp_production_orders
           WHERE ($1 = false OR order_number ILIKE $2)
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

pub async fn get_production_order(pool: &PgPool, id: Uuid) -> Result<ProductionOrder, AppError> {
    sqlx::query_as::<_, ProductionOrder>("SELECT * FROM pp_production_orders WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Production order not found".to_string()))
}

pub async fn get_production_order_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
) -> Result<ProductionOrder, AppError> {
    sqlx::query_as::<_, ProductionOrder>("SELECT * FROM pp_production_orders WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| AppError::NotFound("Production order not found".to_string()))
}

pub async fn create_production_order(
    pool: &PgPool,
    order_number: &str,
    input: &CreateProductionOrder,
    user_id: Uuid,
) -> Result<ProductionOrder, AppError> {
    let row = sqlx::query_as::<_, ProductionOrder>(
        "INSERT INTO pp_production_orders (order_number, material_id, bom_id, routing_id, planned_quantity, uom_id, planned_start, planned_end, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(order_number).bind(input.material_id).bind(input.bom_id).bind(input.routing_id)
    .bind(input.planned_quantity).bind(input.uom_id).bind(input.planned_start).bind(input.planned_end).bind(user_id)
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn update_production_order_status(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateProductionOrderStatus,
) -> Result<ProductionOrder, AppError> {
    let row = sqlx::query_as::<_, ProductionOrder>(
        "UPDATE pp_production_orders SET status = $2, actual_quantity = COALESCE($3, actual_quantity), actual_start = CASE WHEN $2 = 'IN_PROGRESS' AND actual_start IS NULL THEN CURRENT_DATE ELSE actual_start END, actual_end = CASE WHEN $2 = 'COMPLETED' THEN CURRENT_DATE ELSE actual_end END, updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id).bind(&input.status).bind(input.actual_quantity)
    .fetch_optional(pool).await?
    .ok_or_else(|| AppError::NotFound("Production order not found".to_string()))?;
    Ok(row)
}

pub async fn update_production_order_status_on_conn(
    conn: &mut sqlx::PgConnection,
    id: Uuid,
    input: &UpdateProductionOrderStatus,
) -> Result<ProductionOrder, AppError> {
    let row = sqlx::query_as::<_, ProductionOrder>(
        "UPDATE pp_production_orders SET status = $2, actual_quantity = COALESCE($3, actual_quantity), actual_start = CASE WHEN $2 = 'IN_PROGRESS' AND actual_start IS NULL THEN CURRENT_DATE ELSE actual_start END, actual_end = CASE WHEN $2 = 'COMPLETED' THEN CURRENT_DATE ELSE actual_end END, updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id).bind(&input.status).bind(input.actual_quantity)
    .fetch_optional(&mut *conn).await?
    .ok_or_else(|| AppError::NotFound("Production order not found".to_string()))?;
    Ok(row)
}

/// Get BOM items for a production order (by bom_id)
pub async fn get_bom_items_for_order(
    pool: &PgPool,
    bom_id: Uuid,
) -> Result<Vec<BomItem>, AppError> {
    let rows = sqlx::query_as::<_, BomItem>(
        "SELECT * FROM pp_bom_items WHERE bom_id = $1 ORDER BY line_number",
    )
    .bind(bom_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_bom_items_for_order_on_conn(
    conn: &mut sqlx::PgConnection,
    bom_id: Uuid,
) -> Result<Vec<BomItem>, AppError> {
    let rows = sqlx::query_as::<_, BomItem>(
        "SELECT * FROM pp_bom_items WHERE bom_id = $1 ORDER BY line_number",
    )
    .bind(bom_id)
    .fetch_all(&mut *conn)
    .await?;
    Ok(rows)
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

// --- BOM Item Sub-table CRUD ---

pub async fn next_bom_item_line_number(pool: &PgPool, bom_id: Uuid) -> Result<i32, AppError> {
    let (max_line,): (Option<i32>,) =
        sqlx::query_as("SELECT MAX(line_number) FROM pp_bom_items WHERE bom_id = $1")
            .bind(bom_id)
            .fetch_one(pool)
            .await?;
    Ok(max_line.unwrap_or(0) + 1)
}

pub async fn add_bom_item(
    pool: &PgPool,
    bom_id: Uuid,
    line_number: i32,
    input: &AddBomItem,
) -> Result<BomItem, AppError> {
    let row = sqlx::query_as::<_, BomItem>(
        "INSERT INTO pp_bom_items (bom_id, line_number, component_material_id, quantity, uom_id, scrap_percentage) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(bom_id)
    .bind(line_number)
    .bind(input.component_material_id)
    .bind(input.quantity)
    .bind(input.uom_id)
    .bind(input.scrap_percentage.unwrap_or_default())
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn update_bom_item(
    pool: &PgPool,
    bom_id: Uuid,
    item_id: Uuid,
    input: &UpdateBomItem,
) -> Result<BomItem, AppError> {
    let row = sqlx::query_as::<_, BomItem>(
        r#"UPDATE pp_bom_items SET
            quantity = COALESCE($3, quantity),
            uom_id = COALESCE($4, uom_id),
            scrap_percentage = COALESCE($5, scrap_percentage)
        WHERE id = $2 AND bom_id = $1
        RETURNING *"#,
    )
    .bind(bom_id)
    .bind(item_id)
    .bind(input.quantity)
    .bind(input.uom_id)
    .bind(input.scrap_percentage)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("BOM item not found".to_string()))?;
    Ok(row)
}

pub async fn delete_bom_item(pool: &PgPool, bom_id: Uuid, item_id: Uuid) -> Result<(), AppError> {
    let result =
        sqlx::query("DELETE FROM pp_bom_items WHERE id = $1 AND bom_id = $2")
            .bind(item_id)
            .bind(bom_id)
            .execute(pool)
            .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("BOM item not found".to_string()));
    }
    Ok(())
}

// --- Routing Operation Sub-table CRUD ---

pub async fn add_routing_operation(
    pool: &PgPool,
    routing_id: Uuid,
    input: &AddRoutingOperation,
) -> Result<RoutingOperation, AppError> {
    let row = sqlx::query_as::<_, RoutingOperation>(
        "INSERT INTO pp_routing_operations (routing_id, operation_number, work_center, description, setup_time_minutes, run_time_minutes) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(routing_id)
    .bind(input.operation_number)
    .bind(&input.work_center)
    .bind(&input.description)
    .bind(input.setup_time_minutes.unwrap_or(0))
    .bind(input.run_time_minutes.unwrap_or(0))
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn update_routing_operation(
    pool: &PgPool,
    routing_id: Uuid,
    op_id: Uuid,
    input: &UpdateRoutingOperation,
) -> Result<RoutingOperation, AppError> {
    let row = sqlx::query_as::<_, RoutingOperation>(
        r#"UPDATE pp_routing_operations SET
            operation_number = COALESCE($3, operation_number),
            work_center = COALESCE($4, work_center),
            description = COALESCE($5, description),
            setup_time_minutes = COALESCE($6, setup_time_minutes),
            run_time_minutes = COALESCE($7, run_time_minutes)
        WHERE id = $2 AND routing_id = $1
        RETURNING *"#,
    )
    .bind(routing_id)
    .bind(op_id)
    .bind(input.operation_number)
    .bind(&input.work_center)
    .bind(&input.description)
    .bind(input.setup_time_minutes)
    .bind(input.run_time_minutes)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Routing operation not found".to_string()))?;
    Ok(row)
}

pub async fn delete_routing_operation(
    pool: &PgPool,
    routing_id: Uuid,
    op_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        "DELETE FROM pp_routing_operations WHERE id = $1 AND routing_id = $2",
    )
    .bind(op_id)
    .bind(routing_id)
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Routing operation not found".to_string(),
        ));
    }
    Ok(())
}
