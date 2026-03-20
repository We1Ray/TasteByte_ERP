use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;

use crate::shared::types::{AppState, Claims};
use crate::shared::{analytics, approval, auth_trace, bpm, cross_field, email, exchange_rate, output_determination, print_layout, report_builder, scheduler, webhook};
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

// -- Approval Matrix ---------------------------------------------------------

pub async fn list_approval_matrices(
    State(state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> Result<Json<ApiResponse<Vec<approval::ApprovalMatrix>>>, AppError> {
    let op_id = params
        .get("operation_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());
    let matrices = approval::list_matrices(&state.pool, op_id).await?;
    Ok(Json(ApiResponse::success(matrices)))
}

pub async fn create_approval_matrix(
    State(state): State<AppState>,
    Json(input): Json<approval::CreateApprovalMatrix>,
) -> Result<Json<ApiResponse<approval::ApprovalMatrixWithLevels>>, AppError> {
    let result = approval::create_matrix(&state.pool, input).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn get_approval_matrix(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<approval::ApprovalMatrixWithLevels>>, AppError> {
    let result = approval::get_matrix_with_levels(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(result)))
}

pub async fn delete_approval_matrix(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    approval::delete_matrix(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({"deleted": true}),
    )))
}

pub async fn submit_for_approval(
    State(state): State<AppState>,
    claims: Claims,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<approval::ApprovalInstance>>, AppError> {
    let matrix_id = input
        .get("matrix_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| AppError::Validation("matrix_id required".to_string()))?;
    let operation_id = input
        .get("operation_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| AppError::Validation("operation_id required".to_string()))?;
    let record_id = input
        .get("record_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| AppError::Validation("record_id required".to_string()))?;
    let instance = approval::submit_for_approval(
        &state.pool,
        matrix_id,
        operation_id,
        record_id,
        claims.sub,
    )
    .await?;
    Ok(Json(ApiResponse::success(instance)))
}

pub async fn process_approval_action(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<approval::ApprovalInstance>>, AppError> {
    let action = input
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("APPROVE");
    let comment = input.get("comment").and_then(|v| v.as_str());
    let instance =
        approval::process_approval(&state.pool, id, action, claims.sub, comment).await?;
    Ok(Json(ApiResponse::success(instance)))
}

pub async fn get_approval_instance(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let (instance, actions) = approval::get_instance_with_actions(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "instance": instance, "actions": actions }),
    )))
}

pub async fn list_pending_approvals(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<ApiResponse<Vec<approval::ApprovalInstance>>>, AppError> {
    let instances = approval::list_pending_approvals(&state.pool, claims.sub).await?;
    Ok(Json(ApiResponse::success(instances)))
}

// -- BPM Workflows -----------------------------------------------------------

pub async fn list_workflows(
    State(state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> Result<Json<ApiResponse<Vec<bpm::WorkflowDefinition>>>, AppError> {
    let op_id = params
        .get("operation_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());
    let defs = bpm::list_definitions(&state.pool, op_id).await?;
    Ok(Json(ApiResponse::success(defs)))
}

pub async fn create_workflow(
    State(state): State<AppState>,
    claims: Claims,
    Json(input): Json<bpm::CreateWorkflowDefinition>,
) -> Result<Json<ApiResponse<bpm::WorkflowDefinition>>, AppError> {
    let def = bpm::create_definition(&state.pool, input, claims.sub).await?;
    Ok(Json(ApiResponse::success(def)))
}

pub async fn update_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<bpm::CreateWorkflowDefinition>,
) -> Result<Json<ApiResponse<bpm::WorkflowDefinition>>, AppError> {
    let def = bpm::update_definition(&state.pool, id, input).await?;
    Ok(Json(ApiResponse::success(def)))
}

pub async fn delete_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    bpm::delete_definition(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({"deleted": true}),
    )))
}

pub async fn start_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<bpm::WorkflowInstance>>, AppError> {
    let op_id = input
        .get("operation_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| AppError::Validation("operation_id required".to_string()))?;
    let record_id = input
        .get("record_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| AppError::Validation("record_id required".to_string()))?;
    let instance = bpm::start_instance(&state.pool, id, op_id, record_id).await?;
    Ok(Json(ApiResponse::success(instance)))
}

pub async fn advance_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<bpm::WorkflowInstance>>, AppError> {
    let target = input
        .get("target_node")
        .and_then(|v| v.as_str())
        .unwrap_or("next");
    let instance = bpm::advance_instance(&state.pool, id, target).await?;
    Ok(Json(ApiResponse::success(instance)))
}

pub async fn list_workflow_instances(
    State(state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> Result<Json<ApiResponse<Vec<bpm::WorkflowInstance>>>, AppError> {
    let op_id = params
        .get("operation_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| AppError::Validation("operation_id required".to_string()))?;
    let instances = bpm::list_instances(&state.pool, op_id).await?;
    Ok(Json(ApiResponse::success(instances)))
}

pub async fn get_workflow_logs(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<bpm::WorkflowExecLog>>>, AppError> {
    let logs = bpm::get_instance_logs(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(logs)))
}

// -- Cross-field Rules & Formulas --------------------------------------------

pub async fn list_cross_field_rules(
    State(state): State<AppState>,
    Path(operation_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<cross_field::CrossFieldRule>>>, AppError> {
    let rules = cross_field::list_rules(&state.pool, operation_id).await?;
    Ok(Json(ApiResponse::success(rules)))
}

pub async fn create_cross_field_rule(
    State(state): State<AppState>,
    Path(operation_id): Path<Uuid>,
    Json(input): Json<cross_field::CreateCrossFieldRule>,
) -> Result<Json<ApiResponse<cross_field::CrossFieldRule>>, AppError> {
    let rule = cross_field::create_rule(&state.pool, operation_id, input).await?;
    Ok(Json(ApiResponse::success(rule)))
}

pub async fn delete_cross_field_rule(
    State(state): State<AppState>,
    Path((_operation_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    cross_field::delete_rule(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({"deleted": true}),
    )))
}

pub async fn list_formulas(
    State(state): State<AppState>,
    Path(operation_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<cross_field::CalculationFormula>>>, AppError> {
    let formulas = cross_field::list_formulas(&state.pool, operation_id).await?;
    Ok(Json(ApiResponse::success(formulas)))
}

pub async fn create_formula(
    State(state): State<AppState>,
    Path(operation_id): Path<Uuid>,
    Json(input): Json<cross_field::CreateCalculationFormula>,
) -> Result<Json<ApiResponse<cross_field::CalculationFormula>>, AppError> {
    let formula = cross_field::create_formula(&state.pool, operation_id, input).await?;
    Ok(Json(ApiResponse::success(formula)))
}

pub async fn delete_formula(
    State(state): State<AppState>,
    Path((_operation_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    cross_field::delete_formula(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({"deleted": true}),
    )))
}

// -- Output Determination ----------------------------------------------------

pub async fn list_output_rules(
    State(state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> Result<Json<ApiResponse<Vec<output_determination::OutputRule>>>, AppError> {
    let op_id = params
        .get("operation_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());
    let rules = output_determination::list_rules(&state.pool, op_id).await?;
    Ok(Json(ApiResponse::success(rules)))
}

pub async fn create_output_rule(
    State(state): State<AppState>,
    Json(input): Json<output_determination::CreateOutputRule>,
) -> Result<Json<ApiResponse<output_determination::OutputRule>>, AppError> {
    let rule = output_determination::create_rule(&state.pool, input).await?;
    Ok(Json(ApiResponse::success(rule)))
}

pub async fn delete_output_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    output_determination::delete_rule(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({"deleted": true}),
    )))
}

pub async fn list_output_logs(
    State(state): State<AppState>,
    Path(operation_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<output_determination::OutputLog>>>, AppError> {
    let logs = output_determination::list_logs(&state.pool, operation_id).await?;
    Ok(Json(ApiResponse::success(logs)))
}

// -- Number Range Config -----------------------------------------------------

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct NumberRangeConfig {
    pub id: Uuid,
    pub range_prefix: String,
    pub description: Option<String>,
    pub current_value: i64,
    pub start_value: i64,
    pub end_value: i64,
    pub padding: i32,
    pub separator: String,
    pub fiscal_year_dependent: bool,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_number_ranges(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<NumberRangeConfig>>>, AppError> {
    let ranges = sqlx::query_as::<_, NumberRangeConfig>(
        "SELECT * FROM number_range_config ORDER BY range_prefix",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(ApiResponse::success(ranges)))
}

pub async fn update_number_range(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<NumberRangeConfig>>, AppError> {
    let desc = input.get("description").and_then(|v| v.as_str());
    let padding = input
        .get("padding")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
    let separator = input.get("separator").and_then(|v| v.as_str());
    let fiscal = input
        .get("fiscal_year_dependent")
        .and_then(|v| v.as_bool());
    let active = input.get("is_active").and_then(|v| v.as_bool());

    let range = sqlx::query_as::<_, NumberRangeConfig>(
        "UPDATE number_range_config SET description = COALESCE($2, description), padding = COALESCE($3, padding), separator = COALESCE($4, separator), fiscal_year_dependent = COALESCE($5, fiscal_year_dependent), is_active = COALESCE($6, is_active), updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(desc)
    .bind(padding)
    .bind(separator)
    .bind(fiscal)
    .bind(active)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Number range not found".to_string()))?;
    Ok(Json(ApiResponse::success(range)))
}

// -- Auth Trace --------------------------------------------------------------

pub async fn get_auth_trace(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<auth_trace::AuthTraceEntry>>>, AppError> {
    let entries = auth_trace::get_user_trace(&state.pool, user_id, 50).await?;
    Ok(Json(ApiResponse::success(entries)))
}

pub async fn get_auth_denials(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<auth_trace::AuthTraceEntry>>>, AppError> {
    let entries = auth_trace::get_recent_denials(&state.pool, user_id).await?;
    Ok(Json(ApiResponse::success(entries)))
}

// -- Form Variants -----------------------------------------------------------

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct FormVariant {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub variant_name: String,
    pub condition_field: Option<String>,
    pub condition_value: Option<String>,
    pub hidden_fields: Vec<String>,
    pub readonly_fields: Vec<String>,
    pub required_fields: Vec<String>,
    pub default_values: serde_json::Value,
    pub layout_overrides: serde_json::Value,
    pub is_default: bool,
    pub sort_order: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateFormVariant {
    pub variant_name: String,
    pub condition_field: Option<String>,
    pub condition_value: Option<String>,
    pub hidden_fields: Option<Vec<String>>,
    pub readonly_fields: Option<Vec<String>>,
    pub required_fields: Option<Vec<String>>,
    pub default_values: Option<serde_json::Value>,
    pub layout_overrides: Option<serde_json::Value>,
    pub is_default: Option<bool>,
}

pub async fn list_form_variants(
    State(state): State<AppState>,
    Path(operation_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<FormVariant>>>, AppError> {
    let variants = sqlx::query_as::<_, FormVariant>(
        "SELECT * FROM form_variants WHERE operation_id = $1 ORDER BY sort_order",
    )
    .bind(operation_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(ApiResponse::success(variants)))
}

pub async fn create_form_variant(
    State(state): State<AppState>,
    Path(operation_id): Path<Uuid>,
    Json(input): Json<CreateFormVariant>,
) -> Result<Json<ApiResponse<FormVariant>>, AppError> {
    let variant = sqlx::query_as::<_, FormVariant>(
        "INSERT INTO form_variants (operation_id, variant_name, condition_field, condition_value, hidden_fields, readonly_fields, required_fields, default_values, layout_overrides, is_default) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) RETURNING *",
    )
    .bind(operation_id)
    .bind(&input.variant_name)
    .bind(&input.condition_field)
    .bind(&input.condition_value)
    .bind(input.hidden_fields.as_deref().unwrap_or(&[]))
    .bind(input.readonly_fields.as_deref().unwrap_or(&[]))
    .bind(input.required_fields.as_deref().unwrap_or(&[]))
    .bind(
        input
            .default_values
            .as_ref()
            .unwrap_or(&serde_json::json!({})),
    )
    .bind(
        input
            .layout_overrides
            .as_ref()
            .unwrap_or(&serde_json::json!({})),
    )
    .bind(input.is_default.unwrap_or(false))
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(ApiResponse::success(variant)))
}

pub async fn delete_form_variant(
    State(state): State<AppState>,
    Path((_operation_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    sqlx::query("DELETE FROM form_variants WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({"deleted": true}),
    )))
}

// -- Import Template Download ------------------------------------------------

pub async fn download_import_template(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<axum::response::Response, AppError> {
    use axum::http::header;
    use axum::response::IntoResponse;

    let operation = sqlx::query_as::<_, crate::lowcode::models::Operation>(
        "SELECT * FROM lc_operations WHERE operation_code = $1",
    )
    .bind(&code)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    let form =
        crate::lowcode::services::form_builder::get_form(&state.pool, operation.id).await?;
    let fields: Vec<_> = form
        .sections
        .iter()
        .flat_map(|s| s.fields.iter())
        .collect();

    let mut wtr = csv::Writer::from_writer(Vec::new());

    // Header row: field labels
    let headers: Vec<String> = fields.iter().map(|f| f.field.field_label.clone()).collect();
    wtr.write_record(&headers)
        .map_err(|e| AppError::Internal(format!("CSV: {}", e)))?;

    // Second row: field keys (for mapping reference)
    let keys: Vec<String> = fields.iter().map(|f| f.field.field_name.clone()).collect();
    wtr.write_record(&keys)
        .map_err(|e| AppError::Internal(format!("CSV: {}", e)))?;

    // Third row: example/hints
    let hints: Vec<String> = fields
        .iter()
        .map(|f| {
            let mut hint = format!("[{}]", f.field.field_type);
            if f.field.is_required {
                hint.push_str(" *Required");
            }
            if let Some(ref dv) = f.field.default_value {
                hint.push_str(&format!(" Default:{}", dv));
            }
            hint
        })
        .collect();
    wtr.write_record(&hints)
        .map_err(|e| AppError::Internal(format!("CSV: {}", e)))?;

    let csv_bytes = wtr
        .into_inner()
        .map_err(|e| AppError::Internal(format!("CSV: {}", e)))?;
    let filename = format!("{}_import_template.csv", code);

    Ok((
        [
            (
                header::CONTENT_TYPE,
                "text/csv; charset=utf-8".to_string(),
            ),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        csv_bytes,
    )
        .into_response())
}

// ── Reports ────────────────────────────────────────────────────────
pub async fn list_reports(State(state): State<AppState>) -> Result<Json<ApiResponse<Vec<report_builder::ReportDefinition>>>, AppError> {
    Ok(Json(ApiResponse::success(report_builder::list_reports(&state.pool).await?)))
}
pub async fn get_report(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<ApiResponse<report_builder::ReportDefinition>>, AppError> {
    Ok(Json(ApiResponse::success(report_builder::get_report(&state.pool, id).await?)))
}
pub async fn create_report(State(state): State<AppState>, claims: Claims, Json(input): Json<report_builder::CreateReport>) -> Result<Json<ApiResponse<report_builder::ReportDefinition>>, AppError> {
    Ok(Json(ApiResponse::success(report_builder::create_report(&state.pool, input, claims.sub).await?)))
}
pub async fn delete_report(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    report_builder::delete_report(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(serde_json::json!({"deleted":true}))))
}

// ── Analytics ──────────────────────────────────────────────────────
pub async fn track_analytics_event(State(state): State<AppState>, claims: Claims, Json(input): Json<analytics::TrackEvent>) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    analytics::track_event(&state.pool, claims.sub, input).await?;
    Ok(Json(ApiResponse::success(serde_json::json!({"tracked":true}))))
}
pub async fn get_analytics_summary(State(state): State<AppState>, Query(params): Query<serde_json::Value>) -> Result<Json<ApiResponse<Vec<analytics::UsageSummary>>>, AppError> {
    let days = params.get("days").and_then(|v| v.as_i64()).unwrap_or(30) as i32;
    Ok(Json(ApiResponse::success(analytics::get_summary(&state.pool, days).await?)))
}
pub async fn get_operation_analytics(State(state): State<AppState>, Path(operation_id): Path<Uuid>) -> Result<Json<ApiResponse<Vec<analytics::UsageSummary>>>, AppError> {
    Ok(Json(ApiResponse::success(analytics::get_operation_stats(&state.pool, operation_id).await?)))
}

// ── Dashboard Templates ────────────────────────────────────────────
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct DashboardTemplate {
    pub id: Uuid,
    pub template_code: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub definition: serde_json::Value,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
pub async fn list_dashboard_templates(State(state): State<AppState>) -> Result<Json<ApiResponse<Vec<DashboardTemplate>>>, AppError> {
    Ok(Json(ApiResponse::success(sqlx::query_as::<_, DashboardTemplate>("SELECT * FROM dashboard_templates WHERE is_active=true ORDER BY category,name").fetch_all(&state.pool).await?)))
}

// ── Exchange Rates ─────────────────────────────────────────────────
pub async fn list_exchange_rates(State(state): State<AppState>) -> Result<Json<ApiResponse<Vec<exchange_rate::ExchangeRate>>>, AppError> {
    Ok(Json(ApiResponse::success(exchange_rate::list_rates(&state.pool).await?)))
}
pub async fn create_exchange_rate(State(state): State<AppState>, Json(input): Json<exchange_rate::CreateExchangeRate>) -> Result<Json<ApiResponse<exchange_rate::ExchangeRate>>, AppError> {
    Ok(Json(ApiResponse::success(exchange_rate::create_rate(&state.pool, input).await?)))
}
pub async fn delete_exchange_rate(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    exchange_rate::delete_rate(&state.pool, id).await?;
    Ok(Json(ApiResponse::success(serde_json::json!({"deleted":true}))))
}
pub async fn convert_currency(State(state): State<AppState>, Query(params): Query<serde_json::Value>) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let from = params.get("from").and_then(|v| v.as_str()).unwrap_or("USD");
    let to = params.get("to").and_then(|v| v.as_str()).unwrap_or("TWD");
    let amount: rust_decimal::Decimal = params.get("amount").and_then(|v| v.as_str()).and_then(|s| s.parse().ok()).unwrap_or(rust_decimal::Decimal::ONE);
    let date = chrono::Utc::now().date_naive();
    let result = exchange_rate::convert(&state.pool, from, to, amount, date).await?;
    Ok(Json(ApiResponse::success(serde_json::json!({"from":from,"to":to,"amount":amount.to_string(),"result":result.to_string(),"date":date.to_string()}))))
}
