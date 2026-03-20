use axum::{
    routing::{get, post, put},
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
            "/sales-orders/{id}/cancel",
            post(handlers::cancel_sales_order),
        )
        .route(
            "/sales-orders/{id}/document-flow",
            get(handlers::get_sales_order_document_flow),
        )
        .route(
            "/sales-orders/{so_id}/items",
            post(handlers::add_sales_order_item),
        )
        .route(
            "/sales-orders/{so_id}/items/{item_id}",
            put(handlers::update_sales_order_item).delete(handlers::delete_sales_order_item),
        )
        .route(
            "/deliveries",
            get(handlers::list_deliveries).post(handlers::create_delivery),
        )
        .route("/deliveries/{id}", get(handlers::get_delivery))
        .route("/deliveries/{id}/ship", post(handlers::ship_delivery))
        .route(
            "/deliveries/{del_id}/items",
            post(handlers::add_delivery_item),
        )
        .route(
            "/deliveries/{del_id}/items/{item_id}",
            put(handlers::update_delivery_item).delete(handlers::delete_delivery_item),
        )
        .route(
            "/invoices",
            get(handlers::list_sd_invoices).post(handlers::create_sd_invoice),
        )
        .route("/invoices/{id}", get(handlers::get_sd_invoice))
        .route(
            "/customers/{id}/sales-orders",
            get(handlers::list_customer_sales_orders),
        )
        // Reports
        .route("/reports/sales-summary", get(reports::sales_summary))
        .route(
            "/reports/order-fulfillment",
            get(reports::order_fulfillment),
        )
        .route("/reports/top-customers", get(reports::top_customers))
}
