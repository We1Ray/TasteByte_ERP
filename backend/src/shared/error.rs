use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Database(err) => {
                tracing::error!("Database error: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "success": false,
            "error": message,
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found_display() {
        let err = AppError::NotFound("missing".to_string());
        assert_eq!(err.to_string(), "Not found: missing");
    }

    #[test]
    fn unauthorized_display() {
        let err = AppError::Unauthorized("bad token".to_string());
        assert_eq!(err.to_string(), "Unauthorized: bad token");
    }

    #[test]
    fn validation_display() {
        let err = AppError::Validation("invalid input".to_string());
        assert_eq!(err.to_string(), "Validation error: invalid input");
    }

    #[test]
    fn forbidden_display() {
        let err = AppError::Forbidden("no access".to_string());
        assert_eq!(err.to_string(), "Forbidden: no access");
    }

    #[test]
    fn internal_display() {
        let err = AppError::Internal("crash".to_string());
        assert_eq!(err.to_string(), "Internal error: crash");
    }
}
