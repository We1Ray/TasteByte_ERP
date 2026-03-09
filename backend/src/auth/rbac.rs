use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use std::marker::PhantomData;

use crate::shared::types::{AppState, Claims};
use crate::shared::AppError;

/// DB-driven permission extractor. Checks claims.permissions for "module:action" strings.
/// ADMIN role automatically passes all checks.
pub struct RequirePermission<P: PermissionRequirement> {
    pub claims: Claims,
    _marker: PhantomData<P>,
}

pub trait PermissionRequirement: Send + Sync {
    fn required_permission() -> &'static str;
}

// Backward-compatible type alias: all handlers still use RequireRole<FiRead> etc.
pub type RequireRole<P> = RequirePermission<P>;

macro_rules! define_permission {
    ($name:ident, $perm:expr) => {
        pub struct $name;
        impl PermissionRequirement for $name {
            fn required_permission() -> &'static str {
                $perm
            }
        }
    };
}

// FI module
define_permission!(FiRead, "fi:read");
define_permission!(FiWrite, "fi:write");

// CO module
define_permission!(CoRead, "co:read");
define_permission!(CoWrite, "co:write");

// MM module
define_permission!(MmRead, "mm:read");
define_permission!(MmWrite, "mm:write");

// SD module
define_permission!(SdRead, "sd:read");
define_permission!(SdWrite, "sd:write");

// PP module
define_permission!(PpRead, "pp:read");
define_permission!(PpWrite, "pp:write");

// HR module
define_permission!(HrRead, "hr:read");
define_permission!(HrWrite, "hr:write");

// WM module
define_permission!(WmRead, "wm:read");
define_permission!(WmWrite, "wm:write");

// QM module
define_permission!(QmRead, "qm:read");
define_permission!(QmWrite, "qm:write");

impl<P: PermissionRequirement> FromRequestParts<AppState> for RequirePermission<P> {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;

        // ADMIN bypasses all checks
        if claims.is_admin() {
            return Ok(RequirePermission {
                claims,
                _marker: PhantomData,
            });
        }

        // Check if user has the required permission
        let required = P::required_permission();
        if claims.permissions.contains(&required.to_string()) {
            Ok(RequirePermission {
                claims,
                _marker: PhantomData,
            })
        } else {
            Err(AppError::Forbidden(format!(
                "Insufficient permissions. Required: {}",
                required
            )))
        }
    }
}
