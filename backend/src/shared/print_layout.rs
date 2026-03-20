use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PrintLayout {
    pub id: Uuid,
    pub layout_code: String,
    pub name: String,
    pub description: Option<String>,
    pub operation_id: Option<Uuid>,
    pub template_html: String,
    pub paper_size: String,
    pub orientation: String,
    pub margins: serde_json::Value,
    pub header_html: Option<String>,
    pub footer_html: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePrintLayout {
    pub layout_code: String,
    pub name: String,
    pub description: Option<String>,
    pub operation_id: Option<Uuid>,
    pub template_html: String,
    pub paper_size: Option<String>,
    pub orientation: Option<String>,
    pub header_html: Option<String>,
    pub footer_html: Option<String>,
}

pub async fn list_layouts(pool: &PgPool) -> Result<Vec<PrintLayout>, AppError> {
    let layouts = sqlx::query_as::<_, PrintLayout>(
        "SELECT * FROM print_layouts WHERE is_active = true ORDER BY name",
    )
    .fetch_all(pool)
    .await?;
    Ok(layouts)
}

pub async fn get_layout(pool: &PgPool, layout_code: &str) -> Result<PrintLayout, AppError> {
    sqlx::query_as::<_, PrintLayout>(
        "SELECT * FROM print_layouts WHERE layout_code = $1 AND is_active = true",
    )
    .bind(layout_code)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Print layout not found".to_string()))
}

pub async fn save_layout(pool: &PgPool, input: CreatePrintLayout) -> Result<PrintLayout, AppError> {
    let layout = sqlx::query_as::<_, PrintLayout>(
        "INSERT INTO print_layouts (layout_code, name, description, operation_id, template_html, paper_size, orientation, header_html, footer_html) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         ON CONFLICT (layout_code) DO UPDATE SET \
         name = $2, description = $3, operation_id = $4, template_html = $5, \
         paper_size = $6, orientation = $7, header_html = $8, footer_html = $9, \
         updated_at = NOW() \
         RETURNING *",
    )
    .bind(&input.layout_code)
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.operation_id)
    .bind(&input.template_html)
    .bind(input.paper_size.as_deref().unwrap_or("A4"))
    .bind(input.orientation.as_deref().unwrap_or("portrait"))
    .bind(&input.header_html)
    .bind(&input.footer_html)
    .fetch_one(pool)
    .await?;
    Ok(layout)
}

/// Render a print layout with data
pub fn render_layout(layout: &PrintLayout, data: &serde_json::Value) -> String {
    let mut html = layout.template_html.clone();

    // Simple variable substitution
    if let serde_json::Value::Object(map) = data {
        for (key, val) in map {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = match val {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Null => String::new(),
                other => other.to_string(),
            };
            html = html.replace(&placeholder, &value_str);
        }
    }

    // Wrap in full HTML document with print styles
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>{title}</title>
<style>
@page {{ size: {paper_size} {orientation}; margin: {mt}mm {mr}mm {mb}mm {ml}mm; }}
body {{ font-family: 'Noto Sans TC', 'Helvetica Neue', Arial, sans-serif; font-size: 12px; color: #333; }}
table {{ border-collapse: collapse; width: 100%; }}
th, td {{ border: 1px solid #ddd; padding: 6px 8px; text-align: left; }}
th {{ background: #f5f5f5; font-weight: 600; }}
.header {{ border-bottom: 2px solid #333; padding-bottom: 8px; margin-bottom: 16px; }}
.footer {{ border-top: 1px solid #ccc; padding-top: 8px; margin-top: 16px; font-size: 10px; color: #666; }}
@media print {{ .no-print {{ display: none; }} }}
</style>
</head>
<body>
{header}
{body}
{footer}
</body>
</html>"#,
        title = layout.name,
        paper_size = layout.paper_size,
        orientation = layout.orientation,
        mt = layout
            .margins
            .get("top")
            .and_then(|v| v.as_f64())
            .unwrap_or(20.0),
        mr = layout
            .margins
            .get("right")
            .and_then(|v| v.as_f64())
            .unwrap_or(15.0),
        mb = layout
            .margins
            .get("bottom")
            .and_then(|v| v.as_f64())
            .unwrap_or(20.0),
        ml = layout
            .margins
            .get("left")
            .and_then(|v| v.as_f64())
            .unwrap_or(15.0),
        header = layout
            .header_html
            .as_deref()
            .map(|h| format!("<div class=\"header\">{}</div>", h))
            .unwrap_or_default(),
        body = html,
        footer = layout
            .footer_html
            .as_deref()
            .map(|f| format!("<div class=\"footer\">{}</div>", f))
            .unwrap_or_default(),
    )
}
