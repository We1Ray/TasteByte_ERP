use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

use crate::auth::{handlers, role_management};
use crate::shared::types::AppState;

pub fn routes() -> Router<AppState> {
    // Rate limit: login - 10 requests per minute per IP
    let login_governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(6_000) // 60_000ms / 10 = 6_000ms per token
            .burst_size(10)
            .finish()
            .expect("Failed to build login rate limiter config"),
    );

    // Rate limit: register - 5 requests per minute per IP
    let register_governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(12_000) // 60_000ms / 5 = 12_000ms per token
            .burst_size(5)
            .finish()
            .expect("Failed to build register rate limiter config"),
    );

    let login_route = Router::new()
        .route("/login", post(handlers::login))
        .layer(GovernorLayer::new(login_governor_conf));

    let register_route = Router::new()
        .route("/register", post(handlers::register))
        .layer(GovernorLayer::new(register_governor_conf));

    login_route
        .merge(register_route)
        .route("/refresh", post(handlers::refresh))
        .route("/logout", post(handlers::logout))
        .route("/me", get(role_management::get_me))
        // Role management (ADMIN only, enforced in handlers)
        .route(
            "/roles",
            get(role_management::list_roles).post(role_management::create_role),
        )
        .route(
            "/roles/{id}",
            get(role_management::get_role)
                .put(role_management::update_role)
                .delete(role_management::delete_role),
        )
        .route(
            "/roles/{id}/permissions",
            get(role_management::get_role_permissions).put(role_management::set_role_permissions),
        )
        .route("/permissions", get(role_management::list_permissions))
        .route("/users", get(role_management::list_users_with_roles))
        .route(
            "/users/{user_id}/roles",
            post(role_management::assign_user_role),
        )
        .route(
            "/users/{user_id}/roles/{role_id}",
            delete(role_management::remove_user_role),
        )
}
