use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::auth::models::{
    AssignUserRoleRequest, CreateRoleRequest, MeResponse, PermissionRow, RoleRow,
    RoleWithPermissions, SetRolePermissionsRequest, UpdateRoleRequest, UserWithRolesResponse,
    UserWithRolesRow,
};
use serde::Deserialize;

use crate::shared::audit;
use crate::shared::types::{AppState, Claims};
use crate::shared::{ApiResponse, AppError, PaginatedResponse};

#[derive(Debug, Deserialize)]
pub struct UserSearchParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

// Helper: check that the caller has ADMIN role
fn require_admin(claims: &Claims) -> Result<(), AppError> {
    if claims.is_admin() {
        Ok(())
    } else {
        Err(AppError::Forbidden(
            "Only ADMIN can manage roles".to_string(),
        ))
    }
}

/// GET /auth/roles - List all roles with hierarchy
pub async fn list_roles(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<ApiResponse<Vec<RoleRow>>>, AppError> {
    require_admin(&claims)?;

    let roles: Vec<RoleRow> = sqlx::query_as(
        "SELECT id, name, description, is_system, parent_id, sort_order, created_at \
         FROM roles ORDER BY sort_order, name",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(roles)))
}

/// GET /auth/roles/:id - Get role with its permissions
pub async fn get_role(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<RoleWithPermissions>>, AppError> {
    require_admin(&claims)?;

    let role: RoleRow = sqlx::query_as(
        "SELECT id, name, description, is_system, parent_id, sort_order, created_at \
         FROM roles WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    let permissions: Vec<PermissionRow> = sqlx::query_as(
        "SELECT p.id, p.module, p.action, p.description \
         FROM permissions p \
         JOIN role_permissions rp ON rp.permission_id = p.id \
         WHERE rp.role_id = $1 \
         ORDER BY p.module, p.action",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(RoleWithPermissions {
        role,
        permissions,
    })))
}

/// POST /auth/roles - Create a new role
pub async fn create_role(
    State(state): State<AppState>,
    claims: Claims,
    Json(input): Json<CreateRoleRequest>,
) -> Result<Json<ApiResponse<RoleRow>>, AppError> {
    require_admin(&claims)?;
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Validate parent exists if specified
    if let Some(parent_id) = input.parent_id {
        let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM roles WHERE id = $1")
            .bind(parent_id)
            .fetch_optional(&state.pool)
            .await?;
        if exists.is_none() {
            return Err(AppError::Validation("Parent role not found".to_string()));
        }
    }

    let role: RoleRow = sqlx::query_as(
        "INSERT INTO roles (name, description, parent_id, is_system) \
         VALUES ($1, $2, $3, false) \
         RETURNING id, name, description, is_system, parent_id, sort_order, created_at",
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.parent_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.constraint().is_some() => {
            AppError::Validation("Role name already exists".to_string())
        }
        _ => AppError::Database(e),
    })?;

    let _ = audit::log_change(
        &state.pool,
        "roles",
        role.id,
        "CREATE",
        None,
        Some(serde_json::json!({"name": &role.name, "parent_id": role.parent_id})),
        Some(claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(role, "Role created")))
}

/// PUT /auth/roles/:id - Update a role
pub async fn update_role(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateRoleRequest>,
) -> Result<Json<ApiResponse<RoleRow>>, AppError> {
    require_admin(&claims)?;

    // Verify role exists
    let existing: RoleRow = sqlx::query_as(
        "SELECT id, name, description, is_system, parent_id, sort_order, created_at \
         FROM roles WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    // Check circular reference if parent_id is being changed
    if let Some(Some(new_parent_id)) = input.parent_id {
        if new_parent_id == id {
            return Err(AppError::Validation(
                "A role cannot be its own parent".to_string(),
            ));
        }
        // Walk ancestors to detect cycle
        let cycle_check: Option<(Uuid,)> = sqlx::query_as(
            "WITH RECURSIVE ancestors AS ( \
                 SELECT id, parent_id FROM roles WHERE id = $1 \
                 UNION ALL \
                 SELECT r.id, r.parent_id FROM roles r \
                 JOIN ancestors a ON r.id = a.parent_id \
             ) \
             SELECT id FROM ancestors WHERE id = $2",
        )
        .bind(new_parent_id)
        .bind(id)
        .fetch_optional(&state.pool)
        .await?;
        if cycle_check.is_some() {
            return Err(AppError::Validation(
                "Cannot set parent: would create circular reference".to_string(),
            ));
        }
    }

    let old_name = existing.name.clone();
    let name = input.name.unwrap_or(existing.name);
    let description = input.description.or(existing.description);
    let parent_id = match input.parent_id {
        Some(p) => p,               // Some(Some(id)) or Some(None)
        None => existing.parent_id, // unchanged
    };
    let sort_order = input.sort_order.unwrap_or(existing.sort_order);

    let role: RoleRow = sqlx::query_as(
        "UPDATE roles SET name = $1, description = $2, parent_id = $3, sort_order = $4 \
         WHERE id = $5 \
         RETURNING id, name, description, is_system, parent_id, sort_order, created_at",
    )
    .bind(&name)
    .bind(&description)
    .bind(parent_id)
    .bind(sort_order)
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.constraint().is_some() => {
            AppError::Validation("Role name already exists".to_string())
        }
        _ => AppError::Database(e),
    })?;

    let _ = audit::log_change(
        &state.pool,
        "roles",
        role.id,
        "UPDATE",
        Some(serde_json::json!({"name": &old_name})),
        Some(serde_json::json!({"name": &role.name, "parent_id": role.parent_id})),
        Some(claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(role, "Role updated")))
}

/// DELETE /auth/roles/:id - Delete a non-system role
pub async fn delete_role(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    require_admin(&claims)?;

    let role: RoleRow = sqlx::query_as(
        "SELECT id, name, description, is_system, parent_id, sort_order, created_at \
         FROM roles WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?;

    if role.is_system {
        return Err(AppError::Validation(
            "Cannot delete system roles".to_string(),
        ));
    }

    // Use transaction to ensure atomicity of re-parent + delete
    let mut tx = state.pool.begin().await?;

    // Re-parent children to the deleted role's parent
    sqlx::query("UPDATE roles SET parent_id = $1 WHERE parent_id = $2")
        .bind(role.parent_id)
        .bind(id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM roles WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    let _ = audit::log_change(
        &state.pool,
        "roles",
        id,
        "DELETE",
        Some(serde_json::json!({"name": &role.name})),
        None,
        Some(claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message((), "Role deleted")))
}

/// GET /auth/roles/:id/permissions - Get permissions for a role
pub async fn get_role_permissions(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<PermissionRow>>>, AppError> {
    require_admin(&claims)?;

    let permissions: Vec<PermissionRow> = sqlx::query_as(
        "SELECT p.id, p.module, p.action, p.description \
         FROM permissions p \
         JOIN role_permissions rp ON rp.permission_id = p.id \
         WHERE rp.role_id = $1 \
         ORDER BY p.module, p.action",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(permissions)))
}

/// PUT /auth/roles/:id/permissions - Replace all permissions for a role
pub async fn set_role_permissions(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<SetRolePermissionsRequest>,
) -> Result<Json<ApiResponse<Vec<PermissionRow>>>, AppError> {
    require_admin(&claims)?;

    // Verify role exists
    let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM roles WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?;
    if exists.is_none() {
        return Err(AppError::NotFound("Role not found".to_string()));
    }

    // Transaction: delete existing, insert new
    let mut tx = state.pool.begin().await?;

    sqlx::query("DELETE FROM role_permissions WHERE role_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    for perm_id in &input.permission_ids {
        sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)")
            .bind(id)
            .bind(perm_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(ref db_err)
                    if db_err.constraint().is_some()
                        && db_err.message().contains("permissions") =>
                {
                    AppError::Validation(format!("Permission {} not found", perm_id))
                }
                _ => AppError::Database(e),
            })?;
    }

    tx.commit().await?;

    // Return updated permissions
    let permissions: Vec<PermissionRow> = sqlx::query_as(
        "SELECT p.id, p.module, p.action, p.description \
         FROM permissions p \
         JOIN role_permissions rp ON rp.permission_id = p.id \
         WHERE rp.role_id = $1 \
         ORDER BY p.module, p.action",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    let _ = audit::log_change(
        &state.pool,
        "role_permissions",
        id,
        "UPDATE",
        None,
        Some(serde_json::json!({"permission_ids": &input.permission_ids})),
        Some(claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        permissions,
        "Role permissions updated",
    )))
}

/// GET /auth/permissions - List all available permissions
pub async fn list_permissions(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<ApiResponse<Vec<PermissionRow>>>, AppError> {
    require_admin(&claims)?;

    let permissions: Vec<PermissionRow> = sqlx::query_as(
        "SELECT id, module, action, description FROM permissions ORDER BY module, action",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(permissions)))
}

/// GET /auth/users - List users with their assigned roles
pub async fn list_users_with_roles(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<UserSearchParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<UserWithRolesResponse>>>, AppError> {
    require_admin(&claims)?;

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let (users, total): (Vec<UserWithRolesRow>, i64) = if let Some(ref search) = params.search {
        let pattern = format!("%{}%", search);
        let users: Vec<UserWithRolesRow> = sqlx::query_as(
            "SELECT id, username, email, display_name, is_active FROM users \
             WHERE username ILIKE $1 OR email ILIKE $1 OR display_name ILIKE $1 \
             ORDER BY username LIMIT $2 OFFSET $3",
        )
        .bind(&pattern)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.pool)
        .await?;

        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users \
             WHERE username ILIKE $1 OR email ILIKE $1 OR display_name ILIKE $1",
        )
        .bind(&pattern)
        .fetch_one(&state.pool)
        .await?;

        (users, count.0)
    } else {
        let users: Vec<UserWithRolesRow> = sqlx::query_as(
            "SELECT id, username, email, display_name, is_active FROM users \
             ORDER BY username LIMIT $1 OFFSET $2",
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.pool)
        .await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&state.pool)
            .await?;

        (users, count.0)
    };

    // Fetch roles for all users in the result set
    let user_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();

    #[derive(sqlx::FromRow)]
    struct UserRoleJoin {
        user_id: Uuid,
        role_id: Uuid,
        name: String,
        description: Option<String>,
        is_system: bool,
        parent_id: Option<Uuid>,
        sort_order: i32,
        created_at: chrono::DateTime<chrono::Utc>,
    }

    let role_rows: Vec<UserRoleJoin> = sqlx::query_as(
        "SELECT ur.user_id, r.id AS role_id, r.name, r.description, r.is_system, r.parent_id, r.sort_order, r.created_at \
         FROM user_roles ur \
         JOIN roles r ON r.id = ur.role_id \
         WHERE ur.user_id = ANY($1) \
         ORDER BY r.sort_order, r.name",
    )
    .bind(&user_ids)
    .fetch_all(&state.pool)
    .await?;

    // Group roles by user_id
    use std::collections::HashMap;
    let mut roles_map: HashMap<Uuid, Vec<RoleRow>> = HashMap::new();
    for row in role_rows {
        roles_map.entry(row.user_id).or_default().push(RoleRow {
            id: row.role_id,
            name: row.name,
            description: row.description,
            is_system: row.is_system,
            parent_id: row.parent_id,
            sort_order: row.sort_order,
            created_at: row.created_at,
        });
    }

    let data: Vec<UserWithRolesResponse> = users
        .into_iter()
        .map(|u| {
            let roles = roles_map.remove(&u.id).unwrap_or_default();
            UserWithRolesResponse {
                id: u.id,
                username: u.username,
                email: u.email,
                display_name: u.display_name,
                is_active: u.is_active,
                roles,
            }
        })
        .collect();

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i64;

    Ok(Json(ApiResponse::success(PaginatedResponse {
        items: data,
        total,
        page,
        page_size: per_page,
        total_pages,
    })))
}

/// POST /auth/users/:user_id/roles - Assign role to user
pub async fn assign_user_role(
    State(state): State<AppState>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
    Json(input): Json<AssignUserRoleRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    require_admin(&claims)?;

    // Verify user exists
    let user_exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await?;
    if user_exists.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Verify role exists
    let role_exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM roles WHERE id = $1")
        .bind(input.role_id)
        .fetch_optional(&state.pool)
        .await?;
    if role_exists.is_none() {
        return Err(AppError::NotFound("Role not found".to_string()));
    }

    sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(user_id)
        .bind(input.role_id)
        .execute(&state.pool)
        .await?;

    let _ = audit::log_change(
        &state.pool,
        "user_roles",
        user_id,
        "ASSIGN_ROLE",
        None,
        Some(serde_json::json!({"role_id": input.role_id})),
        Some(claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message((), "Role assigned")))
}

/// DELETE /auth/users/:user_id/roles/:role_id - Remove role from user
pub async fn remove_user_role(
    State(state): State<AppState>,
    claims: Claims,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    require_admin(&claims)?;

    let result = sqlx::query("DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2")
        .bind(user_id)
        .bind(role_id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "User-role assignment not found".to_string(),
        ));
    }

    let _ = audit::log_change(
        &state.pool,
        "user_roles",
        user_id,
        "REMOVE_ROLE",
        Some(serde_json::json!({"role_id": role_id})),
        None,
        Some(claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message((), "Role removed")))
}

/// GET /auth/me - Current user's profile with roles and permissions
pub async fn get_me(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<ApiResponse<MeResponse>>, AppError> {
    #[derive(sqlx::FromRow)]
    struct UserInfo {
        id: Uuid,
        username: String,
        email: String,
        display_name: Option<String>,
    }

    let user: UserInfo =
        sqlx::query_as("SELECT id, username, email, display_name FROM users WHERE id = $1")
            .bind(claims.sub)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Query fresh roles and permissions from DB instead of using JWT claims
    let (roles, permissions) =
        crate::auth::handlers::resolve_roles_and_permissions(&state.pool, claims.sub).await?;

    Ok(Json(ApiResponse::success(MeResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        display_name: user.display_name,
        roles,
        permissions,
    })))
}
