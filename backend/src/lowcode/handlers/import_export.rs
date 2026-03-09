use axum::extract::{Path, State};
use axum::Json;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{self, PlatformUser, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

/// Bulk import records into an operation's data store.
/// Inserts each record from the request into lc_operation_data within a single transaction.
/// Returns the count of successfully inserted records and any errors.
pub async fn bulk_import(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(code): Path<String>,
    Json(input): Json<BulkImportRequest>,
) -> Result<Json<ApiResponse<BulkImportResult>>, AppError> {
    if input.records.is_empty() {
        return Ok(Json(ApiResponse::success(BulkImportResult {
            inserted: 0,
            errors: vec![],
        })));
    }

    // Resolve operation by code
    let operation =
        sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE operation_code = $1")
            .bind(&code)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    // Check create permission
    let perm = permission_resolver::resolve_operation_permission(
        &state.pool,
        operation.id,
        guard.claims.sub,
    )
    .await?;
    if !perm.can_create {
        return Err(AppError::Forbidden(
            "You do not have create access to this operation".to_string(),
        ));
    }

    let mut tx = state.pool.begin().await?;
    let mut inserted: usize = 0;
    let mut errors: Vec<String> = Vec::new();

    for (idx, record) in input.records.iter().enumerate() {
        // Validate that each record is a JSON object
        if !record.is_object() {
            errors.push(format!("Record at index {idx} is not a JSON object"));
            continue;
        }

        let result = sqlx::query(
            "INSERT INTO lc_operation_data (operation_id, data, created_by) VALUES ($1, $2, $3)",
        )
        .bind(operation.id)
        .bind(record)
        .bind(guard.claims.sub)
        .execute(&mut *tx)
        .await;

        match result {
            Ok(_) => inserted += 1,
            Err(e) => {
                errors.push(format!("Record at index {idx}: {e}"));
            }
        }
    }

    tx.commit().await?;

    Ok(Json(ApiResponse::with_message(
        BulkImportResult { inserted, errors },
        format!("{inserted} records imported"),
    )))
}
