use axum::extract::State;
use axum::Json;
use chrono::Utc;
use validator::Validate;

use crate::auth::models::{
    LoginRequest, LoginResponse, RefreshRequest, RegisterRequest, TokenResponse, UserResponse,
};
use crate::auth::services;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

/// Resolve roles and permissions (with hierarchy inheritance) for a user in a single query.
pub async fn resolve_roles_and_permissions(
    pool: &sqlx::PgPool,
    user_id: uuid::Uuid,
) -> Result<(Vec<String>, Vec<String>), AppError> {
    // Single query: fetch role names
    let role_rows: Vec<(String,)> = sqlx::query_as(
        "SELECT r.name FROM roles r JOIN user_roles ur ON ur.role_id = r.id WHERE ur.user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    let roles: Vec<String> = role_rows.into_iter().map(|r| r.0).collect();

    // Single query: resolve permissions with hierarchy inheritance via recursive CTE.
    // Walks from user's direct roles UP through parent_id chain, collecting all
    // permissions from ancestor roles.
    let perm_rows: Vec<(String, String)> = sqlx::query_as(
        "WITH RECURSIVE role_tree AS ( \
             SELECT r.id FROM roles r \
             JOIN user_roles ur ON ur.role_id = r.id \
             WHERE ur.user_id = $1 \
             UNION \
             SELECT r.parent_id FROM roles r \
             JOIN role_tree rt ON rt.id = r.id \
             WHERE r.parent_id IS NOT NULL \
         ) \
         SELECT DISTINCT p.module, p.action \
         FROM role_tree rt \
         JOIN role_permissions rp ON rp.role_id = rt.id \
         JOIN permissions p ON p.id = rp.permission_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    let permissions: Vec<String> = perm_rows
        .into_iter()
        .map(|(module, action)| format!("{}:{}", module, action))
        .collect();

    Ok((roles, permissions))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user: UserRow = sqlx::query_as(
        "SELECT id, username, email, password_hash, display_name, is_active, failed_login_attempts, locked_until FROM users WHERE username = $1",
    )
    .bind(&payload.username)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    if !user.is_active {
        return Err(AppError::Unauthorized("Account is disabled".to_string()));
    }

    // Check account lockout
    if let Some(locked_until) = user.locked_until {
        if locked_until > Utc::now() {
            return Err(AppError::Unauthorized(
                "Account is temporarily locked. Please try again later.".to_string(),
            ));
        }
    }

    let valid = services::verify_password(&payload.password, &user.password_hash)?;
    if !valid {
        // Increment failed attempts
        let new_attempts = user.failed_login_attempts + 1;
        if new_attempts >= 5 {
            // Lock account for 15 minutes
            sqlx::query(
                "UPDATE users SET failed_login_attempts = $2, locked_until = NOW() + INTERVAL '15 minutes' WHERE id = $1",
            )
            .bind(user.id)
            .bind(new_attempts)
            .execute(&state.pool)
            .await?;
        } else {
            sqlx::query("UPDATE users SET failed_login_attempts = $2 WHERE id = $1")
                .bind(user.id)
                .bind(new_attempts)
                .execute(&state.pool)
                .await?;
        }
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Reset failed attempts on successful login
    sqlx::query("UPDATE users SET failed_login_attempts = 0, locked_until = NULL WHERE id = $1")
        .bind(user.id)
        .execute(&state.pool)
        .await?;

    let (roles, permissions) = resolve_roles_and_permissions(&state.pool, user.id).await?;

    let access_token = services::generate_token(
        user.id,
        &user.username,
        roles.clone(),
        permissions.clone(),
        &state.settings.jwt_secret,
        state.settings.access_token_expiry_minutes,
    )?;

    // Generate refresh token
    let refresh_token = services::generate_refresh_token();
    let refresh_hash = services::hash_refresh_token(&refresh_token);
    let refresh_expires =
        Utc::now() + chrono::Duration::days(state.settings.refresh_token_expiry_days);

    sqlx::query("INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)")
        .bind(user.id)
        .bind(&refresh_hash)
        .bind(refresh_expires)
        .execute(&state.pool)
        .await?;

    Ok(Json(ApiResponse::success(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.settings.access_token_expiry_minutes * 60,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            is_active: user.is_active,
            roles,
            permissions,
        },
    })))
}

pub async fn register(
    State(state): State<AppState>,
    claims: crate::shared::types::Claims,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    // Only ADMIN users can register new accounts
    if !claims.is_admin() {
        return Err(AppError::Forbidden(
            "Only administrators can register new users".to_string(),
        ));
    }

    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Validate password strength
    services::validate_password_strength(&payload.password)?;

    let password_hash = services::hash_password(&payload.password)?;
    let display_name = payload
        .display_name
        .unwrap_or_else(|| payload.username.clone());

    let user: UserRowFull = sqlx::query_as(
        "INSERT INTO users (username, email, password_hash, display_name) VALUES ($1, $2, $3, $4) RETURNING id, username, email, display_name, is_active",
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&display_name)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.constraint().is_some() => {
            AppError::Validation("Username or email already exists".to_string())
        }
        _ => AppError::Database(e),
    })?;

    Ok(Json(ApiResponse::with_message(
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            is_active: user.is_active,
            roles: vec![],
            permissions: vec![],
        },
        "User registered successfully",
    )))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<ApiResponse<TokenResponse>>, AppError> {
    let token_hash = services::hash_refresh_token(&payload.refresh_token);

    // Find valid refresh token
    let rt: RefreshTokenRow = sqlx::query_as(
        "SELECT id, user_id, expires_at FROM refresh_tokens WHERE token_hash = $1 AND revoked_at IS NULL",
    )
    .bind(&token_hash)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;

    if rt.expires_at < Utc::now() {
        return Err(AppError::Unauthorized("Refresh token expired".to_string()));
    }

    // Check user is still active
    let user_active: Option<(bool,)> = sqlx::query_as("SELECT is_active FROM users WHERE id = $1")
        .bind(rt.user_id)
        .fetch_optional(&state.pool)
        .await?;
    match user_active {
        Some((true,)) => {}
        _ => return Err(AppError::Unauthorized("Account is disabled".to_string())),
    }

    // Revoke old refresh token
    sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE id = $1")
        .bind(rt.id)
        .execute(&state.pool)
        .await?;

    // Get user info for new access token
    let user_info: UserRowBasic = sqlx::query_as("SELECT username FROM users WHERE id = $1")
        .bind(rt.user_id)
        .fetch_one(&state.pool)
        .await?;

    let (roles, permissions) = resolve_roles_and_permissions(&state.pool, rt.user_id).await?;

    let access_token = services::generate_token(
        rt.user_id,
        &user_info.username,
        roles,
        permissions,
        &state.settings.jwt_secret,
        state.settings.access_token_expiry_minutes,
    )?;

    // Generate new refresh token (rotation)
    let new_refresh_token = services::generate_refresh_token();
    let new_refresh_hash = services::hash_refresh_token(&new_refresh_token);
    let refresh_expires =
        Utc::now() + chrono::Duration::days(state.settings.refresh_token_expiry_days);

    sqlx::query("INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)")
        .bind(rt.user_id)
        .bind(&new_refresh_hash)
        .bind(refresh_expires)
        .execute(&state.pool)
        .await?;

    Ok(Json(ApiResponse::success(TokenResponse {
        access_token,
        refresh_token: new_refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.settings.access_token_expiry_minutes * 60,
    })))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let token_hash = services::hash_refresh_token(&payload.refresh_token);

    sqlx::query(
        "UPDATE refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1 AND revoked_at IS NULL",
    )
    .bind(&token_hash)
    .execute(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        (),
        "Logged out successfully",
    )))
}

// Internal query types
#[derive(sqlx::FromRow)]
struct UserRow {
    id: uuid::Uuid,
    username: String,
    email: String,
    password_hash: String,
    display_name: Option<String>,
    is_active: bool,
    failed_login_attempts: i32,
    locked_until: Option<chrono::DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct UserRowFull {
    id: uuid::Uuid,
    username: String,
    email: String,
    display_name: Option<String>,
    is_active: bool,
}

#[derive(sqlx::FromRow)]
struct RefreshTokenRow {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    expires_at: chrono::DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct UserRowBasic {
    username: String,
}
