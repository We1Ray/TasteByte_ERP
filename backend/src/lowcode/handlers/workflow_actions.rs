use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{self, PlatformUser, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

/// Transition the status of a record in an operation's data store.
/// Updates the status field inside the record's JSONB data and optionally
/// logs the transition to lc_document_flows for audit purposes.
pub async fn transition_status(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path((code, record_id)): Path<(String, Uuid)>,
    Json(input): Json<TransitionRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    // Resolve operation by code
    let operation =
        sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE operation_code = $1")
            .bind(&code)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    // Check update permission
    let perm = permission_resolver::resolve_operation_permission(
        &state.pool,
        operation.id,
        guard.claims.sub,
    )
    .await?;
    if !perm.can_update {
        return Err(AppError::Forbidden(
            "You do not have update access to this operation".to_string(),
        ));
    }

    // Get the current record
    let record = sqlx::query_as::<_, OperationData>(
        "SELECT * FROM lc_operation_data WHERE id = $1 AND operation_id = $2",
    )
    .bind(record_id)
    .bind(operation.id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Record not found".to_string()))?;

    // Determine the status field name (default: "status")
    let status_field = input.status_field.as_deref().unwrap_or("status");

    // Update the status in the JSONB data
    let mut data = record.data.clone();
    let old_status = data
        .get(status_field)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if let Some(obj) = data.as_object_mut() {
        obj.insert(
            status_field.to_string(),
            serde_json::Value::String(input.target_status.clone()),
        );

        // Add transition metadata
        obj.insert(
            "_last_transition_by".to_string(),
            serde_json::Value::String(guard.claims.sub.to_string()),
        );
        obj.insert(
            "_last_transition_at".to_string(),
            serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
        );

        if let Some(ref comment) = input.comment {
            obj.insert(
                "_transition_comment".to_string(),
                serde_json::Value::String(comment.clone()),
            );
        }
    } else {
        return Err(AppError::Validation(
            "Record data is not a JSON object".to_string(),
        ));
    }

    // Persist the updated data
    let updated = sqlx::query_as::<_, OperationData>(
        "UPDATE lc_operation_data SET data = $3, updated_at = NOW() WHERE id = $1 AND operation_id = $2 RETURNING *",
    )
    .bind(record_id)
    .bind(operation.id)
    .bind(&data)
    .fetch_one(&state.pool)
    .await?;

    // Log the transition to document_flows for audit trail
    let _ = sqlx::query(
        "INSERT INTO lc_document_flows (source_type, source_id, target_type, target_id, flow_type) \
         VALUES ($1, $2, $3, $4, $5) \
         ON CONFLICT (source_type, source_id, target_type, target_id) DO NOTHING",
    )
    .bind(format!("{}:{}", operation.operation_code, old_status))
    .bind(record_id)
    .bind(format!("{}:{}", operation.operation_code, input.target_status))
    .bind(record_id)
    .bind("STATUS_TRANSITION")
    .execute(&state.pool)
    .await;

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({
            "record": updated,
            "transition": {
                "from": old_status,
                "to": input.target_status,
                "by": guard.claims.sub,
            }
        }),
        format!(
            "Status transitioned from '{}' to '{}'",
            old_status, input.target_status
        ),
    )))
}
