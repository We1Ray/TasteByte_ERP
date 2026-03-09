use axum::body::Body;
use axum::extract::{Multipart, Path, State};
use axum::http::header;
use axum::response::Response;
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformUser, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

const UPLOAD_DIR: &str = "uploads/lowcode";
const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50MB

pub async fn upload_file(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<FileUpload>>, AppError> {
    let mut operation_id: Option<Uuid> = None;
    let mut record_id: Option<Uuid> = None;
    let mut field_id: Option<Uuid> = None;
    let mut file_name: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut content_type: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Validation(format!("Multipart error: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "operation_id" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| AppError::Validation(format!("Read error: {e}")))?;
                operation_id = Some(
                    text.parse()
                        .map_err(|_| AppError::Validation("Invalid operation_id".to_string()))?,
                );
            }
            "record_id" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| AppError::Validation(format!("Read error: {e}")))?;
                record_id = Some(
                    text.parse()
                        .map_err(|_| AppError::Validation("Invalid record_id".to_string()))?,
                );
            }
            "field_id" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| AppError::Validation(format!("Read error: {e}")))?;
                field_id = Some(
                    text.parse()
                        .map_err(|_| AppError::Validation("Invalid field_id".to_string()))?,
                );
            }
            "file" => {
                file_name = field.file_name().map(|s| s.to_string());
                content_type = field.content_type().map(|s| s.to_string());
                file_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|e| AppError::Validation(format!("Read error: {e}")))?
                        .to_vec(),
                );
            }
            _ => {}
        }
    }

    let operation_id =
        operation_id.ok_or_else(|| AppError::Validation("operation_id is required".to_string()))?;
    let original_name =
        file_name.ok_or_else(|| AppError::Validation("File is required".to_string()))?;
    let data = file_data.ok_or_else(|| AppError::Validation("File data is empty".to_string()))?;

    if data.len() > MAX_FILE_SIZE {
        return Err(AppError::Validation(format!(
            "File too large. Maximum size is {}MB",
            MAX_FILE_SIZE / 1024 / 1024
        )));
    }

    // Create upload directory
    let dir = format!("{UPLOAD_DIR}/{operation_id}");
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create directory: {e}")))?;

    // Generate unique filename
    let file_id = Uuid::new_v4();
    let ext = std::path::Path::new(&original_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");
    let storage_name = format!("{file_id}.{ext}");
    let storage_path = format!("{dir}/{storage_name}");

    // Write file
    tokio::fs::write(&storage_path, &data)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to write file: {e}")))?;

    let file_size = data.len() as i64;

    // Detect content type via extension if not provided
    let detected_type = content_type.unwrap_or_else(|| "application/octet-stream".to_string());

    let upload = sqlx::query_as::<_, FileUpload>(
        "INSERT INTO lc_file_uploads (operation_id, record_id, field_id, file_name, file_type, file_size, storage_path, uploaded_by) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
    )
    .bind(operation_id)
    .bind(record_id)
    .bind(field_id)
    .bind(&original_name)
    .bind(&detected_type)
    .bind(file_size)
    .bind(&storage_path)
    .bind(guard.claims.sub)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(upload, "File uploaded")))
}

pub async fn download_file(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformUser>,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    let upload = sqlx::query_as::<_, FileUpload>("SELECT * FROM lc_file_uploads WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("File not found".to_string()))?;

    // Ownership check: only uploader or ADMIN can download
    let is_admin = guard.claims.is_admin();
    let is_owner = upload.uploaded_by == Some(guard.claims.sub);
    if !is_admin && !is_owner {
        // Also allow if user has access to the operation (platform role check already passed)
        let has_operation_access = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM lc_operation_permissions op \
             JOIN lc_platform_roles pr ON pr.id = op.role_id \
             JOIN lc_user_platform_roles upr ON upr.role_id = pr.id \
             WHERE op.operation_id = $1 AND upr.user_id = $2 AND op.can_read = true)",
        )
        .bind(upload.operation_id)
        .bind(guard.claims.sub)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(false);

        if !has_operation_access {
            return Err(AppError::Forbidden(
                "You do not have permission to download this file".to_string(),
            ));
        }
    }

    let data = tokio::fs::read(&upload.storage_path)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to read file: {e}")))?;

    let content_type = upload
        .file_type
        .unwrap_or_else(|| "application/octet-stream".to_string());

    // Sanitize filename: strip path separators, control chars, quotes, and non-ASCII
    let safe_filename: String = upload
        .file_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '-' || *c == '_' || *c == ' ')
        .collect();
    let safe_filename = if safe_filename.is_empty() {
        "download".to_string()
    } else {
        safe_filename
    };

    let response = Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", safe_filename),
        )
        .body(Body::from(data))
        .map_err(|e| AppError::Internal(format!("Response build error: {e}")))?;

    Ok(response)
}
