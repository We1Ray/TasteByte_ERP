use axum::extract::{Path, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::shared::types::{AppState, Claims};
use crate::shared::{ApiResponse, AppError};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StatusHistoryEntry {
    pub id: Uuid,
    pub document_type: String,
    pub document_id: Uuid,
    pub from_status: Option<String>,
    pub to_status: String,
    pub changed_by: Uuid,
    pub change_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn get_status_history(
    State(state): State<AppState>,
    _claims: Claims,
    Path((doc_type, doc_id)): Path<(String, Uuid)>,
) -> Result<Json<ApiResponse<Vec<StatusHistoryEntry>>>, AppError> {
    let entries = sqlx::query_as::<_, StatusHistoryEntry>(
        "SELECT id, document_type, document_id, from_status, to_status, changed_by, change_reason, created_at \
         FROM document_status_history \
         WHERE document_type = $1 AND document_id = $2 \
         ORDER BY created_at ASC"
    )
    .bind(&doc_type)
    .bind(doc_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(entries)))
}
