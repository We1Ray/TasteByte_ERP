use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 100, message = "Username must be 3-100 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

/// Full login response including user info with roles and permissions
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub is_active: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

// --- Role Management Models ---

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct RoleRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub parent_id: Option<Uuid>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct RoleWithPermissions {
    #[serde(flatten)]
    pub role: RoleRow,
    pub permissions: Vec<PermissionRow>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PermissionRow {
    pub id: Uuid,
    pub module: String,
    pub action: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<Option<Uuid>>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SetRolePermissionsRequest {
    pub permission_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct AssignUserRoleRequest {
    pub role_id: Uuid,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserWithRolesRow {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct UserWithRolesResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub is_active: bool,
    pub roles: Vec<RoleRow>,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
