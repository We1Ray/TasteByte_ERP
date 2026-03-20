use axum::extract::State;
use axum::middleware as axum_mw;
use axum::routing::{delete, get, post, put};
use axum::Json;
use axum::Router;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::middleware::metrics::track_metrics;
use crate::shared::admin_api;
use crate::shared::types::{AppState, Claims};
use crate::shared::{ApiResponse, AppError};

pub fn build_router(state: AppState) -> Router {
    let api = Router::new()
        .route("/dashboard/kpis", get(dashboard_kpis))
        .route("/dashboard/charts", get(dashboard_charts))
        .nest("/auth", crate::auth::routes::routes())
        .nest("/fi", crate::fi::routes::routes())
        .nest("/co", crate::co::routes::routes())
        .nest("/mm", crate::mm::routes::routes())
        .nest("/sd", crate::sd::routes::routes())
        .nest("/pp", crate::pp::routes::routes())
        .nest("/hr", crate::hr::routes::routes())
        .nest("/wm", crate::wm::routes::routes())
        .nest("/qm", crate::qm::routes::routes())
        .nest("/lowcode", crate::lowcode::routes::routes())
        .nest("/notifications", crate::notifications::routes::routes())
        .route(
            "/workflow/history/{doc_type}/{doc_id}",
            get(crate::shared::handlers::get_status_history),
        )
        .nest("/system", system_routes());

    Router::new()
        .nest("/api/v1", api)
        .route("/health", get(health_check))
        .route("/metrics", get(metrics_endpoint))
        .layer(axum_mw::from_fn(track_metrics))
        .with_state(state)
}

async fn health_check(
    State(state): State<AppState>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)>
{
    let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .is_ok();

    let pool_size = state.pool.size();
    let pool_idle = state.pool.num_idle();
    let pool_info = serde_json::json!({
        "active_connections": pool_size - pool_idle as u32,
        "idle_connections": pool_idle,
        "max_connections": state.pool.options().get_max_connections(),
    });

    if db_ok {
        Ok(axum::Json(serde_json::json!({
            "status": "ok",
            "service": "TasteByte ERP Backend",
            "version": env!("CARGO_PKG_VERSION"),
            "database": "connected",
            "pool": pool_info
        })))
    } else {
        Err((
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            axum::Json(serde_json::json!({
                "status": "degraded",
                "service": "TasteByte ERP Backend",
                "version": env!("CARGO_PKG_VERSION"),
                "database": "disconnected",
                "pool": pool_info
            })),
        ))
    }
}

async fn metrics_endpoint(State(state): State<AppState>) -> String {
    state.metrics_handle.render()
}

#[derive(Serialize, sqlx::FromRow)]
pub struct DashboardKpi {
    pub total_revenue: Decimal,
    pub total_order_count: i64,
    pub total_inventory_quantity: Decimal,
    pub pending_production_orders: i64,
    pub open_ar_amount: Decimal,
    pub open_ap_amount: Decimal,
}

async fn query_total_revenue(pool: &sqlx::PgPool) -> Result<Decimal, AppError> {
    let (val,): (Decimal,) = sqlx::query_as(
        "SELECT COALESCE(SUM(ji.credit_amount), 0) \
         FROM fi_journal_items ji \
         JOIN fi_accounts a ON a.id = ji.account_id \
         JOIN fi_journal_entries je ON je.id = ji.journal_entry_id \
         WHERE a.account_type = 'REVENUE' AND je.status = 'POSTED'",
    )
    .fetch_one(pool)
    .await?;
    Ok(val)
}

async fn query_total_orders(pool: &sqlx::PgPool) -> Result<i64, AppError> {
    let (val,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM sd_sales_orders WHERE status != 'CANCELLED'")
            .fetch_one(pool)
            .await?;
    Ok(val)
}

async fn query_total_inventory(pool: &sqlx::PgPool) -> Result<Decimal, AppError> {
    let (val,): (Decimal,) =
        sqlx::query_as("SELECT COALESCE(SUM(quantity), 0) FROM mm_plant_stock")
            .fetch_one(pool)
            .await?;
    Ok(val)
}

async fn query_pending_production(pool: &sqlx::PgPool) -> Result<i64, AppError> {
    let (val,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM pp_production_orders WHERE status IN ('CREATED', 'PLANNED', 'IN_PROGRESS')",
    )
    .fetch_one(pool)
    .await?;
    Ok(val)
}

async fn query_open_ar(pool: &sqlx::PgPool) -> Result<Decimal, AppError> {
    let (val,): (Decimal,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_amount - paid_amount), 0) \
         FROM fi_ar_invoices WHERE status NOT IN ('PAID', 'CANCELLED')",
    )
    .fetch_one(pool)
    .await?;
    Ok(val)
}

async fn query_open_ap(pool: &sqlx::PgPool) -> Result<Decimal, AppError> {
    let (val,): (Decimal,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_amount - paid_amount), 0) \
         FROM fi_ap_invoices WHERE status NOT IN ('PAID', 'CANCELLED')",
    )
    .fetch_one(pool)
    .await?;
    Ok(val)
}

async fn dashboard_kpis(
    State(state): State<AppState>,
    _claims: Claims,
) -> Result<Json<ApiResponse<DashboardKpi>>, AppError> {
    let (
        total_revenue,
        total_order_count,
        total_inventory_quantity,
        pending_production_orders,
        open_ar_amount,
        open_ap_amount,
    ) = tokio::try_join!(
        query_total_revenue(&state.pool),
        query_total_orders(&state.pool),
        query_total_inventory(&state.pool),
        query_pending_production(&state.pool),
        query_open_ar(&state.pool),
        query_open_ap(&state.pool),
    )?;

    Ok(Json(ApiResponse::success(DashboardKpi {
        total_revenue,
        total_order_count,
        total_inventory_quantity,
        pending_production_orders,
        open_ar_amount,
        open_ap_amount,
    })))
}

#[derive(Serialize)]
pub struct DashboardCharts {
    pub monthly_revenue: Vec<MonthlyRevenue>,
    pub monthly_orders: Vec<MonthlyOrders>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct MonthlyRevenue {
    pub month: String,
    pub revenue: Decimal,
    pub costs: Decimal,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct MonthlyOrders {
    pub month: String,
    pub orders: i64,
    pub delivered: i64,
}

async fn dashboard_charts(
    State(state): State<AppState>,
    _claims: Claims,
) -> Result<Json<ApiResponse<DashboardCharts>>, AppError> {
    // Get monthly revenue from posted journal entries (last 6 months)
    let monthly_revenue: Vec<MonthlyRevenue> = sqlx::query_as(
        r#"WITH months AS (
            SELECT TO_CHAR(d, 'Mon') as month, EXTRACT(MONTH FROM d)::int as month_num,
                   EXTRACT(YEAR FROM d)::int as year_num
            FROM generate_series(
                DATE_TRUNC('month', CURRENT_DATE - INTERVAL '5 months'),
                DATE_TRUNC('month', CURRENT_DATE),
                '1 month'
            ) d
        )
        SELECT m.month,
            COALESCE(SUM(CASE WHEN a.account_type = 'REVENUE' THEN ji.credit_amount ELSE 0 END), 0) as revenue,
            COALESCE(SUM(CASE WHEN a.account_type = 'EXPENSE' THEN ji.debit_amount ELSE 0 END), 0) as costs
        FROM months m
        LEFT JOIN fi_journal_entries je ON EXTRACT(MONTH FROM je.posting_date) = m.month_num
            AND EXTRACT(YEAR FROM je.posting_date) = m.year_num
            AND je.status = 'POSTED'
        LEFT JOIN fi_journal_items ji ON ji.journal_entry_id = je.id
        LEFT JOIN fi_accounts a ON a.id = ji.account_id AND a.account_type IN ('REVENUE', 'EXPENSE')
        GROUP BY m.month, m.month_num, m.year_num
        ORDER BY m.year_num, m.month_num"#,
    )
    .fetch_all(&state.pool)
    .await?;

    // Get monthly order counts (last 6 months)
    let monthly_orders: Vec<MonthlyOrders> = sqlx::query_as(
        r#"WITH months AS (
            SELECT TO_CHAR(d, 'Mon') as month, EXTRACT(MONTH FROM d)::int as month_num,
                   EXTRACT(YEAR FROM d)::int as year_num
            FROM generate_series(
                DATE_TRUNC('month', CURRENT_DATE - INTERVAL '5 months'),
                DATE_TRUNC('month', CURRENT_DATE),
                '1 month'
            ) d
        )
        SELECT m.month,
            COUNT(CASE WHEN so.id IS NOT NULL THEN 1 END) as orders,
            COUNT(CASE WHEN so.status IN ('DELIVERED', 'COMPLETED', 'INVOICED') THEN 1 END) as delivered
        FROM months m
        LEFT JOIN sd_sales_orders so ON EXTRACT(MONTH FROM so.order_date) = m.month_num
            AND EXTRACT(YEAR FROM so.order_date) = m.year_num
            AND so.status != 'CANCELLED'
        GROUP BY m.month, m.month_num, m.year_num
        ORDER BY m.year_num, m.month_num"#,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(DashboardCharts {
        monthly_revenue,
        monthly_orders,
    })))
}

fn system_routes() -> Router<AppState> {
    Router::new()
        // User preferences
        .route(
            "/preferences",
            get(admin_api::get_user_preferences).put(admin_api::update_user_preferences),
        )
        // Saved variants
        .route(
            "/variants",
            get(admin_api::list_variants).post(admin_api::create_variant),
        )
        .route("/variants/{id}", delete(admin_api::delete_variant))
        // System health
        .route("/health", get(admin_api::system_health))
        // Global search
        .route("/search", get(admin_api::global_search))
        // Email management
        .route("/email/templates", get(admin_api::list_email_templates))
        .route("/email/logs", get(admin_api::list_email_logs_handler))
        // Scheduled jobs
        .route(
            "/jobs",
            get(admin_api::list_scheduled_jobs).post(admin_api::create_scheduled_job),
        )
        .route(
            "/jobs/{id}",
            put(admin_api::toggle_scheduled_job).delete(admin_api::delete_scheduled_job),
        )
        .route("/jobs/{id}/logs", get(admin_api::get_job_logs))
        // Webhooks
        .route(
            "/webhooks",
            get(admin_api::list_webhooks_handler).post(admin_api::create_webhook_handler),
        )
        .route("/webhooks/{id}", delete(admin_api::delete_webhook_handler))
        // Print layouts
        .route(
            "/print/layouts",
            get(admin_api::list_print_layouts_handler).post(admin_api::save_print_layout_handler),
        )
        .route(
            "/print/render/{layout_code}",
            post(admin_api::render_print_handler),
        )
        // Transport orders
        .route(
            "/transports",
            get(admin_api::list_transport_orders).post(admin_api::create_transport_order),
        )
        // Approval Matrix
        .route(
            "/approvals/matrices",
            get(admin_api::list_approval_matrices).post(admin_api::create_approval_matrix),
        )
        .route(
            "/approvals/matrices/{id}",
            get(admin_api::get_approval_matrix).delete(admin_api::delete_approval_matrix),
        )
        .route("/approvals/submit", post(admin_api::submit_for_approval))
        .route(
            "/approvals/instances/{id}/action",
            post(admin_api::process_approval_action),
        )
        .route(
            "/approvals/instances/{id}",
            get(admin_api::get_approval_instance),
        )
        .route("/approvals/pending", get(admin_api::list_pending_approvals))
        // BPM Workflows
        .route(
            "/workflows",
            get(admin_api::list_workflows).post(admin_api::create_workflow),
        )
        .route(
            "/workflows/{id}",
            put(admin_api::update_workflow).delete(admin_api::delete_workflow),
        )
        .route("/workflows/{id}/start", post(admin_api::start_workflow))
        .route(
            "/workflows/instances/{id}/advance",
            post(admin_api::advance_workflow),
        )
        .route(
            "/workflows/instances",
            get(admin_api::list_workflow_instances),
        )
        .route(
            "/workflows/instances/{id}/logs",
            get(admin_api::get_workflow_logs),
        )
        // Cross-field Rules & Formulas
        .route(
            "/rules/{operation_id}",
            get(admin_api::list_cross_field_rules).post(admin_api::create_cross_field_rule),
        )
        .route(
            "/rules/{operation_id}/{id}",
            delete(admin_api::delete_cross_field_rule),
        )
        .route(
            "/formulas/{operation_id}",
            get(admin_api::list_formulas).post(admin_api::create_formula),
        )
        .route(
            "/formulas/{operation_id}/{id}",
            delete(admin_api::delete_formula),
        )
        // Output Determination
        .route(
            "/outputs",
            get(admin_api::list_output_rules).post(admin_api::create_output_rule),
        )
        .route("/outputs/{id}", delete(admin_api::delete_output_rule))
        .route(
            "/outputs/logs/{operation_id}",
            get(admin_api::list_output_logs),
        )
        // Number Range Config
        .route("/number-ranges", get(admin_api::list_number_ranges))
        .route("/number-ranges/{id}", put(admin_api::update_number_range))
        // Auth Trace
        .route("/auth-trace/{user_id}", get(admin_api::get_auth_trace))
        .route(
            "/auth-trace/{user_id}/denials",
            get(admin_api::get_auth_denials),
        )
        // Form Variants
        .route(
            "/form-variants/{operation_id}",
            get(admin_api::list_form_variants).post(admin_api::create_form_variant),
        )
        .route(
            "/form-variants/{operation_id}/{id}",
            delete(admin_api::delete_form_variant),
        )
        // Import Template
        .route(
            "/import-template/{code}",
            get(admin_api::download_import_template),
        )
        // Reports
        .route(
            "/reports",
            get(admin_api::list_reports).post(admin_api::create_report),
        )
        .route(
            "/reports/{id}",
            get(admin_api::get_report).delete(admin_api::delete_report),
        )
        // Analytics
        .route("/analytics/track", post(admin_api::track_analytics_event))
        .route("/analytics/summary", get(admin_api::get_analytics_summary))
        .route(
            "/analytics/operation/{operation_id}",
            get(admin_api::get_operation_analytics),
        )
        // Dashboard Templates
        .route(
            "/dashboard-templates",
            get(admin_api::list_dashboard_templates),
        )
        // Exchange Rates
        .route(
            "/exchange-rates",
            get(admin_api::list_exchange_rates).post(admin_api::create_exchange_rate),
        )
        .route(
            "/exchange-rates/{id}",
            delete(admin_api::delete_exchange_rate),
        )
        .route("/exchange-rates/convert", get(admin_api::convert_currency))
}
