use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::lowcode::services::permission_resolver::{PlatformAdmin, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, PaginatedResponse, PaginationParams};

// ── Response Types ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct UserWithRoles {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub is_active: bool,
    pub platform_roles: Vec<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProjectDeveloperInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

// ── Request Types ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AssignPlatformRole {
    pub role_name: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignProjectDeveloper {
    pub user_id: Uuid,
    pub role: Option<String>,
}

// ── Internal Row Types ──────────────────────────────────────────────────────

#[derive(Debug, sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    email: Option<String>,
    display_name: Option<String>,
    is_active: bool,
}

#[derive(Debug, sqlx::FromRow)]
struct UserRoleRow {
    user_id: Uuid,
    role_name: String,
}

// ── Handlers ────────────────────────────────────────────────────────────────

/// GET /lowcode/users - List users with their platform roles (paginated)
pub async fn list_users_with_roles(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<UserWithRoles>>>, AppError> {
    let per_page = params.per_page();
    let offset = params.offset();

    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await?;

    let users = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, email, display_name, is_active FROM users \
         ORDER BY username ASC LIMIT $1 OFFSET $2",
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let user_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();

    // Fetch all roles for these users in one query
    let role_rows: Vec<UserRoleRow> = if user_ids.is_empty() {
        vec![]
    } else {
        sqlx::query_as::<_, UserRoleRow>(
            "SELECT upr.user_id, pr.role_name \
             FROM lc_user_platform_roles upr \
             JOIN lc_platform_roles pr ON pr.id = upr.role_id \
             WHERE upr.user_id = ANY($1)",
        )
        .bind(&user_ids)
        .fetch_all(&state.pool)
        .await?
    };

    let result: Vec<UserWithRoles> = users
        .into_iter()
        .map(|u| {
            let roles: Vec<String> = role_rows
                .iter()
                .filter(|r| r.user_id == u.id)
                .map(|r| r.role_name.clone())
                .collect();
            UserWithRoles {
                id: u.id,
                username: u.username,
                email: u.email,
                display_name: u.display_name,
                is_active: u.is_active,
                platform_roles: roles,
            }
        })
        .collect();

    Ok(Json(ApiResponse::success(PaginatedResponse::new(
        result, count, &params,
    ))))
}

/// POST /lowcode/users/{user_id}/roles - Assign a platform role to a user
pub async fn assign_platform_role(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformAdmin>,
    Path(user_id): Path<Uuid>,
    Json(input): Json<AssignPlatformRole>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    // Validate role_name
    let valid_roles = ["PLATFORM_ADMIN", "DEVELOPER", "USER"];
    if !valid_roles.contains(&input.role_name.as_str()) {
        return Err(AppError::Validation(format!(
            "Invalid role_name '{}'. Must be one of: {:?}",
            input.role_name, valid_roles
        )));
    }

    // Find role_id
    let role: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM lc_platform_roles WHERE role_name = $1")
            .bind(&input.role_name)
            .fetch_optional(&state.pool)
            .await?;

    let (role_id,) = role.ok_or_else(|| {
        AppError::NotFound(format!("Platform role '{}' not found", input.role_name))
    })?;

    // Verify user exists
    let user_exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await?;

    if user_exists.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Insert (ON CONFLICT DO NOTHING for idempotency)
    sqlx::query(
        "INSERT INTO lc_user_platform_roles (user_id, role_id, assigned_by) \
         VALUES ($1, $2, $3) ON CONFLICT (user_id, role_id) DO NOTHING",
    )
    .bind(user_id)
    .bind(role_id)
    .bind(guard.claims.sub)
    .execute(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "user_id": user_id, "role_name": input.role_name }),
        "Platform role assigned",
    )))
}

/// DELETE /lowcode/users/{user_id}/roles/{role_name} - Revoke a platform role
pub async fn revoke_platform_role(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path((user_id, role_name)): Path<(Uuid, String)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let result = sqlx::query(
        "DELETE FROM lc_user_platform_roles \
         WHERE user_id = $1 AND role_id = (SELECT id FROM lc_platform_roles WHERE role_name = $2)",
    )
    .bind(user_id)
    .bind(&role_name)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "User does not have role '{}'",
            role_name
        )));
    }

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "deleted": true }),
        "Platform role revoked",
    )))
}

/// GET /lowcode/projects/{id}/developers - List developers assigned to a project
pub async fn list_project_developers(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<ProjectDeveloperInfo>>>, AppError> {
    let devs = sqlx::query_as::<_, ProjectDeveloperInfo>(
        "SELECT pd.id, pd.user_id, u.username, u.display_name, u.email, pd.role, pd.created_at \
         FROM lc_project_developers pd \
         JOIN users u ON u.id = pd.user_id \
         WHERE pd.project_id = $1 \
         ORDER BY pd.created_at",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(devs)))
}

/// POST /lowcode/projects/{id}/developers - Assign a developer to a project
pub async fn assign_project_developer(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path(id): Path<Uuid>,
    Json(input): Json<AssignProjectDeveloper>,
) -> Result<Json<ApiResponse<ProjectDeveloperInfo>>, AppError> {
    let role = input.role.as_deref().unwrap_or("DEVELOPER");

    // Validate role
    let valid_roles = ["LEAD", "DEVELOPER", "VIEWER"];
    if !valid_roles.contains(&role) {
        return Err(AppError::Validation(format!(
            "Invalid role '{}'. Must be one of: {:?}",
            role, valid_roles
        )));
    }

    // Verify user exists
    let user_exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(input.user_id)
        .fetch_optional(&state.pool)
        .await?;

    if user_exists.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Verify project exists
    let project_exists: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM lc_projects WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?;

    if project_exists.is_none() {
        return Err(AppError::NotFound("Project not found".to_string()));
    }

    // Insert with conflict handling
    sqlx::query(
        "INSERT INTO lc_project_developers (project_id, user_id, role) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (project_id, user_id) DO UPDATE SET role = $3",
    )
    .bind(id)
    .bind(input.user_id)
    .bind(role)
    .execute(&state.pool)
    .await?;

    // Fetch the result
    let dev = sqlx::query_as::<_, ProjectDeveloperInfo>(
        "SELECT pd.id, pd.user_id, u.username, u.display_name, u.email, pd.role, pd.created_at \
         FROM lc_project_developers pd \
         JOIN users u ON u.id = pd.user_id \
         WHERE pd.project_id = $1 AND pd.user_id = $2",
    )
    .bind(id)
    .bind(input.user_id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        dev,
        "Developer assigned to project",
    )))
}

/// DELETE /lowcode/projects/{id}/developers/{user_id} - Remove a developer from a project
pub async fn remove_project_developer(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformAdmin>,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let result =
        sqlx::query("DELETE FROM lc_project_developers WHERE project_id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&state.pool)
            .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Developer not found in this project".to_string(),
        ));
    }

    Ok(Json(ApiResponse::with_message(
        serde_json::json!({ "deleted": true }),
        "Developer removed from project",
    )))
}
