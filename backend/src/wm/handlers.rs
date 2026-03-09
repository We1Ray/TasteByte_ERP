use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::auth::rbac::{RequireRole, WmRead, WmWrite};
use crate::shared::audit;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, ListParams, PaginatedResponse};
use crate::wm::models::*;
use crate::wm::services;

// --- Warehouses ---
pub async fn list_warehouses(
    State(state): State<AppState>,
    _role: RequireRole<WmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Warehouse>>>, AppError> {
    let result = services::list_warehouses(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_warehouse(
    State(state): State<AppState>,
    _role: RequireRole<WmRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Warehouse>>, AppError> {
    let warehouse = services::get_warehouse(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(warehouse)))
}

pub async fn create_warehouse(
    State(state): State<AppState>,
    role: RequireRole<WmWrite>,
    Json(input): Json<CreateWarehouse>,
) -> Result<Json<ApiResponse<Warehouse>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let warehouse = services::create_warehouse(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "wm_warehouses",
        warehouse.id,
        "CREATE",
        None,
        serde_json::to_value(&warehouse).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        warehouse,
        "Warehouse created",
    )))
}

// --- Storage Bins ---
pub async fn list_storage_bins(
    State(state): State<AppState>,
    _role: RequireRole<WmRead>,
    Path(warehouse_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<StorageBin>>>, AppError> {
    let bins = services::list_storage_bins(&state.pool, warehouse_id).await?;
    Ok(Json(ApiResponse::success(bins)))
}

pub async fn create_storage_bin(
    State(state): State<AppState>,
    role: RequireRole<WmWrite>,
    Json(input): Json<CreateStorageBin>,
) -> Result<Json<ApiResponse<StorageBin>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let bin = services::create_storage_bin(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "wm_storage_bins",
        bin.id,
        "CREATE",
        None,
        serde_json::to_value(&bin).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(bin, "Storage bin created")))
}

// --- Stock Transfers ---
pub async fn list_stock_transfers(
    State(state): State<AppState>,
    _role: RequireRole<WmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<StockTransfer>>>, AppError> {
    let result = services::list_stock_transfers(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_stock_transfer(
    State(state): State<AppState>,
    role: RequireRole<WmWrite>,
    Json(input): Json<CreateStockTransfer>,
) -> Result<Json<ApiResponse<StockTransfer>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let transfer = services::create_stock_transfer(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "wm_stock_transfers",
        transfer.id,
        "CREATE",
        None,
        serde_json::to_value(&transfer).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        transfer,
        "Stock transfer created",
    )))
}

// --- Stock Counts ---
pub async fn list_stock_counts(
    State(state): State<AppState>,
    _role: RequireRole<WmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<StockCount>>>, AppError> {
    let result = services::list_stock_counts(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

#[derive(serde::Serialize)]
pub struct StockCountDetail {
    #[serde(flatten)]
    pub count: StockCount,
    pub items: Vec<StockCountItem>,
}

pub async fn get_stock_count(
    State(state): State<AppState>,
    _role: RequireRole<WmRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<StockCountDetail>>, AppError> {
    let (count, items) = services::get_stock_count(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(StockCountDetail {
        count,
        items,
    })))
}

pub async fn create_stock_count(
    State(state): State<AppState>,
    role: RequireRole<WmWrite>,
    Json(input): Json<CreateStockCount>,
) -> Result<Json<ApiResponse<StockCount>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let count = services::create_stock_count(&state.pool, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "wm_stock_counts",
        count.id,
        "CREATE",
        None,
        serde_json::to_value(&count).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        count,
        "Stock count created",
    )))
}

pub async fn complete_stock_count(
    State(state): State<AppState>,
    role: RequireRole<WmWrite>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<StockCountDetail>>, AppError> {
    let (count, items) = services::complete_stock_count(&state.pool, id, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "wm_stock_counts",
        count.id,
        "COMPLETE",
        None,
        serde_json::to_value(&count).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        StockCountDetail { count, items },
        "Stock count completed and MM plant stock reconciled",
    )))
}
