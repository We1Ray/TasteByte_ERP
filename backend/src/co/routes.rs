use axum::{
    routing::{get, put},
    Router,
};

use crate::co::handlers;
use crate::co::reports;
use crate::shared::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/cost-centers",
            get(handlers::list_cost_centers).post(handlers::create_cost_center),
        )
        .route("/cost-centers/{id}", get(handlers::get_cost_center))
        .route(
            "/profit-centers",
            get(handlers::list_profit_centers).post(handlers::create_profit_center),
        )
        .route("/profit-centers/{id}", get(handlers::get_profit_center))
        .route(
            "/internal-orders",
            get(handlers::list_internal_orders).post(handlers::create_internal_order),
        )
        .route(
            "/internal-orders/{id}",
            get(handlers::get_internal_order).put(handlers::update_internal_order),
        )
        .route(
            "/cost-allocations",
            get(handlers::list_cost_allocations).post(handlers::create_cost_allocation),
        )
        .route(
            "/cost-allocations/{id}",
            get(handlers::get_cost_allocation)
                .put(handlers::update_cost_allocation)
                .delete(handlers::delete_cost_allocation),
        )
        // Reports
        .route(
            "/reports/cost-center-summary",
            get(reports::cost_center_summary),
        )
        .route(
            "/reports/internal-order-budget",
            get(reports::internal_order_budget),
        )
}
