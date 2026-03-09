use axum::{extract::FromRequestParts, http::request::Parts};

use crate::auth::services;
use crate::shared::types::{AppState, Claims};
use crate::shared::AppError;

impl FromRequestParts<AppState> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing authorization token".to_string()))?;

        services::validate_token(token, &state.settings.jwt_secret)
    }
}
