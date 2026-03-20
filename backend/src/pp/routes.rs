use axum::{
    routing::{get, post, put},
    Router,
};

use crate::pp::handlers;
use crate::pp::reports;
use crate::shared::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/boms", get(handlers::list_boms).post(handlers::create_bom))
        .route("/boms/{id}", get(handlers::get_bom))
        .route(
            "/boms/{bom_id}/items",
            post(handlers::add_bom_item),
        )
        .route(
            "/boms/{bom_id}/items/{item_id}",
            put(handlers::update_bom_item).delete(handlers::delete_bom_item),
        )
        .route(
            "/routings",
            get(handlers::list_routings).post(handlers::create_routing),
        )
        .route("/routings/{id}", get(handlers::get_routing))
        .route(
            "/routings/{routing_id}/operations",
            post(handlers::add_routing_operation),
        )
        .route(
            "/routings/{routing_id}/operations/{op_id}",
            put(handlers::update_routing_operation).delete(handlers::delete_routing_operation),
        )
        .route(
            "/production-orders",
            get(handlers::list_production_orders).post(handlers::create_production_order),
        )
        .route(
            "/production-orders/{id}",
            get(handlers::get_production_order),
        )
        .route(
            "/production-orders/{id}/status",
            put(handlers::update_production_order_status),
        )
        .route(
            "/production-orders/{id}/release",
            post(handlers::release_production_order),
        )
        .route(
            "/production-orders/{id}/confirm",
            post(handlers::confirm_production_order),
        )
        // Reports
        .route(
            "/reports/production-analysis",
            get(reports::production_analysis),
        )
        .route(
            "/reports/production-lead-time",
            get(reports::production_lead_time),
        )
}
