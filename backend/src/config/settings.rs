use std::env;

#[derive(Clone, Debug)]
pub struct Settings {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    /// Access token expiry in minutes (used by generate_token; hours kept for backward compat)
    pub access_token_expiry_minutes: i64,
    pub refresh_token_expiry_days: i64,
    pub server_host: String,
    pub server_port: u16,
    // AI Assistant
    pub ai_provider: String,
    pub ai_api_key: Option<String>,
    pub ai_model: String,
    pub ai_max_tokens: u32,
    pub ai_enabled: bool,
}

impl Settings {
    pub fn from_env() -> Self {
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            tracing::error!(
                "JWT_SECRET environment variable is NOT set! Using insecure default — DO NOT use in production."
            );
            // Generate a random secret per process so it's never a known static value
            use rand::Rng;
            let secret: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(64)
                .map(char::from)
                .collect();
            secret
        });

        if jwt_secret.len() < 32 {
            tracing::warn!(
                "JWT_SECRET is shorter than 32 characters — consider using a stronger secret."
            );
        }

        let jwt_expiry_hours: i64 = env::var("JWT_EXPIRY_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .unwrap_or(24);

        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost:5432/TastyByte".to_string()),
            jwt_secret,
            jwt_expiry_hours,
            access_token_expiry_minutes: env::var("ACCESS_TOKEN_EXPIRY_MINUTES")
                .unwrap_or_else(|_| "15".to_string())
                .parse()
                .unwrap_or(15),
            refresh_token_expiry_days: env::var("REFRESH_TOKEN_EXPIRY_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .unwrap_or(7),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .unwrap_or(8000),
            ai_provider: env::var("AI_PROVIDER").unwrap_or_else(|_| "claude".to_string()),
            ai_api_key: env::var("AI_API_KEY").ok().filter(|s| !s.is_empty()),
            ai_model: env::var("AI_MODEL")
                .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string()),
            ai_max_tokens: env::var("AI_MAX_TOKENS")
                .unwrap_or_else(|_| "4096".to_string())
                .parse()
                .unwrap_or(4096),
            ai_enabled: env::var("AI_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }
}
