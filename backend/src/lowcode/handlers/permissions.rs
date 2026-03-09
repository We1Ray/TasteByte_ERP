use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformAdmin, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

// ── Operation Permissions ──────────────────────────────────────────────────

pub async fn list_operation_permissions(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(operation_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<OperationPermission>>>, AppError> {
    let perms = sqlx::query_as::<_, OperationPermission>(
        "SELECT * FROM lc_operation_permissions WHERE operation_id = $1 ORDER BY created_at",
    )
    .bind(operation_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(perms)))
}

pub async fn create_operation_permission(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(operation_id): Path<Uuid>,
    Json(input): Json<CreateOperationPermission>,
) -> Result<Json<ApiResponse<OperationPermission>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    if input.role_id.is_none() && input.user_id.is_none() {
        return Err(AppError::Validation(
            "Either role_id or user_id must be provided".to_string(),
        ));
    }

    let perm = sqlx::query_as::<_, OperationPermission>(
        "INSERT INTO lc_operation_permissions (operation_id, role_id, user_id, can_create, can_read, can_update, can_delete, custom_permissions) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
    )
    .bind(operation_id)
    .bind(input.role_id)
    .bind(input.user_id)
    .bind(input.can_create.unwrap_or(false))
    .bind(input.can_read.unwrap_or(true))
    .bind(input.can_update.unwrap_or(false))
    .bind(input.can_delete.unwrap_or(false))
    .bind(input.custom_permissions.unwrap_or(serde_json::json!({})))
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        perm,
        "Operation permission created",
    )))
}

pub async fn update_operation_permission(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path((operation_id, id)): Path<(Uuid, Uuid)>,
    Json(input): Json<UpdateOperationPermission>,
) -> Result<Json<ApiResponse<OperationPermission>>, AppError> {
    let existing = sqlx::query_as::<_, OperationPermission>(
        "SELECT * FROM lc_operation_permissions WHERE id = $1 AND operation_id = $2",
    )
    .bind(id)
    .bind(operation_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Permission not found".to_string()))?;

    let perm = sqlx::query_as::<_, OperationPermission>(
        "UPDATE lc_operation_permissions SET can_create = $3, can_read = $4, can_update = $5, can_delete = $6, custom_permissions = $7, updated_at = NOW() WHERE id = $1 AND operation_id = $2 RETURNING *",
    )
    .bind(id)
    .bind(operation_id)
    .bind(input.can_create.unwrap_or(existing.can_create))
    .bind(input.can_read.unwrap_or(existing.can_read))
    .bind(input.can_update.unwrap_or(existing.can_update))
    .bind(input.can_delete.unwrap_or(existing.can_delete))
    .bind(input.custom_permissions.unwrap_or(existing.custom_permissions))
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        perm,
        "Operation permission updated",
    )))
}

pub async fn delete_operation_permission(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path((operation_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let result =
        sqlx::query("DELETE FROM lc_operation_permissions WHERE id = $1 AND operation_id = $2")
            .bind(id)
            .bind(operation_id)
            .execute(&state.pool)
            .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Permission not found".to_string()));
    }

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "deleted": true }),
        "Operation permission deleted",
    )))
}

// ── Field Permissions ──────────────────────────────────────────────────────

pub async fn list_field_permissions(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(field_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<FieldPermission>>>, AppError> {
    let perms = sqlx::query_as::<_, FieldPermission>(
        "SELECT * FROM lc_field_permissions WHERE field_id = $1 ORDER BY created_at",
    )
    .bind(field_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(perms)))
}

pub async fn create_field_permission(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(field_id): Path<Uuid>,
    Json(input): Json<CreateFieldPermission>,
) -> Result<Json<ApiResponse<FieldPermission>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    if input.role_id.is_none() && input.user_id.is_none() {
        return Err(AppError::Validation(
            "Either role_id or user_id must be provided".to_string(),
        ));
    }

    let perm = sqlx::query_as::<_, FieldPermission>(
        "INSERT INTO lc_field_permissions (field_id, role_id, user_id, visibility, is_editable) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(field_id)
    .bind(input.role_id)
    .bind(input.user_id)
    .bind(input.visibility.as_deref().unwrap_or("VISIBLE"))
    .bind(input.is_editable.unwrap_or(true))
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        perm,
        "Field permission created",
    )))
}

pub async fn delete_field_permission(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path((_field_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let result = sqlx::query("DELETE FROM lc_field_permissions WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Permission not found".to_string()));
    }

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "deleted": true }),
        "Field permission deleted",
    )))
}

// ── Record Policies ────────────────────────────────────────────────────────

pub async fn list_record_policies(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(operation_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<RecordPolicy>>>, AppError> {
    let policies = sqlx::query_as::<_, RecordPolicy>(
        "SELECT * FROM lc_record_policies WHERE operation_id = $1 ORDER BY created_at",
    )
    .bind(operation_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(policies)))
}

pub async fn create_record_policy(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(operation_id): Path<Uuid>,
    Json(input): Json<CreateRecordPolicy>,
) -> Result<Json<ApiResponse<RecordPolicy>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let policy = sqlx::query_as::<_, RecordPolicy>(
        "INSERT INTO lc_record_policies (operation_id, role_id, user_id, policy_name, filter_sql, is_active) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(operation_id)
    .bind(input.role_id)
    .bind(input.user_id)
    .bind(&input.policy_name)
    .bind(&input.filter_sql)
    .bind(input.is_active.unwrap_or(true))
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        policy,
        "Record policy created",
    )))
}

pub async fn delete_record_policy(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path((_operation_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let result = sqlx::query("DELETE FROM lc_record_policies WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Policy not found".to_string()));
    }

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "deleted": true }),
        "Record policy deleted",
    )))
}
