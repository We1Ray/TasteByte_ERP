use axum::{
    routing::{get, post},
    Router,
};

use crate::sd::handlers;
use crate::sd::reports;
use crate::shared::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/customers",
            get(handlers::list_customers).post(handlers::create_customer),
        )
        .route(
            "/customers/{id}",
            get(handlers::get_customer).put(handlers::update_customer),
        )
        .route("/sales-orders/export", get(handlers::export_sales_orders))
        .route(
            "/sales-orders",
            get(handlers::list_sales_orders).post(handlers::create_sales_order),
        )
        .route("/sales-orders/{id}", get(handlers::get_sales_order))
        .route(
            "/sales-orders/{id}/confirm",
            post(handlers::confirm_sales_order),
        )
        .route(
            "/deliveries",
            get(handlers::list_deliveries).post(handlers::create_delivery),
        )
        .route(
            "/invoices",
            get(handlers::list_sd_invoices).post(handlers::create_sd_invoice),
        )
        // Reports
        .route("/reports/sales-summary", get(reports::sales_summary))
        .route(
            "/reports/order-fulfillment",
            get(reports::order_fulfillment),
        )
        .route("/reports/top-customers", get(reports::top_customers))
}
