use axum::extract::State;
use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::shared::types::{AppState, Claims};
use crate::shared::{ApiResponse, AppError};

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub platform_roles: Vec<String>,
    pub projects: Vec<UserProjectRole>,
}

#[derive(Debug, Serialize)]
pub struct UserProjectRole {
    pub id: Uuid,
    pub name: String,
    pub role: String,
}

/// GET /lowcode/user/me - Get current user's platform roles and project assignments
pub async fn get_my_profile(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<ApiResponse<UserProfile>>, AppError> {
    // Query platform roles
    let roles: Vec<(String,)> = sqlx::query_as(
        "SELECT pr.role_name FROM lc_user_platform_roles upr \
         JOIN lc_platform_roles pr ON pr.id = upr.role_id \
         WHERE upr.user_id = $1",
    )
    .bind(claims.sub)
    .fetch_all(&state.pool)
    .await?;

    // Query project assignments
    let projects: Vec<(Uuid, String, String)> = sqlx::query_as(
        "SELECT p.id, p.name, pd.role FROM lc_project_developers pd \
         JOIN lc_projects p ON p.id = pd.project_id \
         WHERE pd.user_id = $1",
    )
    .bind(claims.sub)
    .fetch_all(&state.pool)
    .await?;

    // Also check if user has traditional ADMIN role - if so, add PLATFORM_ADMIN
    let mut platform_roles: Vec<String> = roles.into_iter().map(|(r,)| r).collect();
    if claims.is_admin() && !platform_roles.contains(&"PLATFORM_ADMIN".to_string()) {
        platform_roles.push("PLATFORM_ADMIN".to_string());
    }

    let profile = UserProfile {
        platform_roles,
        projects: projects
            .into_iter()
            .map(|(id, name, role)| UserProjectRole { id, name, role })
            .collect(),
    };

    Ok(Json(ApiResponse::success(profile)))
}
