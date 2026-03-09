use axum::routing::{delete, get, put};
use axum::Router;

use crate::notifications::handlers;
use crate::shared::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_notifications))
        .route("/unread-count", get(handlers::get_unread_count))
        .route("/read-all", put(handlers::mark_all_as_read))
        .route("/{id}/read", put(handlers::mark_as_read))
        .route("/{id}", delete(handlers::delete_notification))
}
