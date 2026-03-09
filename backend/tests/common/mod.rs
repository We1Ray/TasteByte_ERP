use std::net::SocketAddr;
use std::sync::OnceLock;

use axum_test::TestServer;
use backend::config::Settings;
use backend::routes::build_router;
use backend::schema::run_migrations;
use backend::shared::types::AppState;
use metrics_exporter_prometheus::PrometheusHandle;
use sqlx::postgres::PgPoolOptions;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// Install the Prometheus recorder at most once per test process.
fn metrics_handle() -> PrometheusHandle {
    static HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();
    HANDLE
        .get_or_init(|| {
            metrics_exporter_prometheus::PrometheusBuilder::new()
                .install_recorder()
                .expect("Failed to install Prometheus recorder")
        })
        .clone()
}

static MIGRATED: AtomicBool = AtomicBool::new(false);

pub async fn setup_server() -> TestServer {
    dotenvy::dotenv().ok();
    let settings = Settings::from_env();

    // Create a fresh pool per test (each tokio runtime needs its own pool)
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(0)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&settings.database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations once (racy but idempotent)
    if !MIGRATED.load(Ordering::SeqCst) {
        run_migrations(&pool)
            .await
            .expect("Failed to run migrations in test setup");
        MIGRATED.store(true, Ordering::SeqCst);
    }

    let state = AppState {
        pool,
        settings,
        metrics_handle: metrics_handle(),
    };
    let router = build_router(state);
    // Use into_make_service_with_connect_info so rate limiter can extract peer IP
    TestServer::new(router.into_make_service_with_connect_info::<SocketAddr>())
        .expect("Failed to create test server")
}
