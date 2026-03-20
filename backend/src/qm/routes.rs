use axum::{
    routing::{get, post, put},
    Router,
};

use crate::qm::handlers;
use crate::qm::reports;
use crate::shared::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/inspection-lots",
            get(handlers::list_inspection_lots).post(handlers::create_inspection_lot),
        )
        .route("/inspection-lots/{id}", get(handlers::get_inspection_lot))
        .route(
            "/inspection-lots/{lot_id}/complete",
            put(handlers::complete_inspection),
        )
        .route(
            "/inspection-lots/{lot_id}/results",
            get(handlers::list_inspection_results),
        )
        .route(
            "/inspection-results",
            post(handlers::create_inspection_result),
        )
        .route(
            "/inspection-results/{result_id}",
            put(handlers::update_inspection_result).delete(handlers::delete_inspection_result),
        )
        .route(
            "/notifications",
            get(handlers::list_quality_notifications).post(handlers::create_quality_notification),
        )
        .route(
            "/notifications/{id}",
            get(handlers::get_quality_notification).put(handlers::update_quality_notification),
        )
        // Reports
        .route(
            "/reports/inspection-pass-rate",
            get(reports::inspection_pass_rate),
        )
        .route(
            "/reports/notification-summary",
            get(reports::notification_summary),
        )
}
