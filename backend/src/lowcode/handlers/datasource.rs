use axum::extract::{Path, State};
use axum::Json;
use validator::Validate;

use crate::lowcode::models::*;
use crate::lowcode::services::permission_resolver::{PlatformDeveloper, RequirePlatformRole};
use crate::lowcode::services::sql_engine;
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

pub async fn execute_query(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Json(input): Json<DataSourceQuery>,
) -> Result<Json<ApiResponse<DataSourceResult>>, AppError> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let (columns, rows) =
        sql_engine::execute_safe_query(&state.pool, &input.sql, &input.params).await?;

    let row_count = rows.len();
    Ok(Json(ApiResponse::success(DataSourceResult {
        columns,
        rows,
        row_count,
    })))
}

pub async fn list_tables(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
) -> Result<Json<ApiResponse<Vec<TableInfo>>>, AppError> {
    let tables = sqlx::query_as::<_, TableInfo>(
        "SELECT table_name, table_schema FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE' ORDER BY table_name",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(tables)))
}

pub async fn validate_sql(
    State(_state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Json(input): Json<DataSourceQuery>,
) -> Result<Json<ApiResponse<SqlValidation>>, AppError> {
    match sql_engine::validate_sql(&input.sql) {
        Ok(()) => Ok(Json(ApiResponse::success(SqlValidation {
            valid: true,
            error: None,
        }))),
        Err(AppError::Validation(msg)) => Ok(Json(ApiResponse::success(SqlValidation {
            valid: false,
            error: Some(msg),
        }))),
        Err(e) => Err(e),
    }
}

pub async fn list_columns(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(table_name): Path<String>,
) -> Result<Json<ApiResponse<Vec<ColumnInfoResponse>>>, AppError> {
    // Get columns from information_schema
    let columns = sqlx::query_as::<_, ColumnInfo>(
        "SELECT column_name, data_type, is_nullable, column_default, ordinal_position \
         FROM information_schema.columns \
         WHERE table_schema = 'public' AND table_name = $1 \
         ORDER BY ordinal_position",
    )
    .bind(&table_name)
    .fetch_all(&state.pool)
    .await?;

    if columns.is_empty() {
        return Err(AppError::NotFound(format!(
            "Table '{}' not found",
            table_name
        )));
    }

    // Get primary key columns
    let pk_columns: Vec<String> = sqlx::query_scalar(
        "SELECT kcu.column_name \
         FROM information_schema.table_constraints tc \
         JOIN information_schema.key_column_usage kcu \
           ON tc.constraint_name = kcu.constraint_name \
           AND tc.table_schema = kcu.table_schema \
         WHERE tc.constraint_type = 'PRIMARY KEY' \
           AND tc.table_schema = 'public' \
           AND tc.table_name = $1",
    )
    .bind(&table_name)
    .fetch_all(&state.pool)
    .await?;

    let result: Vec<ColumnInfoResponse> = columns
        .into_iter()
        .map(|c| ColumnInfoResponse {
            is_nullable: c.is_nullable == "YES",
            is_primary_key: pk_columns.contains(&c.column_name),
            column_name: c.column_name,
            data_type: c.data_type,
            column_default: c.column_default,
            ordinal_position: c.ordinal_position,
        })
        .collect();

    Ok(Json(ApiResponse::success(result)))
}
