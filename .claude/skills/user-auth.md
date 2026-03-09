# user-auth Skill

處理 TasteByte ERP 使用者認證與授權，確保系統安全。

## Security Architecture
```
┌─────────────────────────────────────────────────────────┐
│                    Authentication Flow                   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────┐    ┌──────────┐    ┌──────────┐              │
│  │Client│───▶│ Rust API │───▶│ JWT Auth │              │
│  │      │    │ (Axum)   │    │ Middleware│              │
│  └──────┘    └──────────┘    └──────────┘              │
│       │                           │                     │
│       │    ┌──────────────┐      │                     │
│       └───▶│ PostgreSQL   │◀─────┘                     │
│            │ (port 5432)  │                             │
│            └──────────────┘                             │
└─────────────────────────────────────────────────────────┘
```

## Implementation

### Password Hashing (Rust)
```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
```

### JWT Token Management (Rust)
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,           // user_id
    pub email: String,
    pub roles: Vec<String>,
    pub auth_objects: HashMap<String, Vec<String>>,
    pub exp: usize,
    pub iat: usize,
}

pub struct TokenService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl TokenService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn create_access_token(&self, user_id: &str, email: &str, roles: &[String], auth_objects: &HashMap<String, Vec<String>>) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            roles: roles.to_vec(),
            auth_objects: auth_objects.clone(),
            exp: (now + Duration::minutes(15)).timestamp() as usize,
            iat: now.timestamp() as usize,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(e.to_string()))
    }

    pub fn create_refresh_token(&self, user_id: &str) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            email: String::new(),
            roles: vec![],
            auth_objects: HashMap::new(),
            exp: (now + Duration::days(7)).timestamp() as usize,
            iat: now.timestamp() as usize,
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(e.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|_| AppError::Unauthorized)
    }
}
```

### RBAC with SAP-like Authorization Objects (Rust)
```rust
use std::collections::HashMap;

pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub auth_objects: HashMap<String, Vec<String>>,
}

impl CurrentUser {
    /// Check if user has required authorization
    /// e.g., check_auth_object("S_SD_ORDER", "CREATE")
    pub fn check_auth_object(&self, object: &str, action: &str) -> Result<(), AppError> {
        if let Some(actions) = self.auth_objects.get(object) {
            if actions.contains(&action.to_string()) || actions.contains(&"ALL".to_string()) {
                return Ok(());
            }
        }
        Err(AppError::Forbidden(format!(
            "Missing authorization: {} - {}", object, action
        )))
    }
}

// Auth middleware
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    let claims = state.token_service.verify_token(token)?;

    let user = CurrentUser {
        id: Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?,
        email: claims.email,
        roles: claims.roles,
        auth_objects: claims.auth_objects,
    };

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
```

### Rate Limiting (Rust)
```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    limits: HashMap<String, (u32, Duration)>,
    counters: Arc<RwLock<HashMap<String, (u32, Instant)>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        let mut limits = HashMap::new();
        limits.insert("login".into(), (5, Duration::from_secs(300)));
        limits.insert("api".into(), (100, Duration::from_secs(60)));
        Self { limits, counters: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn check_limit(&self, key: &str, action: &str) -> bool {
        let (limit, window) = self.limits.get(action)
            .copied()
            .unwrap_or((100, Duration::from_secs(60)));

        let rate_key = format!("{}:{}", action, key);
        let mut counters = self.counters.write().await;

        match counters.get_mut(&rate_key) {
            Some((count, start)) if start.elapsed() < window => {
                if *count >= limit { return false; }
                *count += 1;
                true
            }
            _ => {
                counters.insert(rate_key, (1, Instant::now()));
                true
            }
        }
    }
}
```

## Security Checklist
- [x] 密碼使用 Argon2 雜湊
- [x] JWT 短效期 (15min) + Refresh Token (7d)
- [x] SAP-like Authorization Objects (RBAC)
- [x] Rate Limiting 防暴力破解
- [x] 帳號鎖定機制
- [x] 稽核日誌記錄 (Audit Trail)
