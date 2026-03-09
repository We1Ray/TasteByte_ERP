use std::env;

use axum::http::{header, HeaderName, HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    let origins = env::var("CORS_ORIGINS").unwrap_or_default();

    let allow_origin = if origins.is_empty() || origins == "*" {
        // Development fallback: allow localhost origins only
        tracing::warn!(
            "CORS_ORIGINS not set — allowing localhost only. Set CORS_ORIGINS for production."
        );
        AllowOrigin::list([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:8000".parse::<HeaderValue>().unwrap(),
        ])
    } else {
        let parsed: Vec<HeaderValue> = origins
            .split(',')
            .filter_map(|s| s.trim().parse::<HeaderValue>().ok())
            .collect();
        AllowOrigin::list(parsed)
    };

    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            HeaderName::from_static("x-request-id"),
        ])
        .allow_credentials(true)
}
