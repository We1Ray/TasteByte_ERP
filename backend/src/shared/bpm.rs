use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkflowDefinition {
    pub id: Uuid,
    pub name: String,
    pub operation_id: Option<Uuid>,
    pub trigger_event: String,
    pub definition: serde_json::Value,
    pub is_active: bool,
    pub version: i32,
    pub created_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkflowInstance {
    pub id: Uuid,
    pub definition_id: Uuid,
    pub record_id: Uuid,
    pub operation_id: Uuid,
    pub current_node: Option<String>,
    pub status: String,
    pub context: serde_json::Value,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct WorkflowExecLog {
    pub id: Uuid,
    pub instance_id: Uuid,
    pub node_id: String,
    pub node_type: String,
    pub status: String,
    pub input_data: Option<serde_json::Value>,
    pub output_data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkflowDefinition {
    pub name: String,
    pub operation_id: Option<Uuid>,
    pub trigger_event: Option<String>,
    pub definition: serde_json::Value,
}

pub async fn list_definitions(
    pool: &PgPool,
    operation_id: Option<Uuid>,
) -> Result<Vec<WorkflowDefinition>, AppError> {
    match operation_id {
        Some(op_id) => Ok(sqlx::query_as::<_, WorkflowDefinition>(
            "SELECT * FROM workflow_definitions WHERE operation_id = $1 ORDER BY name",
        )
        .bind(op_id)
        .fetch_all(pool)
        .await?),
        None => Ok(sqlx::query_as::<_, WorkflowDefinition>(
            "SELECT * FROM workflow_definitions ORDER BY name",
        )
        .fetch_all(pool)
        .await?),
    }
}

pub async fn create_definition(
    pool: &PgPool,
    input: CreateWorkflowDefinition,
    created_by: Uuid,
) -> Result<WorkflowDefinition, AppError> {
    let def = sqlx::query_as::<_, WorkflowDefinition>(
        "INSERT INTO workflow_definitions (name, operation_id, trigger_event, definition, created_by) VALUES ($1,$2,$3,$4,$5) RETURNING *",
    )
    .bind(&input.name)
    .bind(input.operation_id)
    .bind(input.trigger_event.as_deref().unwrap_or("ON_CREATE"))
    .bind(&input.definition)
    .bind(created_by)
    .fetch_one(pool)
    .await?;
    Ok(def)
}

pub async fn update_definition(
    pool: &PgPool,
    id: Uuid,
    input: CreateWorkflowDefinition,
) -> Result<WorkflowDefinition, AppError> {
    let def = sqlx::query_as::<_, WorkflowDefinition>(
        "UPDATE workflow_definitions SET name=$2, operation_id=$3, trigger_event=$4, definition=$5, version=version+1, updated_at=NOW() WHERE id=$1 RETURNING *",
    )
    .bind(id)
    .bind(&input.name)
    .bind(input.operation_id)
    .bind(input.trigger_event.as_deref().unwrap_or("ON_CREATE"))
    .bind(&input.definition)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Workflow not found".to_string()))?;
    Ok(def)
}

pub async fn delete_definition(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM workflow_definitions WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Start a workflow instance for a record
pub async fn start_instance(
    pool: &PgPool,
    definition_id: Uuid,
    operation_id: Uuid,
    record_id: Uuid,
) -> Result<WorkflowInstance, AppError> {
    let def = sqlx::query_as::<_, WorkflowDefinition>(
        "SELECT * FROM workflow_definitions WHERE id = $1 AND is_active = true",
    )
    .bind(definition_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Workflow definition not found".to_string()))?;

    // Find start node
    let start_node = def
        .definition
        .get("start_node")
        .and_then(|v| v.as_str())
        .unwrap_or("start");

    let instance = sqlx::query_as::<_, WorkflowInstance>(
        "INSERT INTO workflow_instances (definition_id, record_id, operation_id, current_node, context) VALUES ($1,$2,$3,$4,$5) RETURNING *",
    )
    .bind(definition_id)
    .bind(record_id)
    .bind(operation_id)
    .bind(start_node)
    .bind(serde_json::json!({}))
    .fetch_one(pool)
    .await?;

    // Log start
    sqlx::query("INSERT INTO workflow_execution_log (instance_id, node_id, node_type, status) VALUES ($1,$2,'START','COMPLETED')")
        .bind(instance.id)
        .bind(start_node)
        .execute(pool)
        .await?;

    Ok(instance)
}

/// Advance workflow to next node
pub async fn advance_instance(
    pool: &PgPool,
    instance_id: Uuid,
    target_node: &str,
) -> Result<WorkflowInstance, AppError> {
    let instance = sqlx::query_as::<_, WorkflowInstance>(
        "UPDATE workflow_instances SET current_node = $2 WHERE id = $1 RETURNING *",
    )
    .bind(instance_id)
    .bind(target_node)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Instance not found".to_string()))?;

    sqlx::query("INSERT INTO workflow_execution_log (instance_id, node_id, node_type, status) VALUES ($1,$2,'NODE','COMPLETED')")
        .bind(instance_id)
        .bind(target_node)
        .execute(pool)
        .await?;

    Ok(instance)
}

/// Complete a workflow instance
#[allow(dead_code)]
pub async fn complete_instance(
    pool: &PgPool,
    instance_id: Uuid,
) -> Result<WorkflowInstance, AppError> {
    let instance = sqlx::query_as::<_, WorkflowInstance>(
        "UPDATE workflow_instances SET status = 'COMPLETED', completed_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(instance_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Instance not found".to_string()))?;
    Ok(instance)
}

pub async fn list_instances(
    pool: &PgPool,
    operation_id: Uuid,
) -> Result<Vec<WorkflowInstance>, AppError> {
    Ok(sqlx::query_as::<_, WorkflowInstance>(
        "SELECT * FROM workflow_instances WHERE operation_id = $1 ORDER BY started_at DESC LIMIT 100",
    )
    .bind(operation_id)
    .fetch_all(pool)
    .await?)
}

pub async fn get_instance_logs(
    pool: &PgPool,
    instance_id: Uuid,
) -> Result<Vec<WorkflowExecLog>, AppError> {
    Ok(sqlx::query_as::<_, WorkflowExecLog>(
        "SELECT * FROM workflow_execution_log WHERE instance_id = $1 ORDER BY executed_at",
    )
    .bind(instance_id)
    .fetch_all(pool)
    .await?)
}
