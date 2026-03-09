use axum::extract::{Path, Query, State};
use axum::response::Response;
use axum::Json;
use sqlx;
use uuid::Uuid;
use validator::Validate;

use crate::auth::rbac::{MmRead, MmWrite, RequireRole};
use crate::mm::models::*;
use crate::mm::services;
use crate::shared::audit;
use crate::shared::export::csv_response;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, ListParams, PaginatedResponse};

// --- UOMs ---
pub async fn list_uoms(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Uom>>>, AppError> {
    let result = services::list_uoms(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_uom(
    State(state): State<AppState>,
    _role: RequireRole<MmWrite>,
    Json(input): Json<CreateUom>,
) -> Result<Json<ApiResponse<Uom>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let uom = services::create_uom(&state.pool, input).await?;
    Ok(Json(ApiResponse::with_message(uom, "UOM created")))
}

// --- Material Groups ---
pub async fn list_material_groups(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<MaterialGroup>>>, AppError> {
    let result = services::list_material_groups(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_material_group(
    State(state): State<AppState>,
    _role: RequireRole<MmWrite>,
    Json(input): Json<CreateMaterialGroup>,
) -> Result<Json<ApiResponse<MaterialGroup>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let group = services::create_material_group(&state.pool, input).await?;
    Ok(Json(ApiResponse::with_message(
        group,
        "Material group created",
    )))
}

// --- Materials ---
pub async fn list_materials(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Material>>>, AppError> {
    let result = services::list_materials(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_material(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Material>>, AppError> {
    let material = services::get_material(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(material)))
}

pub async fn create_material(
    State(state): State<AppState>,
    _role: RequireRole<MmWrite>,
    Json(input): Json<CreateMaterial>,
) -> Result<Json<ApiResponse<Material>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let material = services::create_material(&state.pool, input).await?;
    Ok(Json(ApiResponse::with_message(
        material,
        "Material created",
    )))
}

pub async fn update_material(
    State(state): State<AppState>,
    role: RequireRole<MmWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateMaterial>,
) -> Result<Json<ApiResponse<Material>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let material = services::update_material(&state.pool, id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "mm_materials",
        id,
        "UPDATE",
        None,
        serde_json::to_value(&material).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        material,
        "Material updated",
    )))
}

// --- Vendors ---
pub async fn list_vendors(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Vendor>>>, AppError> {
    let result = services::list_vendors(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_vendor(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vendor>>, AppError> {
    let vendor = services::get_vendor(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(vendor)))
}

pub async fn create_vendor(
    State(state): State<AppState>,
    _role: RequireRole<MmWrite>,
    Json(input): Json<CreateVendor>,
) -> Result<Json<ApiResponse<Vendor>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let vendor = services::create_vendor(&state.pool, input).await?;
    Ok(Json(ApiResponse::with_message(vendor, "Vendor created")))
}

pub async fn update_vendor(
    State(state): State<AppState>,
    role: RequireRole<MmWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateVendor>,
) -> Result<Json<ApiResponse<Vendor>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let vendor = services::update_vendor(&state.pool, id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "mm_vendors",
        id,
        "UPDATE",
        None,
        serde_json::to_value(&vendor).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(vendor, "Vendor updated")))
}

// --- Plant Stock ---
#[derive(serde::Deserialize)]
pub struct StockQuery {
    pub warehouse_id: Option<Uuid>,
    #[serde(flatten)]
    pub list: ListParams,
}

pub async fn list_plant_stock(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Query(query): Query<StockQuery>,
) -> Result<Json<ApiResponse<Vec<PlantStock>>>, AppError> {
    let stock = services::list_plant_stock(&state.pool, query.warehouse_id, &query.list).await?;
    Ok(Json(ApiResponse::success(stock)))
}

// --- Material Movements ---
pub async fn list_material_movements(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<MaterialMovement>>>, AppError> {
    let result = services::list_material_movements(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_material_movement(
    State(state): State<AppState>,
    role: RequireRole<MmWrite>,
    Json(input): Json<CreateMaterialMovement>,
) -> Result<Json<ApiResponse<MaterialMovement>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let movement = services::create_material_movement(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "mm_material_movements",
        movement.id,
        "CREATE",
        None,
        serde_json::to_value(&movement).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        movement,
        "Material movement posted",
    )))
}

// --- Purchase Orders ---
pub async fn list_purchase_orders(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<PurchaseOrder>>>, AppError> {
    let result = services::list_purchase_orders(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

#[derive(serde::Serialize)]
pub struct PurchaseOrderDetail {
    #[serde(flatten)]
    pub order: PurchaseOrder,
    pub items: Vec<PurchaseOrderItem>,
}

pub async fn get_purchase_order(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PurchaseOrderDetail>>, AppError> {
    let (order, items) = services::get_purchase_order(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(PurchaseOrderDetail {
        order,
        items,
    })))
}

pub async fn create_purchase_order(
    State(state): State<AppState>,
    role: RequireRole<MmWrite>,
    Json(input): Json<CreatePurchaseOrder>,
) -> Result<Json<ApiResponse<PurchaseOrder>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let po = services::create_purchase_order(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "mm_purchase_orders",
        po.id,
        "CREATE",
        None,
        serde_json::to_value(&po).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        po,
        "Purchase order created",
    )))
}

pub async fn release_purchase_order(
    State(state): State<AppState>,
    role: RequireRole<MmWrite>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PurchaseOrder>>, AppError> {
    let po = services::release_purchase_order(&state.pool, id, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "mm_purchase_orders",
        po.id,
        "UPDATE",
        serde_json::to_value(serde_json::json!({"status": "DRAFT"})).ok(),
        serde_json::to_value(serde_json::json!({"status": &po.status})).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        po,
        "Purchase order released",
    )))
}

pub async fn receive_purchase_order(
    State(state): State<AppState>,
    role: RequireRole<MmWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<ReceivePurchaseOrder>,
) -> Result<Json<ApiResponse<PurchaseOrder>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let po = services::receive_purchase_order(&state.pool, id, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "mm_purchase_orders",
        po.id,
        "UPDATE",
        None,
        serde_json::to_value(serde_json::json!({"status": &po.status, "action": "GOODS_RECEIPT"}))
            .ok(),
        Some(role.claims.sub),
    )
    .await;

    // Notify the user about PO goods receipt
    crate::notifications::services::notify(
        &state.pool,
        role.claims.sub,
        "Goods Received",
        &format!("Goods received for purchase order {}.", po.po_number),
        "success",
        Some("MM"),
        Some(po.id),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        po,
        "Goods received for purchase order",
    )))
}

// --- Delete Material (soft delete) ---
pub async fn delete_material(
    State(state): State<AppState>,
    role: RequireRole<MmWrite>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    // Check for active PO items referencing this material
    let (po_count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM mm_purchase_order_items poi \
         JOIN mm_purchase_orders po ON poi.purchase_order_id = po.id \
         WHERE poi.material_id = $1 AND po.status NOT IN ('CLOSED', 'CANCELLED')",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    if po_count > 0 {
        return Err(AppError::Validation(
            "Cannot delete material: active purchase orders reference this material".to_string(),
        ));
    }

    // Check for active BOM items
    let (bom_count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM pp_bom_items WHERE material_id = $1")
            .bind(id)
            .fetch_one(&state.pool)
            .await?;

    if bom_count > 0 {
        return Err(AppError::Validation(
            "Cannot delete material: BOM items reference this material".to_string(),
        ));
    }

    // Soft delete
    let result = sqlx::query(
        "UPDATE mm_materials SET is_active = false, updated_at = NOW() WHERE id = $1 AND is_active = true",
    )
    .bind(id)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Material not found or already inactive".to_string(),
        ));
    }

    let _ = audit::log_change(
        &state.pool,
        "mm_materials",
        id,
        "DELETE",
        serde_json::to_value(serde_json::json!({"is_active": true})).ok(),
        serde_json::to_value(serde_json::json!({"is_active": false})).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message((), "Material deactivated")))
}

// --- Export Materials ---
pub async fn export_materials(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,
) -> Result<Response, AppError> {
    let materials = sqlx::query_as::<_, Material>(
        "SELECT * FROM mm_materials WHERE is_active = true ORDER BY material_number",
    )
    .fetch_all(&state.pool)
    .await?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record([
        "Material Number",
        "Name",
        "Description",
        "Type",
        "Weight",
        "Weight UOM",
        "Active",
        "Created At",
    ])
    .map_err(|e| AppError::Internal(e.to_string()))?;

    for m in &materials {
        let weight = m.weight.map(|w| w.to_string()).unwrap_or_default();
        let created = m.created_at.to_rfc3339();
        let active = if m.is_active {
            "Yes".to_string()
        } else {
            "No".to_string()
        };
        wtr.write_record([
            m.material_number.as_str(),
            m.name.as_str(),
            m.description.as_deref().unwrap_or(""),
            m.material_type.as_str(),
            weight.as_str(),
            m.weight_uom.as_deref().unwrap_or(""),
            active.as_str(),
            created.as_str(),
        ])
        .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let csv_data = String::from_utf8(
        wtr.into_inner()
            .map_err(|e| AppError::Internal(e.to_string()))?,
    )
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(csv_response(csv_data, "materials-export.csv"))
}

// --- Goods Issue ---
pub async fn goods_issue(
    State(state): State<AppState>,
    role: RequireRole<MmWrite>,
    Json(input): Json<CreateMaterialMovement>,
) -> Result<Json<ApiResponse<MaterialMovement>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let movement_input = CreateMaterialMovement {
        movement_type: "GOODS_ISSUE".to_string(),
        material_id: input.material_id,
        warehouse_id: input.warehouse_id,
        quantity: input.quantity,
        uom_id: input.uom_id,
        reference_type: input.reference_type,
        reference_id: input.reference_id,
    };
    let movement =
        services::create_material_movement(&state.pool, movement_input, role.claims.sub).await?;
    Ok(Json(ApiResponse::with_message(
        movement,
        "Goods issue posted",
    )))
}
