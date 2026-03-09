use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformAdmin, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, PaginatedResponse, PaginationParams};

pub async fn list_projects(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Project>>>, AppError> {
    let per_page = params.per_page();
    let offset = params.offset();

    let (count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM lc_projects WHERE is_active = true")
            .fetch_one(&state.pool)
            .await?;

    let rows = sqlx::query_as::<_, Project>(
        "SELECT * FROM lc_projects WHERE is_active = true ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(PaginatedResponse::new(
        rows, count, &params,
    ))))
}

pub async fn create_project(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformAdmin>,
    Json(input): Json<CreateProject>,
) -> Result<Json<ApiResponse<Project>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let project_number = crate::fi::repositories::next_number(&state.pool, "LCP").await?;

    let project = sqlx::query_as::<_, Project>(
        "INSERT INTO lc_projects (project_number, name, description, created_by) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(&project_number)
    .bind(&input.name)
    .bind(&input.description)
    .bind(guard.claims.sub)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(project, "Project created")))
}

pub async fn get_project(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Project>>, AppError> {
    let project = sqlx::query_as::<_, Project>("SELECT * FROM lc_projects WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    Ok(Json(ApiResponse::success(project)))
}

pub async fn update_project(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateProject>,
) -> Result<Json<ApiResponse<Project>>, AppError> {
    let existing = sqlx::query_as::<_, Project>("SELECT * FROM lc_projects WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    let project = sqlx::query_as::<_, Project>(
        "UPDATE lc_projects SET name = COALESCE($2, name), description = COALESCE($3, description), is_active = $4, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.is_active.unwrap_or(existing.is_active))
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(project, "Project updated")))
}

pub async fn delete_project(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Project>>, AppError> {
    let project = sqlx::query_as::<_, Project>(
        "UPDATE lc_projects SET is_active = false, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    Ok(Json(ApiResponse::with_message(
        project,
        "Project deactivated",
    )))
}
