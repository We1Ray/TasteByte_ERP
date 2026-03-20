use axum::extract::{Path, Query, State};
use axum::Json;
use sqlx::{Column, PgPool, Row};
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformUser, RequirePlatformRole};
use crate::lowcode::services::{form_builder, permission_resolver, sql_engine};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, PaginatedResponse, PaginationParams};

/// Get form definition by operation_code for rendering
pub async fn get_form_by_code(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(code): Path<String>,
) -> Result<Json<ApiResponse<FormResponse>>, AppError> {
    let operation = sqlx::query_as::<_, Operation>(
        "SELECT * FROM lc_operations WHERE operation_code = $1 AND is_published = true",
    )
    .bind(&code)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Operation not found or not published".to_string()))?;

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

    let mut form = form_builder::get_form(&state.pool, operation.id).await?;

    // Apply field visibility permissions
    for section in &mut form.sections {
        let mut filtered_fields = Vec::new();
        for field_with_opts in section.fields.drain(..) {
            let field_perm = permission_resolver::resolve_field_permission(
                &state.pool,
                field_with_opts.field.id,
                guard.claims.sub,
            )
            .await?;

            match field_perm {
                Some(ref fp) if fp.visibility == "HIDDEN" => {
                    // Skip this field entirely
                    continue;
                }
                Some(ref fp) if fp.visibility == "MASKED" => {
                    // Keep field but mark as masked in config
                    let mut masked_field = field_with_opts;
                    if let serde_json::Value::Object(ref mut config) =
                        masked_field.field.field_config
                    {
                        config.insert("is_masked".to_string(), serde_json::Value::Bool(true));
                    }
                    filtered_fields.push(masked_field);
                }
                _ => {
                    // VISIBLE or no specific permission - show normally
                    filtered_fields.push(field_with_opts);
                }
            }
        }
        section.fields = filtered_fields;
    }

    Ok(Json(ApiResponse::success(form)))
}

/// List records for an operation
pub async fn list_data(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(code): Path<String>,
    Query(query): Query<DataListQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<OperationData>>>, AppError> {
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

    // Fetch active record policies for this user/operation
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

    // Build additional WHERE clause from policies (validated via sqlparser)
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

    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let page = query.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    let (count, rows) = if let Some(ref search) = query.search {
        let pattern = format!("%{search}%");
        let count_sql = format!(
            "SELECT COUNT(*) FROM lc_operation_data WHERE operation_id = $1 AND data::text ILIKE $2{}",
            policy_conditions
        );
        let (c,): (i64,) = sqlx::query_as(&count_sql)
            .bind(operation.id)
            .bind(&pattern)
            .fetch_one(&state.pool)
            .await?;

        let data_sql = format!(
            "SELECT * FROM lc_operation_data WHERE operation_id = $1 AND data::text ILIKE $2{} ORDER BY created_at DESC LIMIT $3 OFFSET $4",
            policy_conditions
        );
        let r = sqlx::query_as::<_, OperationData>(&data_sql)
            .bind(operation.id)
            .bind(&pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.pool)
            .await?;
        (c, r)
    } else {
        let count_sql = format!(
            "SELECT COUNT(*) FROM lc_operation_data WHERE operation_id = $1{}",
            policy_conditions
        );
        let (c,): (i64,) = sqlx::query_as(&count_sql)
            .bind(operation.id)
            .fetch_one(&state.pool)
            .await?;

        let data_sql = format!(
            "SELECT * FROM lc_operation_data WHERE operation_id = $1{} ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            policy_conditions
        );
        let r = sqlx::query_as::<_, OperationData>(&data_sql)
            .bind(operation.id)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.pool)
            .await?;
        (c, r)
    };

    let params = PaginationParams {
        page: Some(page),
        per_page: Some(per_page),
    };
    Ok(Json(ApiResponse::success(PaginatedResponse::new(
        rows, count, &params,
    ))))
}

/// Create a new record
pub async fn create_data(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(code): Path<String>,
    Json(input): Json<CreateOperationData>,
) -> Result<Json<ApiResponse<OperationData>>, AppError> {
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

    // Field validation
    let form = form_builder::get_form(&state.pool, operation.id).await?;
    let all_fields: Vec<_> = form
        .sections
        .iter()
        .flat_map(|s| s.fields.iter())
        .cloned()
        .collect();
    let validation = crate::lowcode::services::field_validator::validate_for_create(
        &state.pool,
        operation.id,
        &all_fields,
        &input.data,
        guard.claims.sub,
    )
    .await?;

    if !validation.is_valid {
        return Err(AppError::Validation(
            serde_json::to_string(&validation.errors)
                .unwrap_or_else(|_| "Validation failed".to_string()),
        ));
    }

    // Cross-field validation
    let cf_rules = crate::shared::cross_field::list_rules(&state.pool, operation.id).await?;
    if !cf_rules.is_empty() {
        let cf_errors = crate::shared::cross_field::validate_cross_field_rules(
            &cf_rules,
            &validation.prepared_data,
        );
        if !cf_errors.is_empty() {
            return Err(AppError::Validation(
                serde_json::to_string(&cf_errors)
                    .unwrap_or_else(|_| "Cross-field validation failed".to_string()),
            ));
        }
    }

    // Apply calculation formulas
    let formulas = crate::shared::cross_field::list_formulas(&state.pool, operation.id).await?;
    let insert_data = if !formulas.is_empty() {
        let mut prepared = validation.prepared_data.clone();
        crate::shared::cross_field::apply_formulas(&formulas, &mut prepared);
        prepared
    } else {
        validation.prepared_data.clone()
    };

    let record = sqlx::query_as::<_, OperationData>(
        "INSERT INTO lc_operation_data (operation_id, data, created_by) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(operation.id)
    .bind(&insert_data)
    .bind(guard.claims.sub)
    .fetch_one(&state.pool)
    .await?;

    // Fire output determination rules
    crate::shared::output_determination::fire_outputs(
        &state.pool,
        operation.id,
        record.id,
        "ON_CREATE",
        &record.data,
    )
    .await;

    // Audit log
    if let Err(e) = crate::shared::audit::log_change(
        &state.pool,
        "lc_operation_data",
        record.id,
        "INSERT",
        None,
        Some(insert_data),
        Some(guard.claims.sub),
    )
    .await
    {
        tracing::warn!("Audit log failed: {}", e);
    }

    Ok(Json(ApiResponse::with_message(record, "Record created")))
}

/// Get a single record
pub async fn get_data(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path((code, id)): Path<(String, Uuid)>,
) -> Result<Json<ApiResponse<OperationData>>, AppError> {
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

    let record = sqlx::query_as::<_, OperationData>(
        "SELECT * FROM lc_operation_data WHERE id = $1 AND operation_id = $2",
    )
    .bind(id)
    .bind(operation.id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Record not found".to_string()))?;

    Ok(Json(ApiResponse::success(record)))
}

/// Update a record
pub async fn update_data(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path((code, id)): Path<(String, Uuid)>,
    Json(input): Json<UpdateOperationData>,
) -> Result<Json<ApiResponse<OperationData>>, AppError> {
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
    if !perm.can_update {
        return Err(AppError::Forbidden(
            "You do not have update access to this operation".to_string(),
        ));
    }

    // Get old record for audit
    let old_record = sqlx::query_as::<_, OperationData>(
        "SELECT * FROM lc_operation_data WHERE id = $1 AND operation_id = $2",
    )
    .bind(id)
    .bind(operation.id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Record not found".to_string()))?;

    // Determine masked fields
    let form = form_builder::get_form(&state.pool, operation.id).await?;
    let all_fields: Vec<_> = form
        .sections
        .iter()
        .flat_map(|s| s.fields.iter())
        .cloned()
        .collect();
    let mut masked_field_names = Vec::new();
    for fwo in &all_fields {
        let fp = permission_resolver::resolve_field_permission(
            &state.pool,
            fwo.field.id,
            guard.claims.sub,
        )
        .await?;
        if let Some(ref fp) = fp {
            if fp.visibility == "MASKED" {
                masked_field_names.push(fwo.field.field_name.clone());
            }
        }
    }

    // Field validation
    let validation = crate::lowcode::services::field_validator::validate_for_update(
        &state.pool,
        operation.id,
        id,
        &all_fields,
        &input.data,
        guard.claims.sub,
        &masked_field_names,
    )
    .await?;

    if !validation.is_valid {
        return Err(AppError::Validation(
            serde_json::to_string(&validation.errors)
                .unwrap_or_else(|_| "Validation failed".to_string()),
        ));
    }

    // Cross-field validation
    let cf_rules = crate::shared::cross_field::list_rules(&state.pool, operation.id).await?;
    if !cf_rules.is_empty() {
        let cf_errors = crate::shared::cross_field::validate_cross_field_rules(
            &cf_rules,
            &validation.prepared_data,
        );
        if !cf_errors.is_empty() {
            return Err(AppError::Validation(
                serde_json::to_string(&cf_errors)
                    .unwrap_or_else(|_| "Cross-field validation failed".to_string()),
            ));
        }
    }

    // Apply calculation formulas
    let formulas = crate::shared::cross_field::list_formulas(&state.pool, operation.id).await?;
    let update_data = if !formulas.is_empty() {
        let mut prepared = validation.prepared_data.clone();
        crate::shared::cross_field::apply_formulas(&formulas, &mut prepared);
        prepared
    } else {
        validation.prepared_data.clone()
    };

    let record = sqlx::query_as::<_, OperationData>(
        "UPDATE lc_operation_data SET data = $3, updated_at = NOW() WHERE id = $1 AND operation_id = $2 RETURNING *",
    )
    .bind(id)
    .bind(operation.id)
    .bind(&update_data)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Record not found".to_string()))?;

    // Fire output determination rules
    crate::shared::output_determination::fire_outputs(
        &state.pool,
        operation.id,
        record.id,
        "ON_UPDATE",
        &record.data,
    )
    .await;

    // Audit log
    if let Err(e) = crate::shared::audit::log_change(
        &state.pool,
        "lc_operation_data",
        record.id,
        "UPDATE",
        Some(old_record.data),
        Some(update_data),
        Some(guard.claims.sub),
    )
    .await
    {
        tracing::warn!("Audit log failed: {}", e);
    }

    Ok(Json(ApiResponse::with_message(record, "Record updated")))
}

/// Delete a record
pub async fn delete_data(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path((code, id)): Path<(String, Uuid)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
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
    if !perm.can_delete {
        return Err(AppError::Forbidden(
            "You do not have delete access to this operation".to_string(),
        ));
    }

    // Get old record for audit
    let old_record = sqlx::query_as::<_, OperationData>(
        "SELECT * FROM lc_operation_data WHERE id = $1 AND operation_id = $2",
    )
    .bind(id)
    .bind(operation.id)
    .fetch_optional(&state.pool)
    .await?;

    let result = sqlx::query("DELETE FROM lc_operation_data WHERE id = $1 AND operation_id = $2")
        .bind(id)
        .bind(operation.id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Record not found".to_string()));
    }

    // Audit log
    if let Some(old) = old_record {
        if let Err(e) = crate::shared::audit::log_change(
            &state.pool,
            "lc_operation_data",
            id,
            "DELETE",
            Some(old.data),
            None,
            Some(guard.claims.sub),
        )
        .await
        {
            tracing::warn!("Audit log failed: {}", e);
        }
    }

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "deleted": true }),
        "Record deleted",
    )))
}

/// List query endpoint for the ListRenderer.
/// If the list definition has a `data_source_sql`, executes it with pagination.
/// Otherwise falls back to querying `lc_operation_data`.
pub async fn list_query(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(code): Path<String>,
    Query(params): Query<ListQueryParams>,
) -> Result<Json<ListQueryResponse>, AppError> {
    // 1. Look up operation by code (must be published)
    let operation = sqlx::query_as::<_, Operation>(
        "SELECT * FROM lc_operations WHERE operation_code = $1 AND is_published = true",
    )
    .bind(&code)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Operation not found or not published".to_string()))?;

    // 2. Check read permission
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

    // 3. Pagination defaults
    let page_size = params.page_size.unwrap_or(20).clamp(1, 500);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * page_size;

    // 4. Load list definition (optional -- may not exist yet)
    let list_def = sqlx::query_as::<_, ListDefinitionRow>(
        "SELECT * FROM lc_list_definitions WHERE operation_id = $1",
    )
    .bind(operation.id)
    .fetch_optional(&state.pool)
    .await?;

    let data_source_sql = list_def
        .as_ref()
        .and_then(|ld| ld.data_source_sql.as_deref());

    let (items, total) = if let Some(sql) = data_source_sql {
        // --- SQL data source path ---
        execute_list_sql(
            &state.pool,
            sql,
            page_size,
            offset,
            params.search.as_deref(),
            params.sort_by.as_deref(),
            params.sort_order.as_deref(),
        )
        .await?
    } else {
        // --- Fallback: lc_operation_data path ---
        execute_list_operation_data(
            &state.pool,
            operation.id,
            guard.claims.sub,
            page_size,
            offset,
            params.search.as_deref(),
        )
        .await?
    };

    let total_pages = if page_size > 0 {
        (total as f64 / page_size as f64).ceil() as i64
    } else {
        1
    };

    Ok(Json(ListQueryResponse {
        items,
        total,
        page,
        page_size,
        total_pages,
    }))
}

/// Execute a custom SQL data source with pagination, search, and sorting.
/// The SQL is validated as read-only SELECT and executed in a read-only transaction.
/// If the base SQL contains `$search`, the search term is bound as a named parameter.
async fn execute_list_sql(
    pool: &PgPool,
    base_sql: &str,
    limit: i64,
    offset: i64,
    search: Option<&str>,
    sort_by: Option<&str>,
    sort_order: Option<&str>,
) -> Result<(Vec<serde_json::Value>, i64), AppError> {
    // Validate the base SQL (must be a SELECT)
    sql_engine::validate_sql(base_sql)?;

    // If the SQL contains $search, replace it with a positional param and bind the value.
    let has_search_param = base_sql.contains("$search");
    let (resolved_sql, bind_values) = if has_search_param {
        let search_val = search.unwrap_or("").to_string();
        let resolved = base_sql.replace("$search", "$1");
        (resolved, vec![search_val])
    } else {
        (base_sql.to_string(), vec![])
    };

    // Build the count query
    let count_sql = format!("SELECT COUNT(*) AS _cnt FROM ({resolved_sql}) AS _sub");

    // Build the data query with optional ORDER BY and LIMIT/OFFSET
    let order_clause = if let Some(col) = sort_by {
        // Sanitise column name: allow only alphanumeric, underscore, dot
        if !col
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
        {
            return Err(AppError::Validation(
                "Invalid sort_by column name".to_string(),
            ));
        }
        let dir = match sort_order {
            Some(d) if d.eq_ignore_ascii_case("desc") => "DESC",
            _ => "ASC",
        };
        format!(" ORDER BY \"{col}\" {dir}")
    } else {
        String::new()
    };

    let data_sql = format!(
        "SELECT * FROM ({resolved_sql}) AS _sub{order_clause} LIMIT {limit} OFFSET {offset}"
    );

    // Execute both in a read-only transaction with timeout
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION READ ONLY")
        .execute(&mut *tx)
        .await?;
    sqlx::query("SET LOCAL statement_timeout = '10000ms'")
        .execute(&mut *tx)
        .await?;

    // Count
    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    for val in &bind_values {
        count_query = count_query.bind(val);
    }
    let count_row = count_query
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::Validation(format!("Count query error: {e}")))?;

    // Data
    let mut data_query = sqlx::query(&data_sql);
    for val in &bind_values {
        data_query = data_query.bind(val);
    }
    let rows: Vec<sqlx::postgres::PgRow> = data_query
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| AppError::Validation(format!("Data query error: {e}")))?;

    tx.rollback().await.ok();

    // Convert rows to JSON values
    let items = rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            for col in row.columns() {
                let name = col.name();
                let val: serde_json::Value = row
                    .try_get::<serde_json::Value, _>(name)
                    .or_else(|_| {
                        row.try_get::<String, _>(name)
                            .map(serde_json::Value::String)
                    })
                    .or_else(|_| {
                        row.try_get::<i64, _>(name)
                            .map(|v| serde_json::Value::Number(v.into()))
                    })
                    .or_else(|_| {
                        row.try_get::<f64, _>(name).map(|v| {
                            serde_json::Number::from_f64(v)
                                .map(serde_json::Value::Number)
                                .unwrap_or(serde_json::Value::Null)
                        })
                    })
                    .or_else(|_| row.try_get::<bool, _>(name).map(serde_json::Value::Bool))
                    .unwrap_or(serde_json::Value::Null);
                map.insert(name.to_string(), val);
            }
            serde_json::Value::Object(map)
        })
        .collect();

    Ok((items, count_row.0))
}

/// Fallback: query lc_operation_data with pagination and search.
/// Returns each row's `data` JSONB field merged with `id`, `created_at`, `updated_at`.
async fn execute_list_operation_data(
    pool: &PgPool,
    operation_id: Uuid,
    user_id: Uuid,
    limit: i64,
    offset: i64,
    search: Option<&str>,
) -> Result<(Vec<serde_json::Value>, i64), AppError> {
    // Fetch active record policies
    let policies: Vec<RecordPolicy> = sqlx::query_as(
        "SELECT rp.* FROM lc_record_policies rp \
         WHERE rp.operation_id = $1 AND rp.is_active = true \
         AND (rp.user_id = $2 OR rp.role_id IN (\
             SELECT upr.role_id FROM lc_user_platform_roles upr WHERE upr.user_id = $2\
         ))",
    )
    .bind(operation_id)
    .bind(user_id)
    .fetch_all(pool)
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

    let (total, rows) = if let Some(search) = search {
        let pattern = format!("%{search}%");
        let count_sql = format!(
            "SELECT COUNT(*) FROM lc_operation_data WHERE operation_id = $1 AND data::text ILIKE $2{policy_conditions}"
        );
        let (c,): (i64,) = sqlx::query_as(&count_sql)
            .bind(operation_id)
            .bind(&pattern)
            .fetch_one(pool)
            .await?;

        let data_sql = format!(
            "SELECT * FROM lc_operation_data WHERE operation_id = $1 AND data::text ILIKE $2{policy_conditions} ORDER BY created_at DESC LIMIT $3 OFFSET $4"
        );
        let r = sqlx::query_as::<_, OperationData>(&data_sql)
            .bind(operation_id)
            .bind(&pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;
        (c, r)
    } else {
        let count_sql = format!(
            "SELECT COUNT(*) FROM lc_operation_data WHERE operation_id = $1{policy_conditions}"
        );
        let (c,): (i64,) = sqlx::query_as(&count_sql)
            .bind(operation_id)
            .fetch_one(pool)
            .await?;

        let data_sql = format!(
            "SELECT * FROM lc_operation_data WHERE operation_id = $1{policy_conditions} ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        );
        let r = sqlx::query_as::<_, OperationData>(&data_sql)
            .bind(operation_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;
        (c, r)
    };

    // Convert OperationData rows to flattened JSON (merge `data` JSONB with metadata)
    let items: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            let mut obj = match row.data {
                serde_json::Value::Object(map) => map,
                other => {
                    let mut m = serde_json::Map::new();
                    m.insert("data".to_string(), other);
                    m
                }
            };
            obj.insert("id".to_string(), serde_json::json!(row.id));
            obj.insert("created_at".to_string(), serde_json::json!(row.created_at));
            obj.insert("updated_at".to_string(), serde_json::json!(row.updated_at));
            serde_json::Value::Object(obj)
        })
        .collect();

    Ok((items, total))
}
