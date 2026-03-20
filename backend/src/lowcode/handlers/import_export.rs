use axum::extract::{Path, State};
use axum::Json;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{self, PlatformUser, RequirePlatformRole};
use crate::lowcode::services::{field_validator, form_builder};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

pub async fn bulk_import(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(code): Path<String>,
    Json(input): Json<BulkImportRequest>,
) -> Result<Json<ApiResponse<BulkImportResult>>, AppError> {
    if input.records.is_empty() {
        return Ok(Json(ApiResponse::success(BulkImportResult {
            inserted: 0,
            skipped: 0,
            errors: vec![],
            row_errors: vec![],
        })));
    }

    let operation =
        sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE operation_code = $1")
            .bind(&code)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

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

    // Load form fields for validation
    let form = form_builder::get_form(&state.pool, operation.id).await?;
    let all_fields: Vec<_> = form
        .sections
        .iter()
        .flat_map(|s| s.fields.iter())
        .cloned()
        .collect();

    let mut inserted: usize = 0;
    let mut skipped: usize = 0;
    let mut errors: Vec<String> = Vec::new();
    let mut row_errors: Vec<BulkImportError> = Vec::new();

    // Validate all records first
    let mut validated_records = Vec::new();
    for (idx, record) in input.records.iter().enumerate() {
        if !record.is_object() {
            errors.push(format!("Record at index {} is not a JSON object", idx));
            row_errors.push(BulkImportError {
                row_index: idx,
                errors: vec![FieldError {
                    field_name: String::new(),
                    field_label: String::new(),
                    message: "Record is not a JSON object".to_string(),
                }],
            });
            continue;
        }

        let validation = field_validator::validate_for_create(
            &state.pool,
            operation.id,
            &all_fields,
            record,
            guard.claims.sub,
        )
        .await?;

        if validation.is_valid {
            validated_records.push((idx, validation.prepared_data));
        } else {
            row_errors.push(BulkImportError {
                row_index: idx,
                errors: validation.errors,
            });
            if !input.skip_invalid {
                errors.push(format!(
                    "Record at index {} has validation errors",
                    idx
                ));
            }
        }
    }

    // If dry_run, return validation results without writing
    if input.dry_run {
        return Ok(Json(ApiResponse::with_message(
            BulkImportResult {
                inserted: validated_records.len(),
                skipped: input.records.len() - validated_records.len(),
                errors,
                row_errors,
            },
            "Dry run completed",
        )));
    }

    // If not skip_invalid and there are errors, abort
    if !input.skip_invalid && !row_errors.is_empty() {
        return Ok(Json(ApiResponse::with_message(
            BulkImportResult {
                inserted: 0,
                skipped: 0,
                errors,
                row_errors,
            },
            "Import aborted due to validation errors",
        )));
    }

    // Insert validated records
    let mut tx = state.pool.begin().await?;
    for (_idx, data) in &validated_records {
        let result = sqlx::query(
            "INSERT INTO lc_operation_data (operation_id, data, created_by) VALUES ($1, $2, $3)",
        )
        .bind(operation.id)
        .bind(data)
        .bind(guard.claims.sub)
        .execute(&mut *tx)
        .await;

        match result {
            Ok(_) => inserted += 1,
            Err(e) => {
                errors.push(format!("Insert error: {}", e));
                skipped += 1;
            }
        }
    }
    tx.commit().await?;

    skipped += input.records.len() - validated_records.len();

    Ok(Json(ApiResponse::with_message(
        BulkImportResult {
            inserted,
            skipped,
            errors,
            row_errors,
        },
        format!("{} records imported", inserted),
    )))
}
