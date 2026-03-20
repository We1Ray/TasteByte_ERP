use axum::extract::{Path, Query, State};
use axum::response::Response;
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::auth::rbac::{RequireRole, SdRead, SdWrite};
use crate::sd::models::*;
use crate::sd::services;
use crate::shared::audit;
use crate::shared::export::csv_response;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError, ListParams, PaginatedResponse};

// --- Customers ---
pub async fn list_customers(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Customer>>>, AppError> {
    let result = services::list_customers(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_customer(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Customer>>, AppError> {
    let customer = services::get_customer(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(customer)))
}

pub async fn create_customer(
    State(state): State<AppState>,
    _role: RequireRole<SdWrite>,
    Json(input): Json<CreateCustomer>,
) -> Result<Json<ApiResponse<Customer>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let customer = services::create_customer(&state.pool, input).await?;
    Ok(Json(ApiResponse::with_message(
        customer,
        "Customer created",
    )))
}

pub async fn update_customer(
    State(state): State<AppState>,
    role: RequireRole<SdWrite>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateCustomer>,
) -> Result<Json<ApiResponse<Customer>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let customer = services::update_customer(&state.pool, id, input).await?;

    let _ = audit::log_change(
        &state.pool,
        "sd_customers",
        id,
        "UPDATE",
        None,
        serde_json::to_value(&customer).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        customer,
        "Customer updated",
    )))
}

// --- Sales Orders ---
pub async fn list_sales_orders(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<SalesOrder>>>, AppError> {
    let result = services::list_sales_orders(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

#[derive(serde::Serialize)]
pub struct SalesOrderDetail {
    #[serde(flatten)]
    pub order: SalesOrder,
    pub items: Vec<SalesOrderItem>,
}

pub async fn get_sales_order(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SalesOrderDetail>>, AppError> {
    let (order, items) = services::get_sales_order(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(SalesOrderDetail {
        order,
        items,
    })))
}

pub async fn create_sales_order(
    State(state): State<AppState>,
    role: RequireRole<SdWrite>,
    Json(input): Json<CreateSalesOrder>,
) -> Result<Json<ApiResponse<SalesOrder>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let so = services::create_sales_order(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "sd_sales_orders",
        so.id,
        "CREATE",
        None,
        serde_json::to_value(&so).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(so, "Sales order created")))
}

pub async fn confirm_sales_order(
    State(state): State<AppState>,
    role: RequireRole<SdWrite>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SalesOrder>>, AppError> {
    let so = services::confirm_sales_order(&state.pool, id, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "sd_sales_orders",
        so.id,
        "UPDATE",
        serde_json::to_value(serde_json::json!({"status": "DRAFT"})).ok(),
        serde_json::to_value(serde_json::json!({"status": &so.status})).ok(),
        Some(role.claims.sub),
    )
    .await;

    // Notify the user who created the SO
    if let Some(created_by) = so.created_by {
        crate::notifications::services::notify(
            &state.pool,
            created_by,
            "Sales Order Confirmed",
            &format!("Sales order {} has been confirmed.", so.order_number),
            "success",
            Some("SD"),
            Some(so.id),
        )
        .await;
    }

    Ok(Json(ApiResponse::with_message(so, "Sales order confirmed")))
}

// --- Deliveries ---
pub async fn list_deliveries(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Delivery>>>, AppError> {
    let result = services::list_deliveries(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_delivery(
    State(state): State<AppState>,
    role: RequireRole<SdWrite>,
    Json(input): Json<CreateDelivery>,
) -> Result<Json<ApiResponse<Delivery>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let delivery = services::create_delivery(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "sd_deliveries",
        delivery.id,
        "CREATE",
        None,
        serde_json::to_value(&delivery).ok(),
        Some(role.claims.sub),
    )
    .await;

    // Notify the acting user about delivery creation
    crate::notifications::services::notify(
        &state.pool,
        role.claims.sub,
        "Delivery Created",
        &format!("Delivery {} has been created.", delivery.delivery_number),
        "success",
        Some("SD"),
        Some(delivery.id),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        delivery,
        "Delivery created",
    )))
}

// --- Ship Delivery ---
pub async fn ship_delivery(
    State(state): State<AppState>,
    role: RequireRole<SdWrite>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Delivery>>, AppError> {
    let delivery = services::ship_delivery(&state.pool, id, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "sd_deliveries",
        delivery.id,
        "UPDATE",
        serde_json::to_value(serde_json::json!({"status": "CREATED"})).ok(),
        serde_json::to_value(serde_json::json!({"status": &delivery.status})).ok(),
        Some(role.claims.sub),
    )
    .await;

    crate::notifications::services::notify(
        &state.pool,
        role.claims.sub,
        "Delivery Shipped",
        &format!("Delivery {} has been shipped.", delivery.delivery_number),
        "success",
        Some("SD"),
        Some(delivery.id),
    )
    .await;

    Ok(Json(ApiResponse::with_message(
        delivery,
        "Delivery marked as shipped",
    )))
}

// --- Get Delivery Detail ---
#[derive(serde::Serialize)]
pub struct DeliveryDetail {
    #[serde(flatten)]
    pub delivery: Delivery,
    pub items: Vec<DeliveryItem>,
}

pub async fn get_delivery(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DeliveryDetail>>, AppError> {
    let (delivery, items) = services::get_delivery(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(DeliveryDetail {
        delivery,
        items,
    })))
}

// --- SD Invoices ---
pub async fn list_sd_invoices(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<SdInvoice>>>, AppError> {
    let result = services::list_sd_invoices(&state.pool, &params).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn create_sd_invoice(
    State(state): State<AppState>,
    role: RequireRole<SdWrite>,
    Json(input): Json<CreateSdInvoice>,
) -> Result<Json<ApiResponse<SdInvoice>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    let invoice = services::create_sd_invoice(&state.pool, input, role.claims.sub).await?;

    let _ = audit::log_change(
        &state.pool,
        "sd_invoices",
        invoice.id,
        "CREATE",
        None,
        serde_json::to_value(&invoice).ok(),
        Some(role.claims.sub),
    )
    .await;

    Ok(Json(ApiResponse::with_message(invoice, "Invoice created")))
}

// --- Export Sales Orders ---
pub async fn export_sales_orders(
    State(state): State<AppState>,
    _role: RequireRole<SdRead>,
) -> Result<Response, AppError> {
    let orders =
        sqlx::query_as::<_, SalesOrder>("SELECT * FROM sd_sales_orders ORDER BY created_at DESC")
            .fetch_all(&state.pool)
            .await?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record([
        "Order Number",
        "Customer ID",
        "Order Date",
        "Requested Delivery Date",
        "Status",
        "Total Amount",
        "Currency",
        "Notes",
        "Created At",
    ])
    .map_err(|e| AppError::Internal(e.to_string()))?;

    for o in &orders {
        let customer_id = o.customer_id.to_string();
        let order_date = o.order_date.to_string();
        let delivery_date = o
            .requested_delivery_date
            .map(|d| d.to_string())
            .unwrap_or_default();
        let total = o.total_amount.to_string();
        let created = o.created_at.to_rfc3339();
        wtr.write_record([
            o.order_number.as_str(),
            customer_id.as_str(),
            order_date.as_str(),
            delivery_date.as_str(),
            o.status.as_str(),
            total.as_str(),
            o.currency.as_str(),
            o.notes.as_deref().unwrap_or(""),
            created.as_str(),
        ])
        .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let csv_data = String::from_utf8(
        wtr.into_inner()
            .map_err(|e| AppError::Internal(e.to_string()))?,
    )
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(csv_response(csv_data, "sales-orders-export.csv"))
}
