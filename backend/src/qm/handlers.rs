use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::auth::rbac::{QmRead, QmWrite, RequireRole};
use crate::qm::models::*;
use crate::qm::services;
use crate::shared::audit;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, ListParams, PaginatedResponse};

// --- Inspection Lots ---
pub async fn list_inspection_lots(
    State(state): State<AppState>,
    _role: RequireRole<QmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<InspectionLot>>>, AppError> {
    let result = services::list_inspection_lots(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_inspection_lot(
    State(state): State<AppState>,
    _role: RequireRole<QmRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<InspectionLot>>, AppError> {
    let lot = services::get_inspection_lot(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(lot)))
}

pub async fn create_inspection_lot(
    State(state): State<AppState>,
    role: RequireRole<QmWrite>,
    Json(input): Json<CreateInspectionLot>,
) -> Result<Json<ApiResponse<InspectionLot>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let lot = services::create_inspection_lot(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "qm_inspection_lots",
        lot.id,
        "CREATE",
        None,
        serde_json::to_value(&lot).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        lot,
        "Inspection lot created",
    )))
}

// --- Inspection Results ---
pub async fn list_inspection_results(
    State(state): State<AppState>,
    _role: RequireRole<QmRead>,
    Path(lot_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<InspectionResult>>>, AppError> {
    let results = services::list_inspection_results(&state.pool, lot_id).await?;
    Ok(Json(ApiResponse::success(results)))
}

pub async fn create_inspection_result(
    State(state): State<AppState>,
    role: RequireRole<QmWrite>,
    Json(input): Json<CreateInspectionResult>,
) -> Result<Json<ApiResponse<InspectionResult>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let result = services::create_inspection_result(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "qm_inspection_results",
        result.id,
        "CREATE",
        None,
        serde_json::to_value(&result).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        result,
        "Inspection result recorded",
    )))
}

// --- Inspection Result Update/Delete ---
pub async fn update_inspection_result(
    State(state): State<AppState>,
    role: RequireRole<QmWrite>,
    Path(result_id): Path<Uuid>,
    Json(input): Json<UpdateInspectionResult>,
) -> Result<Json<ApiResponse<InspectionResult>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let result = services::update_inspection_result(&state.pool, result_id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "qm_inspection_results",
        result_id,
        "UPDATE",
        None,
        serde_json::to_value(&result).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        result,
        "Inspection result updated",
    )))
}

pub async fn delete_inspection_result(
    State(state): State<AppState>,
    role: RequireRole<QmWrite>,
    Path(result_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    services::delete_inspection_result(&state.pool, result_id).await?;

    let _ = audit::log_change(
        &state.pool,
        "qm_inspection_results",
        result_id,
        "DELETE",
        None,
        None,
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        (),
        "Inspection result deleted",
    )))
}

// --- Quality Notifications ---
pub async fn list_quality_notifications(
    State(state): State<AppState>,
    _role: RequireRole<QmRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<QualityNotification>>>, AppError> {
    let result = services::list_quality_notifications(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_quality_notification(
    State(state): State<AppState>,
    _role: RequireRole<QmRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<QualityNotification>>, AppError> {
    let notif = services::get_quality_notification(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(notif)))
}

pub async fn create_quality_notification(
    State(state): State<AppState>,
    role: RequireRole<QmWrite>,
    Json(input): Json<CreateQualityNotification>,
) -> Result<Json<ApiResponse<QualityNotification>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let notif = services::create_quality_notification(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "qm_quality_notifications",
        notif.id,
        "CREATE",
        None,
        serde_json::to_value(&notif).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        notif,
        "Quality notification created",
    )))
}

pub async fn update_quality_notification(
    State(state): State<AppState>,
    role: RequireRole<QmWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateQualityNotification>,
) -> Result<Json<ApiResponse<QualityNotification>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let notif = services::update_quality_notification(&state.pool, id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "qm_quality_notifications",
        id,
        "UPDATE",
        None,
        serde_json::to_value(&notif).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        notif,
        "Quality notification updated",
    )))
}

pub async fn complete_inspection(
    State(state): State<AppState>,
    role: RequireRole<QmWrite>,
    Path(lot_id): Path<Uuid>,
    Json(input): Json<CompleteInspectionLot>,
) -> Result<Json<ApiResponse<InspectionLot>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let passed = input.passed;
    let lot =
        services::complete_inspection_lot(&state.pool, lot_id, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "qm_inspection_lots",
        lot_id,
        "UPDATE",
        None,
        serde_json::to_value(serde_json::json!({
            "status": "COMPLETED",
            "passed": passed,
        }))
        .ok(),
        Some(role.claims.sub),
    )
    .await;

    let message = if passed {
        "Inspection completed - quality passed, stock released"
    } else {
        "Inspection completed - quality failed, stock held, notification created"
    };
    Ok(Json(ApiResponse::with_message(lot, message)))
}
