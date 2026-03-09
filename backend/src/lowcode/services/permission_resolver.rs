use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use sqlx::PgPool;
use std::marker::PhantomData;
use uuid::Uuid;

use crate::lowcode::models::{FieldPermission, OperationPermission};
use crate::shared::types::{AppState, Claims};
use crate::shared::AppError;

// ── Platform Role Guard ───────────────────────────────────────────────────

/// Axum extractor that enforces platform role requirements.
/// Checks `lc_user_platform_roles` in DB. ADMIN in traditional RBAC bypasses all checks.
pub struct RequirePlatformRole<R: PlatformRoleRequirement> {
    pub claims: Claims,
    _marker: PhantomData<R>,
}

pub trait PlatformRoleRequirement: Send + Sync {
    fn required_roles() -> &'static [&'static str];
}

macro_rules! define_platform_role {
    ($name:ident, $($role:expr),+) => {
        pub struct $name;
        impl PlatformRoleRequirement for $name {
            fn required_roles() -> &'static [&'static str] {
                &[$($role),+]
            }
        }
    };
}

// PLATFORM_ADMIN only
define_platform_role!(PlatformAdmin, "PLATFORM_ADMIN");
// DEVELOPER or PLATFORM_ADMIN
define_platform_role!(PlatformDeveloper, "PLATFORM_ADMIN", "DEVELOPER");
// Any platform role (USER, DEVELOPER, or PLATFORM_ADMIN)
define_platform_role!(PlatformUser, "PLATFORM_ADMIN", "DEVELOPER", "USER");

impl<R: PlatformRoleRequirement> FromRequestParts<AppState> for RequirePlatformRole<R> {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;

        // ADMIN in traditional RBAC bypasses all platform checks
        if claims.is_admin() {
            return Ok(RequirePlatformRole {
                claims,
                _marker: PhantomData,
            });
        }

        // Check platform roles from DB
        let user_roles: Vec<(String,)> = sqlx::query_as(
            "SELECT pr.role_name FROM lc_user_platform_roles upr \
             JOIN lc_platform_roles pr ON pr.id = upr.role_id \
             WHERE upr.user_id = $1",
        )
        .bind(claims.sub)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to check platform roles: {e}")))?;

        let required = R::required_roles();
        let has_role = user_roles.iter().any(|(r,)| required.contains(&r.as_str()));

        if has_role {
            Ok(RequirePlatformRole {
                claims,
                _marker: PhantomData,
            })
        } else {
            Err(AppError::Forbidden(format!(
                "Insufficient platform permissions. Required one of: {:?}",
                required
            )))
        }
    }
}

// ── Resolved Permissions ──────────────────────────────────────────────────

/// Resolved permissions for a user on a specific operation
#[derive(Debug)]
pub struct ResolvedPermission {
    pub can_create: bool,
    pub can_read: bool,
    pub can_update: bool,
    pub can_delete: bool,
}

/// Resolve effective operation permissions for a user.
/// Priority: user-specific > role-specific. Most permissive wins among roles.
pub async fn resolve_operation_permission(
    pool: &PgPool,
    operation_id: Uuid,
    user_id: Uuid,
) -> Result<ResolvedPermission, AppError> {
    // Check user-specific permissions first
    let user_perm = sqlx::query_as::<_, OperationPermission>(
        "SELECT * FROM lc_operation_permissions WHERE operation_id = $1 AND user_id = $2",
    )
    .bind(operation_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    if let Some(p) = user_perm {
        return Ok(ResolvedPermission {
            can_create: p.can_create,
            can_read: p.can_read,
            can_update: p.can_update,
            can_delete: p.can_delete,
        });
    }

    // Fall back to role-based permissions
    let role_perms: Vec<OperationPermission> = sqlx::query_as(
        "SELECT op.* FROM lc_operation_permissions op \
         JOIN lc_user_platform_roles upr ON upr.role_id = op.role_id \
         WHERE op.operation_id = $1 AND upr.user_id = $2",
    )
    .bind(operation_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    if role_perms.is_empty() {
        // No permissions found: default read-only
        return Ok(ResolvedPermission {
            can_create: false,
            can_read: true,
            can_update: false,
            can_delete: false,
        });
    }

    // Most permissive wins among roles
    Ok(ResolvedPermission {
        can_create: role_perms.iter().any(|p| p.can_create),
        can_read: role_perms.iter().any(|p| p.can_read),
        can_update: role_perms.iter().any(|p| p.can_update),
        can_delete: role_perms.iter().any(|p| p.can_delete),
    })
}

/// Resolve field-level permissions for a user on a specific field.
pub async fn resolve_field_permission(
    pool: &PgPool,
    field_id: Uuid,
    user_id: Uuid,
) -> Result<Option<FieldPermission>, AppError> {
    // User-specific first
    let user_perm = sqlx::query_as::<_, FieldPermission>(
        "SELECT * FROM lc_field_permissions WHERE field_id = $1 AND user_id = $2",
    )
    .bind(field_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    if user_perm.is_some() {
        return Ok(user_perm);
    }

    // Role-based
    let role_perm = sqlx::query_as::<_, FieldPermission>(
        "SELECT fp.* FROM lc_field_permissions fp \
         JOIN lc_user_platform_roles upr ON upr.role_id = fp.role_id \
         WHERE fp.field_id = $1 AND upr.user_id = $2 \
         LIMIT 1",
    )
    .bind(field_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(role_perm)
}
