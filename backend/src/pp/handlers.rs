use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::auth::rbac::{PpRead, PpWrite, RequireRole};
use crate::pp::models::*;
use crate::pp::services;
use crate::shared::audit;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, ListParams, PaginatedResponse};

// --- BOMs ---
pub async fn list_boms(
    State(state): State<AppState>,
    _role: RequireRole<PpRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Bom>>>, AppError> {
    let result = services::list_boms(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

#[derive(serde::Serialize)]
pub struct BomDetail {
    #[serde(flatten)]
    pub bom: Bom,
    pub items: Vec<BomItem>,
}

pub async fn get_bom(
    State(state): State<AppState>,
    _role: RequireRole<PpRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<BomDetail>>, AppError> {
    let (bom, items) = services::get_bom(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(BomDetail { bom, items })))
}

pub async fn create_bom(
    State(state): State<AppState>,
    _role: RequireRole<PpWrite>,
    Json(input): Json<CreateBom>,
) -> Result<Json<ApiResponse<Bom>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let bom = services::create_bom(&state.pool, input).await?;
    Ok(Json(ApiResponse::with_message(bom, "BOM created")))
}

// --- BOM Items (sub-table CRUD) ---
pub async fn add_bom_item(
    State(state): State<AppState>,
    _role: RequireRole<PpWrite>,
    Path(bom_id): Path<Uuid>,
    Json(input): Json<AddBomItem>,
) -> Result<Json<ApiResponse<BomItem>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let item = services::add_bom_item(&state.pool, bom_id, input).await?;
    Ok(Json(ApiResponse::with_message(item, "BOM item added")))
}

pub async fn update_bom_item(
    State(state): State<AppState>,
    _role: RequireRole<PpWrite>,
    Path((bom_id, item_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<UpdateBomItem>,
) -> Result<Json<ApiResponse<BomItem>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let item = services::update_bom_item(&state.pool, bom_id, item_id, input).await?;
    Ok(Json(ApiResponse::with_message(item, "BOM item updated")))
}

pub async fn delete_bom_item(
    State(state): State<AppState>,
    _role: RequireRole<PpWrite>,
    Path((bom_id, item_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    services::delete_bom_item(&state.pool, bom_id, item_id).await?;
    Ok(Json(ApiResponse::with_message((), "BOM item deleted")))
}

// --- Routings ---
pub async fn list_routings(
    State(state): State<AppState>,
    _role: RequireRole<PpRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Routing>>>, AppError> {
    let result = services::list_routings(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

#[derive(serde::Serialize)]
pub struct RoutingDetail {
    #[serde(flatten)]
    pub routing: Routing,
    pub operations: Vec<RoutingOperation>,
}

pub async fn get_routing(
    State(state): State<AppState>,
    _role: RequireRole<PpRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<RoutingDetail>>, AppError> {
    let (routing, operations) = services::get_routing(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(RoutingDetail {
        routing,
        operations,
    })))
}

pub async fn create_routing(
    State(state): State<AppState>,
    _role: RequireRole<PpWrite>,
    Json(input): Json<CreateRouting>,
) -> Result<Json<ApiResponse<Routing>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let routing = services::create_routing(&state.pool, input).await?;
    Ok(Json(ApiResponse::with_message(routing, "Routing created")))
}

// --- Routing Operations (sub-table CRUD) ---
pub async fn add_routing_operation(
    State(state): State<AppState>,
    _role: RequireRole<PpWrite>,
    Path(routing_id): Path<Uuid>,
    Json(input): Json<AddRoutingOperation>,
) -> Result<Json<ApiResponse<RoutingOperation>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let op = services::add_routing_operation(&state.pool, routing_id, input).await?;
    Ok(Json(ApiResponse::with_message(
        op,
        "Routing operation added",
    )))
}

pub async fn update_routing_operation(
    State(state): State<AppState>,
    _role: RequireRole<PpWrite>,
    Path((routing_id, op_id)): Path<(Uuid, Uuid)>,
    Json(input): Json<UpdateRoutingOperation>,
) -> Result<Json<ApiResponse<RoutingOperation>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let op = services::update_routing_operation(&state.pool, routing_id, op_id, input).await?;
    Ok(Json(ApiResponse::with_message(
        op,
        "Routing operation updated",
    )))
}

pub async fn delete_routing_operation(
    State(state): State<AppState>,
    _role: RequireRole<PpWrite>,
    Path((routing_id, op_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    services::delete_routing_operation(&state.pool, routing_id, op_id).await?;
    Ok(Json(ApiResponse::with_message(
        (),
        "Routing operation deleted",
    )))
}

// --- Production Orders ---
pub async fn list_production_orders(
    State(state): State<AppState>,
    _role: RequireRole<PpRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<ProductionOrder>>>, AppError> {
    let result = services::list_production_orders(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_production_order(
    State(state): State<AppState>,
    _role: RequireRole<PpRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProductionOrder>>, AppError> {
    let order = services::get_production_order(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(order)))
}

pub async fn create_production_order(
    State(state): State<AppState>,
    role: RequireRole<PpWrite>,
    Json(input): Json<CreateProductionOrder>,
) -> Result<Json<ApiResponse<ProductionOrder>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let order = services::create_production_order(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "pp_production_orders",
        order.id,
        "CREATE",
        None,
        serde_json::to_value(&order).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        order,
        "Production order created",
    )))
}

pub async fn update_production_order_status(
    State(state): State<AppState>,
    role: RequireRole<PpWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateProductionOrderStatus>,
) -> Result<Json<ApiResponse<ProductionOrder>>, AppError> {
    let new_status = input.status.clone();
    let order =
        services::update_production_order_status(&state.pool, id, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "pp_production_orders",
        order.id,
        "UPDATE",
        None,
        serde_json::to_value(serde_json::json!({"status": new_status})).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        order,
        "Production order status updated",
    )))
}

pub async fn release_production_order(
    State(state): State<AppState>,
    role: RequireRole<PpWrite>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProductionOrder>>, AppError> {
    let input = UpdateProductionOrderStatus {
        status: "RELEASED".to_string(),
        actual_quantity: None,
    };
    let order =
        services::update_production_order_status(&state.pool, id, input, role.claims.sub).await?;
    Ok(Json(ApiResponse::with_message(
        order,
        "Production order released",
    )))
}

pub async fn confirm_production_order(
    State(state): State<AppState>,
    role: RequireRole<PpWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<ConfirmProductionOrder>,
) -> Result<Json<ApiResponse<ProductionOrder>>, AppError> {
    let status_input = UpdateProductionOrderStatus {
        status: "COMPLETED".to_string(),
        actual_quantity: Some(input.quantity),
    };
    let order =
        services::update_production_order_status(&state.pool, id, status_input, role.claims.sub)
            .await?;
    Ok(Json(ApiResponse::with_message(
        order,
        "Production order confirmed",
    )))
}
