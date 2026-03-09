use metrics_exporter_prometheus::PrometheusHandle;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Settings;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub settings: Settings,
    pub metrics_handle: PrometheusHandle,
    pub llm_client: Option<std::sync::Arc<crate::lowcode::services::ai_service::LlmClient>>,
}

pub const ADMIN_ROLE: &str = "ADMIN";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub username: String,
    pub roles: Vec<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn is_admin(&self) -> bool {
        self.roles.iter().any(|r| r == ADMIN_ROLE)
    }
}
