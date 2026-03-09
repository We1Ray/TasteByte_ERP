use axum::http::header;
use axum::response::{IntoResponse, Response};

pub fn csv_response(csv_data: String, filename: &str) -> Response {
    let safe_filename = filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect::<String>();
    let headers = [
        (header::CONTENT_TYPE, "text/csv; charset=utf-8".to_string()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", safe_filename),
        ),
    ];
    (headers, csv_data).into_response()
}
