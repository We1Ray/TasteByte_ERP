# data-validation Skill

驗證所有輸入資料，防止惡意輸入和資料損壞。

## Validation Layers
```
┌─────────────────────────────────────────────────────────┐
│                   Validation Pipeline                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Input ──▶ Schema ──▶ Sanitize ──▶ Business ──▶ Store   │
│           Validation   Input       Rules       Data     │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Implementation

### Schema Validation (Rust - validator crate)
```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserCreateRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 12, max = 128, message = "Password must be 12-128 characters"))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: String,

    #[validate(length(min = 2, max = 100, message = "Name must be 2-100 characters"))]
    pub name: String,
}

fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "!@#$%^&*(),.?\":{}|<>".contains(c));

    if !has_upper || !has_lower || !has_digit || !has_special {
        return Err(validator::ValidationError::new("password_strength")
            .with_message("Password must contain uppercase, lowercase, digit, and special character".into()));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct DocumentUploadRequest {
    #[validate(length(max = 255))]
    #[validate(custom(function = "validate_filename"))]
    pub filename: String,

    #[validate(custom(function = "validate_content_type"))]
    pub content_type: String,

    #[validate(range(min = 1, max = 52428800, message = "File size must be 1B - 50MB"))]
    pub size: i64,
}

fn validate_filename(filename: &str) -> Result<(), validator::ValidationError> {
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err(validator::ValidationError::new("invalid_filename")
            .with_message("Invalid filename: path traversal detected".into()));
    }

    let allowed_extensions = ["pdf", "xlsx", "xls", "doc", "docx", "csv"];
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    if !allowed_extensions.contains(&ext.as_str()) {
        return Err(validator::ValidationError::new("invalid_extension")
            .with_message(format!("File type not allowed: .{}", ext).into()));
    }
    Ok(())
}

fn validate_content_type(content_type: &str) -> Result<(), validator::ValidationError> {
    let allowed_types = [
        "application/pdf",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "application/vnd.ms-excel",
        "application/msword",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "text/csv",
    ];
    if !allowed_types.contains(&content_type) {
        return Err(validator::ValidationError::new("invalid_content_type")
            .with_message(format!("Content type not allowed: {}", content_type).into()));
    }
    Ok(())
}

// Usage in Axum handler
pub async fn create_user(
    Json(payload): Json<UserCreateRequest>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    payload.validate().map_err(AppError::from)?;
    // ... proceed with creation
}
```

### Schema Validation (TypeScript - Zod)
```typescript
import { z } from "zod";

export const userCreateSchema = z.object({
  email: z.string().email("Invalid email format"),
  password: z
    .string()
    .min(12, "Password must be at least 12 characters")
    .max(128)
    .regex(/[A-Z]/, "Must contain uppercase letter")
    .regex(/[a-z]/, "Must contain lowercase letter")
    .regex(/\d/, "Must contain digit")
    .regex(/[!@#$%^&*(),.?":{}\|<>]/, "Must contain special character"),
  name: z
    .string()
    .min(2, "Name must be at least 2 characters")
    .max(100)
    .transform((v) => v.replace(/[<>&"']/g, "").trim()),
});

export type UserCreateInput = z.infer<typeof userCreateSchema>;
```

### Input Sanitization (Rust)
```rust
use regex::Regex;

pub struct Sanitizer;

impl Sanitizer {
    /// HTML escape special characters
    pub fn html_escape(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }

    /// Strip HTML tags
    pub fn strip_html(text: &str) -> String {
        let re = Regex::new(r"<[^>]*>").unwrap();
        re.replace_all(text, "").to_string()
    }

    /// Sanitize filename to safe characters only
    pub fn sanitize_filename(filename: &str) -> String {
        filename
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '.' || c == '_' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }

    /// Escape SQL LIKE wildcards
    pub fn sanitize_sql_like(text: &str) -> String {
        text.replace('%', "\\%").replace('_', "\\_")
    }
}
```

### SQL Injection Prevention (Rust - SQLx)
```rust
use sqlx::PgPool;
use uuid::Uuid;

// SQLx uses parameterized queries by default via bind()
// This is the ONLY correct way to query

// Correct: parameterized query
pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM sys_users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

// NEVER do this: string interpolation in SQL
// let query = format!("SELECT * FROM sys_users WHERE id = '{}'", user_id);
```

## Validation Error Response
```json
{
    "success": false,
    "data": null,
    "error": {
        "code": "VALIDATION_ERROR",
        "message": "Invalid input data",
        "details": {
            "fields": {
                "password": ["Must contain uppercase letter"],
                "email": ["Invalid email format"]
            }
        }
    }
}
```
