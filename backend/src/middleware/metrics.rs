use std::time::Instant;

use axum::{body::Body, extract::MatchedPath, http::Request, middleware::Next, response::Response};
use metrics::{counter, histogram};

/// Middleware that records HTTP request metrics (counter + duration histogram).
pub async fn track_metrics(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|mp| mp.as_str().to_owned())
        .unwrap_or_else(|| req.uri().path().to_owned());

    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed().as_secs_f64();

    let status = response.status().as_u16().to_string();
    let method_str = method.to_string();

    counter!(
        "http_requests_total",
        "method" => method_str.clone(),
        "path" => path.clone(),
        "status" => status.clone()
    )
    .increment(1);

    histogram!(
        "http_request_duration_seconds",
        "method" => method_str,
        "path" => path,
        "status" => status
    )
    .record(duration);

    response
}
