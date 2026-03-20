use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApprovalMatrix {
    pub id: Uuid,
    pub name: String,
    pub operation_id: Option<Uuid>,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApprovalLevel {
    pub id: Uuid,
    pub matrix_id: Uuid,
    pub level_order: i32,
    pub name: String,
    pub condition_field: Option<String>,
    pub condition_operator: String,
    pub condition_value: Option<rust_decimal::Decimal>,
    pub approver_type: String,
    pub approver_role: Option<String>,
    pub approver_user_id: Option<Uuid>,
    pub is_parallel: bool,
    pub sla_hours: Option<i32>,
    pub auto_escalate: bool,
    pub escalate_to_role: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApprovalInstance {
    pub id: Uuid,
    pub matrix_id: Uuid,
    pub operation_id: Uuid,
    pub record_id: Uuid,
    pub status: String,
    pub current_level: i32,
    pub submitted_by: Option<Uuid>,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApprovalAction {
    pub id: Uuid,
    pub instance_id: Uuid,
    pub level_id: Uuid,
    pub action: String,
    pub acted_by: Uuid,
    pub comment: Option<String>,
    pub acted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApprovalMatrix {
    pub name: String,
    pub operation_id: Option<Uuid>,
    pub description: Option<String>,
    pub levels: Vec<CreateApprovalLevel>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApprovalLevel {
    pub level_order: i32,
    pub name: String,
    pub condition_field: Option<String>,
    pub condition_operator: Option<String>,
    pub condition_value: Option<rust_decimal::Decimal>,
    pub approver_type: String,
    pub approver_role: Option<String>,
    pub approver_user_id: Option<Uuid>,
    pub is_parallel: Option<bool>,
    pub sla_hours: Option<i32>,
    pub auto_escalate: Option<bool>,
    pub escalate_to_role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApprovalMatrixWithLevels {
    #[serde(flatten)]
    pub matrix: ApprovalMatrix,
    pub levels: Vec<ApprovalLevel>,
}

pub async fn list_matrices(
    pool: &PgPool,
    operation_id: Option<Uuid>,
) -> Result<Vec<ApprovalMatrix>, AppError> {
    let matrices = match operation_id {
        Some(op_id) => {
            sqlx::query_as::<_, ApprovalMatrix>(
                "SELECT * FROM approval_matrices WHERE operation_id = $1 ORDER BY name",
            )
            .bind(op_id)
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, ApprovalMatrix>("SELECT * FROM approval_matrices ORDER BY name")
                .fetch_all(pool)
                .await?
        }
    };
    Ok(matrices)
}

pub async fn get_matrix_with_levels(
    pool: &PgPool,
    id: Uuid,
) -> Result<ApprovalMatrixWithLevels, AppError> {
    let matrix =
        sqlx::query_as::<_, ApprovalMatrix>("SELECT * FROM approval_matrices WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Approval matrix not found".to_string()))?;
    let levels = sqlx::query_as::<_, ApprovalLevel>(
        "SELECT * FROM approval_levels WHERE matrix_id = $1 ORDER BY level_order",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    Ok(ApprovalMatrixWithLevels { matrix, levels })
}

pub async fn create_matrix(
    pool: &PgPool,
    input: CreateApprovalMatrix,
) -> Result<ApprovalMatrixWithLevels, AppError> {
    let mut tx = pool.begin().await?;
    let matrix = sqlx::query_as::<_, ApprovalMatrix>(
        "INSERT INTO approval_matrices (name, operation_id, description) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&input.name)
    .bind(input.operation_id)
    .bind(&input.description)
    .fetch_one(&mut *tx)
    .await?;

    let mut levels = Vec::new();
    for level in &input.levels {
        let l = sqlx::query_as::<_, ApprovalLevel>(
            "INSERT INTO approval_levels (matrix_id, level_order, name, condition_field, condition_operator, condition_value, approver_type, approver_role, approver_user_id, is_parallel, sla_hours, auto_escalate, escalate_to_role) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) RETURNING *",
        )
        .bind(matrix.id)
        .bind(level.level_order)
        .bind(&level.name)
        .bind(&level.condition_field)
        .bind(level.condition_operator.as_deref().unwrap_or("gte"))
        .bind(level.condition_value)
        .bind(&level.approver_type)
        .bind(&level.approver_role)
        .bind(level.approver_user_id)
        .bind(level.is_parallel.unwrap_or(false))
        .bind(level.sla_hours.unwrap_or(24))
        .bind(level.auto_escalate.unwrap_or(false))
        .bind(&level.escalate_to_role)
        .fetch_one(&mut *tx)
        .await?;
        levels.push(l);
    }
    tx.commit().await?;
    Ok(ApprovalMatrixWithLevels { matrix, levels })
}

pub async fn delete_matrix(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM approval_matrices WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Submit a record for approval
pub async fn submit_for_approval(
    pool: &PgPool,
    matrix_id: Uuid,
    operation_id: Uuid,
    record_id: Uuid,
    submitted_by: Uuid,
) -> Result<ApprovalInstance, AppError> {
    let instance = sqlx::query_as::<_, ApprovalInstance>(
        "INSERT INTO approval_instances (matrix_id, operation_id, record_id, submitted_by) VALUES ($1,$2,$3,$4) RETURNING *",
    )
    .bind(matrix_id)
    .bind(operation_id)
    .bind(record_id)
    .bind(submitted_by)
    .fetch_one(pool)
    .await?;
    Ok(instance)
}

/// Approve or reject at current level
pub async fn process_approval(
    pool: &PgPool,
    instance_id: Uuid,
    action: &str,
    acted_by: Uuid,
    comment: Option<&str>,
) -> Result<ApprovalInstance, AppError> {
    let instance =
        sqlx::query_as::<_, ApprovalInstance>("SELECT * FROM approval_instances WHERE id = $1")
            .bind(instance_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Approval instance not found".to_string()))?;

    if instance.status != "PENDING" {
        return Err(AppError::Validation("Approval is not pending".to_string()));
    }

    let levels = sqlx::query_as::<_, ApprovalLevel>(
        "SELECT * FROM approval_levels WHERE matrix_id = $1 ORDER BY level_order",
    )
    .bind(instance.matrix_id)
    .fetch_all(pool)
    .await?;

    let current_level = levels
        .iter()
        .find(|l| l.level_order == instance.current_level)
        .ok_or_else(|| AppError::Internal("Current level not found".to_string()))?;

    // Record the action
    sqlx::query("INSERT INTO approval_actions (instance_id, level_id, action, acted_by, comment) VALUES ($1,$2,$3,$4,$5)")
        .bind(instance_id)
        .bind(current_level.id)
        .bind(action)
        .bind(acted_by)
        .bind(comment)
        .execute(pool)
        .await?;

    if action == "REJECT" {
        let updated = sqlx::query_as::<_, ApprovalInstance>(
            "UPDATE approval_instances SET status = 'REJECTED', completed_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(instance_id)
        .fetch_one(pool)
        .await?;
        return Ok(updated);
    }

    // Check if there are more levels
    let next_level = levels
        .iter()
        .find(|l| l.level_order > instance.current_level);
    let updated = if next_level.is_some() {
        sqlx::query_as::<_, ApprovalInstance>(
            "UPDATE approval_instances SET current_level = current_level + 1 WHERE id = $1 RETURNING *",
        )
        .bind(instance_id)
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_as::<_, ApprovalInstance>(
            "UPDATE approval_instances SET status = 'APPROVED', completed_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(instance_id)
        .fetch_one(pool)
        .await?
    };
    Ok(updated)
}

pub async fn get_instance_with_actions(
    pool: &PgPool,
    instance_id: Uuid,
) -> Result<(ApprovalInstance, Vec<ApprovalAction>), AppError> {
    let instance =
        sqlx::query_as::<_, ApprovalInstance>("SELECT * FROM approval_instances WHERE id = $1")
            .bind(instance_id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Not found".to_string()))?;
    let actions = sqlx::query_as::<_, ApprovalAction>(
        "SELECT * FROM approval_actions WHERE instance_id = $1 ORDER BY acted_at",
    )
    .bind(instance_id)
    .fetch_all(pool)
    .await?;
    Ok((instance, actions))
}

pub async fn list_pending_approvals(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<ApprovalInstance>, AppError> {
    let instances = sqlx::query_as::<_, ApprovalInstance>(
        "SELECT ai.* FROM approval_instances ai \
         JOIN approval_levels al ON al.matrix_id = ai.matrix_id AND al.level_order = ai.current_level \
         WHERE ai.status = 'PENDING' \
         AND (al.approver_user_id = $1 OR al.approver_role IN (\
             SELECT upr.role_id::text FROM lc_user_platform_roles upr WHERE upr.user_id = $1\
         )) ORDER BY ai.submitted_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(instances)
}
