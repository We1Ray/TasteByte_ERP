use std::env;

use metrics_exporter_prometheus::PrometheusHandle;
use sentry::ClientInitGuard;

/// Initialize Sentry error tracking. Returns `None` if `SENTRY_DSN` is not set
/// or is empty, making Sentry completely optional.
pub fn init_sentry() -> Option<ClientInitGuard> {
    let dsn = env::var("SENTRY_DSN").ok()?;
    if dsn.is_empty() {
        return None;
    }

    let guard = sentry::init(sentry::ClientOptions {
        dsn: Some(dsn.parse().expect("Invalid SENTRY_DSN")),
        release: sentry::release_name!(),
        traces_sample_rate: 0.1,
        environment: env::var("APP_ENV").ok().map(Into::into),
        ..Default::default()
    });

    tracing::info!("Sentry initialized");
    Some(guard)
}

/// Install the Prometheus metrics recorder and return a handle for rendering.
pub fn init_metrics() -> PrometheusHandle {
    let builder = metrics_exporter_prometheus::PrometheusBuilder::new();
    builder
        .install_recorder()
        .expect("Failed to install Prometheus recorder")
}
