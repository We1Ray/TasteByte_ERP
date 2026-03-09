use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformUser, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

/// Get the document flow chain starting from a given source document.
/// Recursively follows forward links (source -> target) and also collects
/// backward links (where this document is a target) to build the full chain.
pub async fn get_document_flow(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformUser>,
    Path((source_type, source_id)): Path<(String, Uuid)>,
) -> Result<Json<ApiResponse<Vec<DocumentFlowEntry>>>, AppError> {
    // Use a recursive CTE to follow the document flow chain in both directions
    let entries = sqlx::query_as::<_, DocumentFlowEntry>(
        "WITH RECURSIVE flow_chain AS ( \
             -- Base case: direct links FROM this document \
             SELECT id, source_type, source_id, target_type, target_id, flow_type, created_at, 1 AS depth \
             FROM lc_document_flows \
             WHERE source_type = $1 AND source_id = $2 \
             UNION \
             -- Base case: direct links TO this document (backward) \
             SELECT id, source_type, source_id, target_type, target_id, flow_type, created_at, 1 AS depth \
             FROM lc_document_flows \
             WHERE target_type = $1 AND target_id = $2 \
             UNION \
             -- Recursive: follow forward links from targets \
             SELECT df.id, df.source_type, df.source_id, df.target_type, df.target_id, df.flow_type, df.created_at, fc.depth + 1 \
             FROM lc_document_flows df \
             INNER JOIN flow_chain fc ON df.source_type = fc.target_type AND df.source_id = fc.target_id \
             WHERE fc.depth < 10 \
         ) \
         SELECT DISTINCT id, source_type, source_id, target_type, target_id, flow_type, created_at \
         FROM flow_chain \
         ORDER BY created_at",
    )
    .bind(&source_type)
    .bind(source_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(entries)))
}
