use sqlx::PgPool;
use uuid::Uuid;

use crate::qm::models::*;
use crate::shared::pagination::ListParams;
use crate::shared::AppError;

// --- Inspection Lots ---
pub async fn list_inspection_lots(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<InspectionLot>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, InspectionLot>(
        r#"SELECT * FROM qm_inspection_lots
           WHERE ($1 = false OR lot_number ILIKE $2)
             AND ($3 = false OR status = $4)
             AND ($5 = false OR inspection_type = $6)
           ORDER BY created_at DESC
           LIMIT $7 OFFSET $8"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_filter_type)
    .bind(&filter_type)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_inspection_lots(pool: &PgPool, params: &ListParams) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM qm_inspection_lots
           WHERE ($1 = false OR lot_number ILIKE $2)
             AND ($3 = false OR status = $4)
             AND ($5 = false OR inspection_type = $6)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_filter_type)
    .bind(&filter_type)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_inspection_lot(pool: &PgPool, id: Uuid) -> Result<InspectionLot, AppError> {
    sqlx::query_as::<_, InspectionLot>("SELECT * FROM qm_inspection_lots WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Inspection lot not found".to_string()))
}

pub async fn create_inspection_lot(
    pool: &PgPool,
    lot_number: &str,
    input: &CreateInspectionLot,
    user_id: Uuid,
) -> Result<InspectionLot, AppError> {
    let row = sqlx::query_as::<_, InspectionLot>(
        "INSERT INTO qm_inspection_lots (lot_number, material_id, reference_type, reference_id, inspection_type, planned_quantity, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(lot_number).bind(input.material_id).bind(&input.reference_type).bind(input.reference_id)
    .bind(input.inspection_type.as_deref().unwrap_or("INCOMING")).bind(input.planned_quantity).bind(user_id)
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn create_inspection_lot_on_conn(
    conn: &mut sqlx::PgConnection,
    lot_number: &str,
    input: &CreateInspectionLot,
    user_id: Uuid,
) -> Result<InspectionLot, AppError> {
    let row = sqlx::query_as::<_, InspectionLot>(
        "INSERT INTO qm_inspection_lots (lot_number, material_id, reference_type, reference_id, inspection_type, planned_quantity, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(lot_number).bind(input.material_id).bind(&input.reference_type).bind(input.reference_id)
    .bind(input.inspection_type.as_deref().unwrap_or("INCOMING")).bind(input.planned_quantity).bind(user_id)
    .fetch_one(&mut *conn).await?;
    Ok(row)
}

// --- Inspection Results ---
pub async fn list_inspection_results(
    pool: &PgPool,
    lot_id: Uuid,
) -> Result<Vec<InspectionResult>, AppError> {
    let rows = sqlx::query_as::<_, InspectionResult>(
        "SELECT * FROM qm_inspection_results WHERE inspection_lot_id = $1 ORDER BY characteristic",
    )
    .bind(lot_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_inspection_result(
    pool: &PgPool,
    input: &CreateInspectionResult,
    user_id: Uuid,
) -> Result<InspectionResult, AppError> {
    let row = sqlx::query_as::<_, InspectionResult>(
        "INSERT INTO qm_inspection_results (inspection_lot_id, characteristic, target_value, actual_value, is_conforming, inspected_by) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(input.inspection_lot_id).bind(&input.characteristic).bind(&input.target_value)
    .bind(&input.actual_value).bind(input.is_conforming).bind(user_id)
    .fetch_one(pool).await?;
    Ok(row)
}

// --- Quality Notifications ---
pub async fn list_quality_notifications(
    pool: &PgPool,
    params: &ListParams,
) -> Result<Vec<QualityNotification>, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let rows = sqlx::query_as::<_, QualityNotification>(
        r#"SELECT * FROM qm_quality_notifications
           WHERE ($1 = false OR (notification_number ILIKE $2 OR COALESCE(description, '') ILIKE $2))
             AND ($3 = false OR status = $4)
             AND ($5 = false OR priority = $6)
           ORDER BY created_at DESC
           LIMIT $7 OFFSET $8"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_filter_type)
    .bind(&filter_type)
    .bind(params.per_page())
    .bind(params.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_quality_notifications(
    pool: &PgPool,
    params: &ListParams,
) -> Result<i64, AppError> {
    let search_pattern = params.search_pattern();
    let has_search = search_pattern.is_some();
    let pattern = search_pattern.unwrap_or_default();

    let has_status = params.status.is_some();
    let status = params.status.clone().unwrap_or_default();

    let has_filter_type = params.filter_type.is_some();
    let filter_type = params.filter_type.clone().unwrap_or_default();

    let (count,): (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM qm_quality_notifications
           WHERE ($1 = false OR (notification_number ILIKE $2 OR COALESCE(description, '') ILIKE $2))
             AND ($3 = false OR status = $4)
             AND ($5 = false OR priority = $6)"#,
    )
    .bind(has_search)
    .bind(&pattern)
    .bind(has_status)
    .bind(&status)
    .bind(has_filter_type)
    .bind(&filter_type)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_quality_notification(
    pool: &PgPool,
    id: Uuid,
) -> Result<QualityNotification, AppError> {
    sqlx::query_as::<_, QualityNotification>("SELECT * FROM qm_quality_notifications WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Quality notification not found".to_string()))
}

pub async fn create_quality_notification(
    pool: &PgPool,
    notif_number: &str,
    input: &CreateQualityNotification,
    user_id: Uuid,
) -> Result<QualityNotification, AppError> {
    let row = sqlx::query_as::<_, QualityNotification>(
        "INSERT INTO qm_quality_notifications (notification_number, notification_type, material_id, description, priority, reported_by, assigned_to) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(notif_number).bind(&input.notification_type).bind(input.material_id).bind(&input.description)
    .bind(input.priority.as_deref().unwrap_or("MEDIUM")).bind(user_id).bind(input.assigned_to)
    .fetch_one(pool).await?;
    Ok(row)
}

pub async fn update_quality_notification(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateQualityNotification,
) -> Result<QualityNotification, AppError> {
    let row = sqlx::query_as::<_, QualityNotification>(
        "UPDATE qm_quality_notifications SET status = COALESCE($2, status), priority = COALESCE($3, priority), assigned_to = COALESCE($4, assigned_to), updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id).bind(&input.status).bind(&input.priority).bind(input.assigned_to)
    .fetch_optional(pool).await?
    .ok_or_else(|| AppError::NotFound("Quality notification not found".to_string()))?;
    Ok(row)
}

/// Update an inspection lot's status to the given value.
pub async fn update_inspection_lot_status(
    pool: &PgPool,
    id: Uuid,
    status: &str,
) -> Result<InspectionLot, AppError> {
    sqlx::query_as::<_, InspectionLot>(
        "UPDATE qm_inspection_lots SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(status)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Inspection lot not found".to_string()))
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
