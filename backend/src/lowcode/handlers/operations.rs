use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformDeveloper, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, PaginatedResponse};

pub async fn list_operations(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Query(filter): Query<OperationFilter>,
) -> Result<Json<ApiResponse<PaginatedResponse<Operation>>>, AppError> {
    let per_page = filter.per_page.unwrap_or(20).clamp(1, 100);
    let page = filter.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    // Build WHERE conditions dynamically
    let mut conditions = Vec::new();
    let mut bind_idx = 1u32;

    if filter.project_id.is_some() {
        conditions.push(format!("project_id = ${bind_idx}"));
        bind_idx += 1;
    }
    if filter.module.is_some() {
        conditions.push(format!("module = ${bind_idx}"));
        bind_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    let count_sql = format!("SELECT COUNT(*) FROM lc_operations{where_clause}");
    let list_sql = format!(
        "SELECT * FROM lc_operations{where_clause} ORDER BY created_at DESC LIMIT ${bind_idx} OFFSET ${}",
        bind_idx + 1
    );

    // Build count query
    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    if let Some(ref pid) = filter.project_id {
        count_query = count_query.bind(pid);
    }
    if let Some(ref m) = filter.module {
        count_query = count_query.bind(m);
    }
    let (count,) = count_query.fetch_one(&state.pool).await?;

    // Build list query
    let mut list_query = sqlx::query_as::<_, Operation>(&list_sql);
    if let Some(ref pid) = filter.project_id {
        list_query = list_query.bind(pid);
    }
    if let Some(ref m) = filter.module {
        list_query = list_query.bind(m);
    }
    list_query = list_query.bind(per_page).bind(offset);
    let rows = list_query.fetch_all(&state.pool).await?;

    let params = crate::shared::PaginationParams {
        page: Some(page),
        per_page: Some(per_page),
    };
    Ok(Json(ApiResponse::success(PaginatedResponse::new(
        rows, count, &params,
    ))))
}

pub async fn create_operation(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformDeveloper>,
    Json(input): Json<CreateOperation>,
) -> Result<Json<ApiResponse<Operation>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let op_code = crate::fi::repositories::next_number(&state.pool, "LCO").await?;

    let operation = sqlx::query_as::<_, Operation>(
        "INSERT INTO lc_operations (operation_code, project_id, name, description, target_table, operation_type, created_by, module, sidebar_icon, sidebar_sort_order) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *",
    )
    .bind(&op_code)
    .bind(input.project_id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.target_table)
    .bind(input.operation_type.as_deref().unwrap_or("FORM"))
    .bind(guard.claims.sub)
    .bind(&input.module)
    .bind(&input.sidebar_icon)
    .bind(input.sidebar_sort_order.unwrap_or(100))
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        operation,
        "Operation created",
    )))
}

pub async fn get_operation(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Operation>>, AppError> {
    let operation = sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    Ok(Json(ApiResponse::success(operation)))
}

pub async fn update_operation(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateOperation>,
) -> Result<Json<ApiResponse<Operation>>, AppError> {
    let operation = sqlx::query_as::<_, Operation>(
        "UPDATE lc_operations SET name = COALESCE($2, name), description = COALESCE($3, description), target_table = COALESCE($4, target_table), operation_type = COALESCE($5, operation_type), module = COALESCE($6, module), sidebar_icon = COALESCE($7, sidebar_icon), sidebar_sort_order = COALESCE($8, sidebar_sort_order), updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.target_table)
    .bind(&input.operation_type)
    .bind(&input.module)
    .bind(&input.sidebar_icon)
    .bind(input.sidebar_sort_order)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    Ok(Json(ApiResponse::with_message(
        operation,
        "Operation updated",
    )))
}
