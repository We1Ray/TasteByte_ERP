use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::auth::rbac::{CoRead, CoWrite, RequireRole};
use crate::co::models::*;
use crate::co::services;
use crate::shared::audit;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, ListParams, PaginatedResponse};

// --- Cost Centers ---
pub async fn list_cost_centers(
    State(state): State<AppState>,
    _role: RequireRole<CoRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<CostCenter>>>, AppError> {
    let result = services::list_cost_centers(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_cost_center(
    State(state): State<AppState>,
    _role: RequireRole<CoRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<CostCenter>>, AppError> {
    let center = services::get_cost_center(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(center)))
}

pub async fn create_cost_center(
    State(state): State<AppState>,
    role: RequireRole<CoWrite>,
    Json(input): Json<CreateCostCenter>,
) -> Result<Json<ApiResponse<CostCenter>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let center = services::create_cost_center(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "co_cost_centers",
        center.id,
        "CREATE",
        None,
        serde_json::to_value(&center).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        center,
        "Cost center created",
    )))
}

// --- Profit Centers ---
pub async fn list_profit_centers(
    State(state): State<AppState>,
    _role: RequireRole<CoRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<ProfitCenter>>>, AppError> {
    let result = services::list_profit_centers(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_profit_center(
    State(state): State<AppState>,
    _role: RequireRole<CoRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProfitCenter>>, AppError> {
    let center = services::get_profit_center(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(center)))
}

pub async fn create_profit_center(
    State(state): State<AppState>,
    role: RequireRole<CoWrite>,
    Json(input): Json<CreateProfitCenter>,
) -> Result<Json<ApiResponse<ProfitCenter>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let center = services::create_profit_center(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "co_profit_centers",
        center.id,
        "CREATE",
        None,
        serde_json::to_value(&center).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        center,
        "Profit center created",
    )))
}

// --- Internal Orders ---
pub async fn list_internal_orders(
    State(state): State<AppState>,
    _role: RequireRole<CoRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<InternalOrder>>>, AppError> {
    let result = services::list_internal_orders(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_internal_order(
    State(state): State<AppState>,
    _role: RequireRole<CoRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<InternalOrder>>, AppError> {
    let order = services::get_internal_order(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(order)))
}

pub async fn create_internal_order(
    State(state): State<AppState>,
    role: RequireRole<CoWrite>,
    Json(input): Json<CreateInternalOrder>,
) -> Result<Json<ApiResponse<InternalOrder>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let order = services::create_internal_order(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "co_internal_orders",
        order.id,
        "CREATE",
        None,
        serde_json::to_value(&order).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        order,
        "Internal order created",
    )))
}

pub async fn update_internal_order(
    State(state): State<AppState>,
    role: RequireRole<CoWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateInternalOrder>,
) -> Result<Json<ApiResponse<InternalOrder>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let order = services::update_internal_order(&state.pool, id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "co_internal_orders",
        id,
        "UPDATE",
        None,
        serde_json::to_value(&order).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        order,
        "Internal order updated",
    )))
}

// --- Cost Allocations ---
pub async fn list_cost_allocations(
    State(state): State<AppState>,
    _role: RequireRole<CoRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<CostAllocation>>>, AppError> {
    let result = services::list_cost_allocations(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_cost_allocation(
    State(state): State<AppState>,
    role: RequireRole<CoWrite>,
    Json(input): Json<CreateCostAllocation>,
) -> Result<Json<ApiResponse<CostAllocation>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let allocation = services::create_cost_allocation(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "co_cost_allocations",
        allocation.id,
        "CREATE",
        None,
        serde_json::to_value(&allocation).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        allocation,
        "Cost allocation created",
    )))
}
