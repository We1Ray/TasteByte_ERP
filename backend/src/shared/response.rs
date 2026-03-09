use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }

    pub fn with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: Some(message.into()),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_response_success() {
        let resp = ApiResponse::success("hello");
        assert!(resp.success);
        assert_eq!(resp.data, Some("hello"));
        assert!(resp.error.is_none());
        assert!(resp.message.is_none());
    }

    #[test]
    fn api_response_with_message() {
        let resp = ApiResponse::with_message(42, "Created");
        assert!(resp.success);
        assert_eq!(resp.data, Some(42));
        assert_eq!(resp.message, Some("Created".to_string()));
        assert!(resp.error.is_none());
    }

    #[test]
    fn api_response_serializes_correctly() {
        let resp = ApiResponse::success(vec![1, 2, 3]);
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["success"], true);
        assert_eq!(json["data"], serde_json::json!([1, 2, 3]));
        assert!(json.get("error").is_none());
    }

    #[test]
    fn api_response_with_message_serializes() {
        let resp = ApiResponse::with_message("ok", "Done");
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["message"], "Done");
        assert_eq!(json["data"], "ok");
    }
}
