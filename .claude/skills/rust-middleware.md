# Rust 中間件開發指南

## 目錄
1. [JWT 認證中間件](#jwt-認證中間件)
2. [Rate Limiting](#rate-limiting)
3. [請求日誌](#請求日誌)
4. [CORS 配置](#cors-配置)

## JWT 認證中間件

### JWT 驗證
```rust
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // user_id
    pub email: String,
    pub role: String,
    pub exp: usize,         // expiration
    pub iat: usize,         // issued at
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub role: String,
}

pub async fn auth_middleware<B>(
    State(state): State<Arc<AppState>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // 取得 Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 解析 Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 驗證 JWT
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?
    .claims;

    // 建立 CurrentUser
    let user = CurrentUser {
        id: uuid::Uuid::parse_str(&claims.sub)
            .map_err(|_| StatusCode::UNAUTHORIZED)?,
        email: claims.email,
        role: claims.role,
    };

    // 插入到請求擴展
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
```

### 可選認證中間件
```rust
pub async fn optional_auth_middleware<B>(
    State(state): State<Arc<AppState>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    if let Some(auth_header) = request.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = header_str.strip_prefix("Bearer ") {
                if let Ok(token_data) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
                    &Validation::new(Algorithm::HS256),
                ) {
                    if let Ok(user_id) = uuid::Uuid::parse_str(&token_data.claims.sub) {
                        let user = CurrentUser {
                            id: user_id,
                            email: token_data.claims.email,
                            role: token_data.claims.role,
                        };
                        request.extensions_mut().insert(Some(user));
                        return next.run(request).await;
                    }
                }
            }
        }
    }

    request.extensions_mut().insert(None::<CurrentUser>);
    next.run(request).await
}
```

## Rate Limiting

### Token Bucket 實作
```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
    max_tokens: u32,
    refill_rate: u32,  // tokens per second
}

struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(max_tokens: u32, refill_rate: u32) -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            max_tokens,
            refill_rate,
        }
    }

    pub async fn check(&self, key: &str) -> bool {
        let mut buckets = self.buckets.write().await;

        let bucket = buckets.entry(key.to_string()).or_insert(TokenBucket {
            tokens: self.max_tokens as f64,
            last_refill: Instant::now(),
        });

        // 補充 tokens
        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * self.refill_rate as f64)
            .min(self.max_tokens as f64);
        bucket.last_refill = now;

        // 檢查是否有足夠 tokens
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

// Rate Limiting 中間件
pub async fn rate_limit_middleware<B>(
    State(state): State<Arc<AppState>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // 使用 IP 或 User ID 作為 key
    let key = request
        .headers()
        .get("X-Forwarded-For")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    if !state.rate_limiter.check(&key).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}
```

### In-Memory Per-IP Rate Limiting
```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

pub struct PerIpRateLimiter {
    limits: Arc<RwLock<HashMap<String, (u32, Instant)>>>,
    max_requests: u32,
    window: Duration,
}

impl PerIpRateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }

    pub async fn check(&self, ip: &str) -> bool {
        let mut limits = self.limits.write().await;
        match limits.get_mut(ip) {
            Some((count, start)) if start.elapsed() < self.window => {
                if *count >= self.max_requests { return false; }
                *count += 1;
                true
            }
            _ => {
                limits.insert(ip.to_string(), (1, Instant::now()));
                true
            }
        }
    }
}
```

## 請求日誌

### Tracing 中間件
```rust
use tracing::{info, warn, Span};
use tower_http::trace::{TraceLayer, MakeSpan, OnRequest, OnResponse};
use axum::http::{Request, Response};
use std::time::Duration;

pub fn create_trace_layer() -> TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
    impl MakeSpan<Body> + Clone,
    impl OnRequest<Body> + Clone,
    impl OnResponse<Body> + Clone,
> {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let request_id = uuid::Uuid::new_v4().to_string();
            tracing::info_span!(
                "http_request",
                method = %request.method(),
                uri = %request.uri(),
                request_id = %request_id,
            )
        })
        .on_request(|request: &Request<Body>, _span: &Span| {
            info!(
                method = %request.method(),
                uri = %request.uri(),
                "started processing request"
            );
        })
        .on_response(|response: &Response<Body>, latency: Duration, _span: &Span| {
            info!(
                status = %response.status(),
                latency = ?latency,
                "finished processing request"
            );
        })
}
```

### 結構化日誌設定
```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "backend=debug,tower_http=debug,sqlx=warn".into()
        }))
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}
```

## CORS 配置

### 開發環境 CORS
```rust
use tower_http::cors::{CorsLayer, Any};
use axum::http::{header, Method};

pub fn cors_layer_dev() -> CorsLayer {
    CorsLayer::permissive()
}
```

### 生產環境 CORS
```rust
pub fn cors_layer_prod(allowed_origins: Vec<String>) -> CorsLayer {
    let origins: Vec<_> = allowed_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
}
```
