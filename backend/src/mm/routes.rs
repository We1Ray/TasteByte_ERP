use axum::{
    routing::{get, post},
    Router,
};

use crate::mm::handlers;
use crate::mm::reports;
use crate::shared::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        // UOMs
        .route("/uoms", get(handlers::list_uoms).post(handlers::create_uom))
        // Material Groups
        .route(
            "/material-groups",
            get(handlers::list_material_groups).post(handlers::create_material_group),
        )
        // Materials
        .route("/materials/export", get(handlers::export_materials))
        .route(
            "/materials",
            get(handlers::list_materials).post(handlers::create_material),
        )
        .route(
            "/materials/{id}",
            get(handlers::get_material)
                .put(handlers::update_material)
                .delete(handlers::delete_material),
        )
        // Vendors
        .route(
            "/vendors",
            get(handlers::list_vendors).post(handlers::create_vendor),
        )
        .route(
            "/vendors/{id}",
            get(handlers::get_vendor).put(handlers::update_vendor),
        )
        // Plant Stock
        .route("/plant-stock", get(handlers::list_plant_stock))
        // Material Movements
        .route(
            "/material-movements",
            get(handlers::list_material_movements).post(handlers::create_material_movement),
        )
        // Purchase Orders
        .route(
            "/purchase-orders",
            get(handlers::list_purchase_orders).post(handlers::create_purchase_order),
        )
        .route("/purchase-orders/{id}", get(handlers::get_purchase_order))
        .route(
            "/purchase-orders/{id}/release",
            post(handlers::release_purchase_order),
        )
        .route(
            "/purchase-orders/{id}/receive",
            post(handlers::receive_purchase_order),
        )
        // Goods Issue (convenience endpoint)
        .route("/goods-issue", post(handlers::goods_issue))
        // Reports
        .route("/reports/stock-valuation", get(reports::stock_valuation))
        .route("/reports/movement-summary", get(reports::movement_summary))
        .route("/reports/slow-moving", get(reports::slow_moving))
}
