use std::time::Duration;

use backend::auth::services::hash_password;
use backend::config::{create_pool, Settings};
use backend::lowcode::services::ai_service::LlmClient;
use backend::middleware::{cors::cors_layer, logging::trace_layer, request_id::request_id_layers};
use backend::routes::build_router;
use backend::schema::run_migrations;
use backend::shared::monitoring::{init_metrics, init_sentry};
use backend::shared::types::AppState;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Load .env
    dotenvy::dotenv().ok();

    // Init Sentry (noop if SENTRY_DSN not set)
    let _sentry_guard = init_sentry();

    // Init tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Init Prometheus metrics
    let metrics_handle = init_metrics();

    // Load config
    let settings = Settings::from_env();
    info!(
        "Starting TasteByte ERP Backend on {}:{}",
        settings.server_host, settings.server_port
    );

    // Create database pool
    let pool = create_pool(&settings.database_url).await;
    info!("Database connection pool created");

    // Run migrations
    if let Err(e) = run_migrations(&pool).await {
        tracing::error!("Migration failed: {}", e);
        std::process::exit(1);
    }

    // Ensure admin password is properly hashed (seed SQL uses a placeholder)
    ensure_admin_password(&pool).await;

    // Initialize AI assistant (optional)
    let llm_client = LlmClient::new(&settings).map(std::sync::Arc::new);
    if llm_client.is_some() {
        info!("AI assistant enabled (provider: {})", settings.ai_provider);
    }

    // Build state
    let state = AppState {
        pool,
        settings: settings.clone(),
        metrics_handle,
        llm_client,
    };

    // Build router with middleware
    let (set_request_id, propagate_request_id) = request_id_layers();
    let app = build_router(state)
        .layer(propagate_request_id)
        .layer(set_request_id)
        .layer(trace_layer())
        .layer(cors_layer())
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)); // 10MB global limit

    // Start server
    let addr = format!("{}:{}", settings.server_host, settings.server_port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");
    info!("Server listening on {}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Server error");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { info!("Received Ctrl+C, starting graceful shutdown"); },
        _ = terminate => { info!("Received SIGTERM, starting graceful shutdown"); },
    }
}

async fn ensure_admin_password(pool: &sqlx::PgPool) {
    let admin_exists: Option<(String,)> =
        sqlx::query_as("SELECT password_hash FROM users WHERE username = 'admin'")
            .fetch_optional(pool)
            .await
            .unwrap_or(None);

    if let Some((current_hash,)) = admin_exists {
        // Check if the hash is the seed placeholder (doesn't start with proper argon2 params)
        // Re-hash with proper argon2 if needed
        use argon2::password_hash::PasswordHash;
        if PasswordHash::new(&current_hash).is_err() || current_hash.contains("YWRtaW4xMjNzYWx0") {
            let new_hash = hash_password("admin123").expect("Failed to hash admin password");
            sqlx::query("UPDATE users SET password_hash = $1 WHERE username = 'admin'")
                .bind(&new_hash)
                .execute(pool)
                .await
                .expect("Failed to update admin password");
            info!("Admin password hash updated");
        }
    }
}
