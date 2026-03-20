use axum::{
    routing::{get, post, put},
    Router,
};

use crate::shared::types::AppState;
use crate::wm::handlers;
use crate::wm::reports;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/warehouses",
            get(handlers::list_warehouses).post(handlers::create_warehouse),
        )
        .route("/warehouses/{id}", get(handlers::get_warehouse))
        .route(
            "/warehouses/{warehouse_id}/storage-bins",
            get(handlers::list_storage_bins),
        )
        .route("/storage-bins", post(handlers::create_storage_bin))
        .route(
            "/stock-transfers",
            get(handlers::list_stock_transfers).post(handlers::create_stock_transfer),
        )
        .route("/stock-transfers/{id}", get(handlers::get_stock_transfer))
        .route(
            "/stock-counts",
            get(handlers::list_stock_counts).post(handlers::create_stock_count),
        )
        .route("/stock-counts/{id}", get(handlers::get_stock_count))
        .route(
            "/stock-counts/{sc_id}/items",
            post(handlers::add_stock_count_item),
        )
        .route(
            "/stock-counts/{sc_id}/items/{item_id}",
            put(handlers::update_stock_count_item).delete(handlers::delete_stock_count_item),
        )
        .route(
            "/stock-counts/{id}/complete",
            put(handlers::complete_stock_count),
        )
        // Reports
        .route(
            "/reports/warehouse-utilization",
            get(reports::warehouse_utilization),
        )
        .route("/reports/transfer-summary", get(reports::transfer_summary))
}
