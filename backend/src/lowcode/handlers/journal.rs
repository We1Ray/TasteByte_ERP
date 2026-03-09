use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::form_builder;
use crate::lowcode::services::permission_resolver::{PlatformDeveloper, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

pub async fn list_journal(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(operation_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<DevJournalEntry>>>, AppError> {
    let entries = sqlx::query_as::<_, DevJournalEntry>(
        "SELECT * FROM lc_dev_journal WHERE operation_id = $1 ORDER BY created_at DESC LIMIT 100",
    )
    .bind(operation_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(entries)))
}

pub async fn rollback_to_version(
    State(state): State<AppState>,
    guard: RequirePlatformRole<PlatformDeveloper>,
    Path((operation_id, version)): Path<(Uuid, i32)>,
) -> Result<Json<ApiResponse<DevJournalEntry>>, AppError> {
    // Find the journal entry with the target version's snapshot
    let target = sqlx::query_as::<_, DevJournalEntry>(
        "SELECT * FROM lc_dev_journal WHERE operation_id = $1 AND version = $2 AND form_snapshot IS NOT NULL ORDER BY created_at DESC LIMIT 1",
    )
    .bind(operation_id)
    .bind(version)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| {
        AppError::NotFound(format!(
            "No snapshot found for version {version}"
        ))
    })?;

    let snapshot = target
        .form_snapshot
        .as_ref()
        .ok_or_else(|| AppError::Internal("Snapshot is null".to_string()))?;

    // Try to deserialize and restore the form from snapshot
    let form_response: FormResponse = serde_json::from_value(snapshot.clone())
        .map_err(|e| AppError::Internal(format!("Failed to deserialize snapshot: {e}")))?;

    // Rebuild form from snapshot using save_form
    let sections: Vec<SaveSectionInput> = form_response
        .sections
        .into_iter()
        .map(|s| SaveSectionInput {
            id: Some(s.section.id),
            title: s.section.title,
            description: s.section.description,
            columns: Some(s.section.columns),
            sort_order: s.section.sort_order,
            is_collapsible: Some(s.section.is_collapsible),
            is_default_collapsed: Some(s.section.is_default_collapsed),
            visibility_rule: s.section.visibility_rule,
            fields: s
                .fields
                .into_iter()
                .map(|f| SaveFieldInput {
                    id: Some(f.field.id),
                    field_name: f.field.field_name,
                    field_label: f.field.field_label,
                    field_type: f.field.field_type,
                    db_table: f.field.db_table,
                    db_column: f.field.db_column,
                    is_required: Some(f.field.is_required),
                    is_unique: Some(f.field.is_unique),
                    is_searchable: Some(f.field.is_searchable),
                    default_value: f.field.default_value,
                    default_value_sql: f.field.default_value_sql,
                    placeholder: f.field.placeholder,
                    help_text: f.field.help_text,
                    validation_regex: f.field.validation_regex,
                    validation_message: f.field.validation_message,
                    min_value: f.field.min_value,
                    max_value: f.field.max_value,
                    min_length: f.field.min_length,
                    max_length: f.field.max_length,
                    depends_on: f.field.depends_on,
                    data_source_sql: f.field.data_source_sql,
                    display_column: f.field.display_column,
                    value_column: f.field.value_column,
                    visibility_rule: f.field.visibility_rule,
                    field_config: Some(f.field.field_config),
                    sort_order: f.field.sort_order,
                    column_span: Some(f.field.column_span),
                    options: Some(
                        f.options
                            .into_iter()
                            .map(|o| SaveFieldOptionInput {
                                id: Some(o.id),
                                option_label: o.option_label,
                                option_value: o.option_value,
                                sort_order: o.sort_order,
                                is_default: Some(o.is_default),
                                is_active: Some(o.is_active),
                            })
                            .collect(),
                    ),
                })
                .collect(),
        })
        .collect();

    let save_request = SaveFormRequest {
        layout_config: Some(form_response.form.layout_config),
        form_settings: Some(form_response.form.form_settings),
        sections,
    };

    form_builder::save_form(&state.pool, operation_id, save_request).await?;

    // Record the rollback in journal
    let entry = sqlx::query_as::<_, DevJournalEntry>(
        "INSERT INTO lc_dev_journal (operation_id, changed_by, change_type, diff_summary, form_snapshot, version) VALUES ($1, $2, 'ROLLBACK', $3, $4, $5) RETURNING *",
    )
    .bind(operation_id)
    .bind(guard.claims.sub)
    .bind(format!("Rolled back to version {version}"))
    .bind(snapshot)
    .bind(version)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::with_message(
        entry,
        format!("Rolled back to version {version}"),
    )))
}
