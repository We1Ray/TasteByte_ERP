use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;

use crate::shared::types::{AppState, Claims};
use crate::shared::{email, print_layout, scheduler, webhook};
use crate::shared::{ApiResponse, AppError};

// -- User Preferences -------------------------------------------------------

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct UserPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub language: String,
    pub timezone: String,
    pub date_format: String,
    pub theme: String,
    pub notifications_enabled: bool,
    pub email_notifications: bool,
    pub page_size: i32,
    pub sidebar_collapsed: bool,
    pub custom_settings: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserPreference {
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub date_format: Option<String>,
    pub theme: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub email_notifications: Option<bool>,
    pub page_size: Option<i32>,
    pub sidebar_collapsed: Option<bool>,
    pub custom_settings: Option<serde_json::Value>,
}

pub async fn get_user_preferences(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<ApiResponse<UserPreference>>, AppError> {
    let pref = sqlx::query_as::<_, UserPreference>(
        "SELECT * FROM user_preferences WHERE user_id = $1",
    )
    .bind(claims.sub)
    .fetch_optional(&state.pool)
    .await?;

    match pref {
        Some(p) => Ok(Json(ApiResponse::success(p))),
        None => {
            // Create default preferences
            let p = sqlx::query_as::<_, UserPreference>(
                "INSERT INTO user_preferences (user_id) VALUES ($1) RETURNING *",
            )
            .bind(claims.sub)
            .fetch_one(&state.pool)
            .await?;
            Ok(Json(ApiResponse::success(p)))
        }
    }
}

pub async fn update_user_preferences(
    State(state): State<AppState>,
    claims: Claims,
    Json(input): Json<UpdateUserPreference>,
) -> Result<Json<ApiResponse<UserPreference>>, AppError> {
    let pref = sqlx::query_as::<_, UserPreference>(
        "INSERT INTO user_preferences (user_id, language, timezone, date_format, theme, notifications_enabled, email_notifications, page_size, sidebar_collapsed, custom_settings) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) \
         ON CONFLICT (user_id) DO UPDATE SET \
         language = COALESCE($2, user_preferences.language), \
         timezone = COALESCE($3, user_preferences.timezone), \
         date_format = COALESCE($4, user_preferences.date_format), \
         theme = COALESCE($5, user_preferences.theme), \
         notifications_enabled = COALESCE($6, user_preferences.notifications_enabled), \
         email_notifications = COALESCE($7, user_preferences.email_notifications), \
         page_size = COALESCE($8, user_preferences.page_size), \
         sidebar_collapsed = COALESCE($9, user_preferences.sidebar_collapsed), \
         custom_settings = COALESCE($10, user_preferences.custom_settings), \
         updated_at = NOW() \
         RETURNING *",
    )
    .bind(claims.sub)
    .bind(input.language)
    .bind(input.timezone)
    .bind(input.date_format)
    .bind(input.theme)
    .bind(input.notifications_enabled)
    .bind(input.email_notifications)
    .bind(input.page_size)
    .bind(input.sidebar_collapsed)
    .bind(input.custom_settings)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(pref)))
}

// -- Saved Variants ----------------------------------------------------------

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct SavedVariant {
    pub id: Uuid,
    pub user_id: Uuid,
    pub context: String,
    pub variant_name: String,
    pub is_default: bool,
    pub filters: serde_json::Value,
    pub columns: Option<serde_json::Value>,
    pub sort_config: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct VariantQuery {
    pub context: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateVariant {
    pub context: String,
    pub variant_name: String,
    pub is_default: Option<bool>,
    pub filters: serde_json::Value,
    pub columns: Option<serde_json::Value>,
    pub sort_config: Option<serde_json::Value>,
}

pub async fn list_variants(
    State(state): State<AppState>,
    claims: Claims,
    Query(q): Query<VariantQuery>,
) -> Result<Json<ApiResponse<Vec<SavedVariant>>>, AppError> {
    let variants = sqlx::query_as::<_, SavedVariant>(
        "SELECT * FROM saved_variants WHERE user_id = $1 AND context = $2 ORDER BY variant_name",
    )
    .bind(claims.sub)
    .bind(&q.context)
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(ApiResponse::success(variants)))
}

pub async fn create_variant(
    State(state): State<AppState>,
    claims: Claims,
    Json(input): Json<CreateVariant>,
) -> Result<Json<ApiResponse<SavedVariant>>, AppError> {
    if input.is_default.unwrap_or(false) {
        // Unset other defaults
        sqlx::query(
            "UPDATE saved_variants SET is_default = false WHERE user_id = $1 AND context = $2",
        )
        .bind(claims.sub)
        .bind(&input.context)
        .execute(&state.pool)
        .await?;
    }

    let variant = sqlx::query_as::<_, SavedVariant>(
        "INSERT INTO saved_variants (user_id, context, variant_name, is_default, filters, columns, sort_config) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(claims.sub)
    .bind(&input.context)
    .bind(&input.variant_name)
    .bind(input.is_default.unwrap_or(false))
    .bind(&input.filters)
    .bind(&input.columns)
    .bind(&input.sort_config)
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(ApiResponse::success(variant)))
}

pub async fn delete_variant(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    sqlx::query("DELETE FROM saved_variants WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(claims.sub)
        .execute(&state.pool)
        .await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({"deleted": true}),
    )))
}

// -- System Health -----------------------------------------------------------

#[derive(Debug, serde::Serialize)]
pub struct SystemHealth {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub database: DatabaseHealth,
    pub stats: SystemStats,
}

#[derive(Debug, serde::Serialize)]
pub struct DatabaseHealth {
    pub connected: bool,
    pub pool_size: u32,
    pub idle_connections: u32,
}

#[derive(Debug, serde::Serialize)]
pub struct SystemStats {
    pub total_users: i64,
    pub total_operations: i64,
    pub total_records: i64,
    pub pending_releases: i64,
    pub active_jobs: i64,
}

static START_TIME: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

pub fn init_start_time() {
    START_TIME.get_or_init(std::time::Instant::now);
}

pub async fn system_health(State(state): State<AppState>) -> Result<Json<SystemHealth>, AppError> {
    let pool = &state.pool;

    let uptime = START_TIME
        .get()
        .map(|t| t.elapsed().as_secs())
        .unwrap_or(0);

    let db_ok = sqlx::query("SELECT 1").execute(pool).await.is_ok();
    let pool_size = pool.size();
    let idle = pool.num_idle() as u32;

    let (users,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    let (operations,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM lc_operations")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    let (records,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM lc_operation_data")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    let (pending,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM lc_releases WHERE status = 'SUBMITTED'")
            .fetch_one(pool)
            .await
            .unwrap_or((0,));
    let (jobs,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM scheduled_jobs WHERE is_active = true")
            .fetch_one(pool)
            .await
            .unwrap_or((0,));

    Ok(Json(SystemHealth {
        status: if db_ok {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        database: DatabaseHealth {
            connected: db_ok,
            pool_size,
            idle_connections: idle,
        },
        stats: SystemStats {
            total_users: users,
            total_operations: operations,
            total_records: records,
            pending_releases: pending,
            active_jobs: jobs,
        },
    }))
}

// -- Global Search -----------------------------------------------------------

#[derive(Debug, serde::Deserialize)]
pub struct GlobalSearchQuery {
    pub q: String,
    pub limit: Option<i64>,
}

#[derive(Debug, serde::Serialize)]
pub struct GlobalSearchResult {
    pub category: String,
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub url: String,
}

pub async fn global_search(
    State(state): State<AppState>,
    Query(query): Query<GlobalSearchQuery>,
) -> Result<Json<ApiResponse<Vec<GlobalSearchResult>>>, AppError> {
    let q = query.q.trim();
    if q.is_empty() {
        return Ok(Json(ApiResponse::success(vec![])));
    }
    let limit = query.limit.unwrap_or(20).clamp(1, 50);
    let pattern = format!("%{}%", q);
    let mut results = Vec::new();

    // Search operations
    let ops: Vec<(Uuid, String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, operation_code, name, description FROM lc_operations WHERE name ILIKE $1 OR operation_code ILIKE $1 LIMIT $2",
    )
    .bind(&pattern)
    .bind(limit)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    for (id, code, name, desc) in ops {
        results.push(GlobalSearchResult {
            category: "Operation".to_string(),
            id: id.to_string(),
            title: format!("{} ({})", name, code),
            subtitle: desc,
            url: format!("/developer/operations/{}", id),
        });
    }

    // Search projects
    let projects: Vec<(Uuid, String, Option<String>)> = sqlx::query_as(
        "SELECT id, name, description FROM lc_projects WHERE name ILIKE $1 LIMIT $2",
    )
    .bind(&pattern)
    .bind(limit)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    for (id, name, desc) in projects {
        results.push(GlobalSearchResult {
            category: "Project".to_string(),
            id: id.to_string(),
            title: name,
            subtitle: desc,
            url: "/admin/projects".to_string(),
        });
    }

    // Search users
    let users: Vec<(Uuid, String, Option<String>)> = sqlx::query_as(
        "SELECT id, username, full_name FROM users WHERE username ILIKE $1 OR full_name ILIKE $1 LIMIT $2",
    )
    .bind(&pattern)
    .bind(limit)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    for (id, username, full_name) in users {
        results.push(GlobalSearchResult {
            category: "User".to_string(),
            id: id.to_string(),
            title: full_name.unwrap_or(username),
            subtitle: None,
            url: "/admin/users".to_string(),
        });
    }

    results.truncate(limit as usize);
    Ok(Json(ApiResponse::success(results)))
}

// -- Email/Scheduler/Webhook/Print CRUD endpoints ----------------------------

pub async fn list_email_templates(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<email::EmailTemplate>>>, AppError> {
    let templates = email::list_templates(&state.pool).await?;
    Ok(Json(ApiResponse::success(templates)))
}

pub async fn list_email_logs_handler(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<email::EmailLog>>>, AppError> {
    let logs = email::list_email_logs(&state.pool, 100).await?;
    Ok(Json(ApiResponse::success(logs)))
}

pub async fn list_scheduled_jobs(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<scheduler::ScheduledJob>>>, AppError> {
    let jobs = scheduler::list_jobs(&state.pool).await?;
    Ok(Json(ApiResponse::success(jobs)))
}

pub async fn create_scheduled_job(
    State(state): State<AppState>,
    claims: Claims,
    Json(input): Json<scheduler::CreateScheduledJob>,
) -> Result<Json<ApiResponse<scheduler::ScheduledJob>>, AppError> {
    let job = scheduler::create_job(&state.pool, input, claims.sub).await?;
    Ok(Json(ApiResponse::success(job)))
}

pub async fn toggle_scheduled_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<scheduler::ScheduledJob>>, AppError> {
    let is_active = body
        .get("is_active")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let job = scheduler::toggle_job(&state.pool, id, is_active).await?;
    Ok(Json(ApiResponse::success(job)))
}

pub async fn delete_scheduled_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    scheduler::delete_job(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({"deleted": true}),
    )))
}

pub async fn get_job_logs(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<scheduler::JobExecutionLog>>>, AppError> {
    let logs = scheduler::get_execution_logs(&state.pool, id, 50).await?;
    Ok(Json(ApiResponse::success(logs)))
}

pub async fn list_webhooks_handler(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<webhook::Webhook>>>, AppError> {
    let hooks = webhook::list_webhooks(&state.pool).await?;
    Ok(Json(ApiResponse::success(hooks)))
}

pub async fn create_webhook_handler(
    State(state): State<AppState>,
    claims: Claims,
    Json(input): Json<webhook::CreateWebhook>,
) -> Result<Json<ApiResponse<webhook::Webhook>>, AppError> {
    let hook = webhook::create_webhook(&state.pool, input, claims.sub).await?;
    Ok(Json(ApiResponse::success(hook)))
}

pub async fn delete_webhook_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    webhook::delete_webhook(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({"deleted": true}),
    )))
}

pub async fn list_print_layouts_handler(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<print_layout::PrintLayout>>>, AppError> {
    let layouts = print_layout::list_layouts(&state.pool).await?;
    Ok(Json(ApiResponse::success(layouts)))
}

pub async fn save_print_layout_handler(
    State(state): State<AppState>,
    Json(input): Json<print_layout::CreatePrintLayout>,
) -> Result<Json<ApiResponse<print_layout::PrintLayout>>, AppError> {
    let layout = print_layout::save_layout(&state.pool, input).await?;
    Ok(Json(ApiResponse::success(layout)))
}

pub async fn render_print_handler(
    State(state): State<AppState>,
    Path(layout_code): Path<String>,
    Json(data): Json<serde_json::Value>,
) -> Result<axum::response::Html<String>, AppError> {
    let layout = print_layout::get_layout(&state.pool, &layout_code).await?;
    let html = print_layout::render_layout(&layout, &data);
    Ok(axum::response::Html(html))
}

// -- Transport Orders --------------------------------------------------------

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct TransportOrder {
    pub id: Uuid,
    pub transport_number: String,
    pub description: Option<String>,
    pub source_env: String,
    pub target_env: String,
    pub status: String,
    pub payload: serde_json::Value,
    pub object_type: String,
    pub object_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub applied_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateTransportOrder {
    pub description: Option<String>,
    pub target_env: String,
    pub object_type: String,
    pub object_id: Option<Uuid>,
    pub payload: serde_json::Value,
}

pub async fn list_transport_orders(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<TransportOrder>>>, AppError> {
    let orders = sqlx::query_as::<_, TransportOrder>(
        "SELECT * FROM transport_orders ORDER BY created_at DESC LIMIT 100",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(ApiResponse::success(orders)))
}

pub async fn create_transport_order(
    State(state): State<AppState>,
    claims: Claims,
    Json(input): Json<CreateTransportOrder>,
) -> Result<Json<ApiResponse<TransportOrder>>, AppError> {
    let transport_number =
        crate::shared::number_range::next_number(&state.pool, "TRN").await?;
    let order = sqlx::query_as::<_, TransportOrder>(
        "INSERT INTO transport_orders (transport_number, description, target_env, object_type, object_id, payload, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(&transport_number)
    .bind(&input.description)
    .bind(&input.target_env)
    .bind(&input.object_type)
    .bind(input.object_id)
    .bind(&input.payload)
    .bind(claims.sub)
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(ApiResponse::success(order)))
}
