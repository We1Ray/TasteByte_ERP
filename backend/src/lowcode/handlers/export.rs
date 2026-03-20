use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use serde_json::Value;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{self, PlatformUser, RequirePlatformRole};
use crate::lowcode::services::{form_builder, sql_engine};
use crate::shared::types::AppState;
use crate::shared::AppError;

pub async fn export_data(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(code): Path<String>,
    Query(params): Query<ExportParams>,
) -> Result<Response, AppError> {
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
    if !perm.can_read {
        return Err(AppError::Forbidden(
            "You do not have read access to this operation".to_string(),
        ));
    }

    // ── Apply Record Policies ──────────────────────────────────────────
    let policies: Vec<RecordPolicy> = sqlx::query_as(
        "SELECT rp.* FROM lc_record_policies rp \
         WHERE rp.operation_id = $1 AND rp.is_active = true \
         AND (rp.user_id = $2 OR rp.role_id IN (\
             SELECT upr.role_id FROM lc_user_platform_roles upr WHERE upr.user_id = $2\
         ))",
    )
    .bind(operation.id)
    .bind(guard.claims.sub)
    .fetch_all(&state.pool)
    .await?;

    let mut policy_conditions = String::new();
    for policy in &policies {
        if let Err(err) = sql_engine::validate_filter_expression(&policy.filter_sql) {
            tracing::warn!(
                "Skipping unsafe record policy {} ({}): {}",
                policy.id,
                err,
                policy.filter_sql
            );
            continue;
        }
        policy_conditions.push_str(&format!(" AND ({})", policy.filter_sql));
    }

    let query_sql = format!(
        "SELECT * FROM lc_operation_data WHERE operation_id = $1{} ORDER BY created_at DESC",
        policy_conditions
    );
    let records = sqlx::query_as::<_, OperationData>(&query_sql)
        .bind(operation.id)
        .fetch_all(&state.pool)
        .await?;

    // ── Resolve Field Permissions ──────────────────────────────────────
    let form = form_builder::get_form(&state.pool, operation.id).await?;
    let all_fields: Vec<_> = form
        .sections
        .iter()
        .flat_map(|s| s.fields.iter())
        .collect();

    let mut hidden_fields: Vec<String> = Vec::new();
    let mut masked_fields: Vec<String> = Vec::new();

    for fwo in &all_fields {
        let fp = permission_resolver::resolve_field_permission(
            &state.pool,
            fwo.field.id,
            guard.claims.sub,
        )
        .await?;
        if let Some(ref fp) = fp {
            if fp.visibility == "HIDDEN" {
                hidden_fields.push(fwo.field.field_name.clone());
            } else if fp.visibility == "MASKED" {
                masked_fields.push(fwo.field.field_name.clone());
            }
        }
    }

    // ── Apply field filtering to records ───────────────────────────────
    let filtered_records: Vec<(uuid::Uuid, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, Value)> = records
        .into_iter()
        .map(|r| {
            let mut data = r.data;
            if let Value::Object(ref mut map) = data {
                // Remove hidden fields
                for hf in &hidden_fields {
                    map.remove(hf);
                }
                // Mask masked fields
                for mf in &masked_fields {
                    if map.contains_key(mf) {
                        map.insert(mf.clone(), Value::String("***".to_string()));
                    }
                }
            }
            (r.id, r.created_at, r.updated_at, data)
        })
        .collect();

    let format = params.format.as_deref().unwrap_or("json");

    match format {
        "csv" => {
            let mut wtr = csv::Writer::from_writer(Vec::new());

            // Collect keys, excluding hidden fields
            let mut all_keys: Vec<String> = Vec::new();
            for (_, _, _, data) in &filtered_records {
                if let Value::Object(map) = data {
                    for key in map.keys() {
                        if !all_keys.contains(key) && !hidden_fields.contains(key) {
                            all_keys.push(key.clone());
                        }
                    }
                }
            }
            all_keys.sort();

            let mut headers = vec![
                "id".to_string(),
                "created_at".to_string(),
                "updated_at".to_string(),
            ];
            headers.extend(all_keys.clone());
            wtr.write_record(&headers)
                .map_err(|e| AppError::Internal(format!("CSV error: {}", e)))?;

            for (id, created_at, updated_at, data) in &filtered_records {
                let mut row = vec![
                    id.to_string(),
                    created_at.to_rfc3339(),
                    updated_at.to_rfc3339(),
                ];
                for key in &all_keys {
                    let val = data
                        .get(key)
                        .map(|v| match v {
                            Value::String(s) => s.clone(),
                            Value::Null => String::new(),
                            other => other.to_string(),
                        })
                        .unwrap_or_default();
                    row.push(val);
                }
                wtr.write_record(&row)
                    .map_err(|e| AppError::Internal(format!("CSV error: {}", e)))?;
            }

            let csv_bytes = wtr
                .into_inner()
                .map_err(|e| AppError::Internal(format!("CSV error: {}", e)))?;
            let filename = format!("{}.csv", code);

            Ok((
                [
                    (header::CONTENT_TYPE, "text/csv; charset=utf-8".to_string()),
                    (
                        header::CONTENT_DISPOSITION,
                        format!("attachment; filename=\"{}\"", filename),
                    ),
                ],
                csv_bytes,
            )
                .into_response())
        }
        _ => {
            let items: Vec<Value> = filtered_records
                .into_iter()
                .map(|(id, created_at, updated_at, data)| {
                    let mut obj = match data {
                        Value::Object(map) => map,
                        other => {
                            let mut m = serde_json::Map::new();
                            m.insert("data".to_string(), other);
                            m
                        }
                    };
                    obj.insert("id".to_string(), serde_json::json!(id));
                    obj.insert("created_at".to_string(), serde_json::json!(created_at));
                    obj.insert("updated_at".to_string(), serde_json::json!(updated_at));
                    Value::Object(obj)
                })
                .collect();

            let json_bytes = serde_json::to_vec_pretty(&items)
                .map_err(|e| AppError::Internal(format!("JSON error: {}", e)))?;
            let filename = format!("{}.json", code);

            Ok((
                [
                    (
                        header::CONTENT_TYPE,
                        "application/json; charset=utf-8".to_string(),
                    ),
                    (
                        header::CONTENT_DISPOSITION,
                        format!("attachment; filename=\"{}\"", filename),
                    ),
                ],
                json_bytes,
            )
                .into_response())
        }
    }
}
