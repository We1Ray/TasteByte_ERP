use sqlx::PgPool;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::shared::AppError;

/// Get full form definition with sections, fields, and options
pub async fn get_form(pool: &PgPool, operation_id: Uuid) -> Result<FormResponse, AppError> {
    // Get or create form definition
    let form = get_or_create_form_definition(pool, operation_id).await?;

    let sections = sqlx::query_as::<_, FormSection>(
        "SELECT * FROM lc_form_sections WHERE form_id = $1 ORDER BY sort_order",
    )
    .bind(form.id)
    .fetch_all(pool)
    .await?;

    let mut sections_with_fields = Vec::with_capacity(sections.len());
    for section in sections {
        let fields = sqlx::query_as::<_, FieldDefinition>(
            "SELECT * FROM lc_field_definitions WHERE section_id = $1 ORDER BY sort_order",
        )
        .bind(section.id)
        .fetch_all(pool)
        .await?;

        let mut fields_with_options = Vec::with_capacity(fields.len());
        for field in fields {
            let options = sqlx::query_as::<_, FieldOption>(
                "SELECT * FROM lc_field_options WHERE field_id = $1 ORDER BY sort_order",
            )
            .bind(field.id)
            .fetch_all(pool)
            .await?;
            fields_with_options.push(FieldWithOptions { field, options });
        }

        sections_with_fields.push(SectionWithFields {
            section,
            fields: fields_with_options,
        });
    }

    Ok(FormResponse {
        form,
        sections: sections_with_fields,
    })
}

/// Bulk save form: replace all sections and fields in a single transaction
pub async fn save_form(
    pool: &PgPool,
    operation_id: Uuid,
    input: SaveFormRequest,
) -> Result<FormResponse, AppError> {
    let mut tx = pool.begin().await?;

    // Get or create form definition
    let form = get_or_create_form_definition_tx(&mut tx, operation_id).await?;

    // Update form-level settings
    let layout = input.layout_config.unwrap_or(form.layout_config.clone());
    let settings = input.form_settings.unwrap_or(form.form_settings.clone());

    sqlx::query(
        "UPDATE lc_form_definitions SET layout_config = $2, form_settings = $3, version = version + 1, updated_at = NOW() WHERE id = $1",
    )
    .bind(form.id)
    .bind(&layout)
    .bind(&settings)
    .execute(&mut *tx)
    .await?;

    // Delete existing sections (cascades to fields and options)
    sqlx::query("DELETE FROM lc_form_sections WHERE form_id = $1")
        .bind(form.id)
        .execute(&mut *tx)
        .await?;

    // Insert new sections with fields and options
    for section_input in &input.sections {
        let section_id = section_input.id.unwrap_or_else(Uuid::new_v4);
        sqlx::query(
            "INSERT INTO lc_form_sections (id, form_id, title, description, columns, sort_order, is_collapsible, is_default_collapsed, visibility_rule) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(section_id)
        .bind(form.id)
        .bind(&section_input.title)
        .bind(&section_input.description)
        .bind(section_input.columns.unwrap_or(2))
        .bind(section_input.sort_order)
        .bind(section_input.is_collapsible.unwrap_or(false))
        .bind(section_input.is_default_collapsed.unwrap_or(false))
        .bind(&section_input.visibility_rule)
        .execute(&mut *tx)
        .await?;

        for field_input in &section_input.fields {
            let field_id = field_input.id.unwrap_or_else(Uuid::new_v4);
            sqlx::query(
                "INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, db_table, db_column, is_required, is_unique, is_searchable, default_value, default_value_sql, placeholder, help_text, validation_regex, validation_message, min_value, max_value, min_length, max_length, depends_on, data_source_sql, display_column, value_column, visibility_rule, field_config, sort_order, column_span) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)",
            )
            .bind(field_id)
            .bind(section_id)
            .bind(&field_input.field_name)
            .bind(&field_input.field_label)
            .bind(&field_input.field_type)
            .bind(&field_input.db_table)
            .bind(&field_input.db_column)
            .bind(field_input.is_required.unwrap_or(false))
            .bind(field_input.is_unique.unwrap_or(false))
            .bind(field_input.is_searchable.unwrap_or(false))
            .bind(&field_input.default_value)
            .bind(&field_input.default_value_sql)
            .bind(&field_input.placeholder)
            .bind(&field_input.help_text)
            .bind(&field_input.validation_regex)
            .bind(&field_input.validation_message)
            .bind(field_input.min_value)
            .bind(field_input.max_value)
            .bind(field_input.min_length)
            .bind(field_input.max_length)
            .bind(field_input.depends_on)
            .bind(&field_input.data_source_sql)
            .bind(&field_input.display_column)
            .bind(&field_input.value_column)
            .bind(&field_input.visibility_rule)
            .bind(field_input.field_config.as_ref().unwrap_or(&serde_json::json!({})))
            .bind(field_input.sort_order)
            .bind(field_input.column_span.unwrap_or(1))
            .execute(&mut *tx)
            .await?;

            // Insert field options
            if let Some(options) = &field_input.options {
                for opt in options {
                    let opt_id = opt.id.unwrap_or_else(Uuid::new_v4);
                    sqlx::query(
                        "INSERT INTO lc_field_options (id, field_id, option_label, option_value, sort_order, is_default, is_active) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                    )
                    .bind(opt_id)
                    .bind(field_id)
                    .bind(&opt.option_label)
                    .bind(&opt.option_value)
                    .bind(opt.sort_order)
                    .bind(opt.is_default.unwrap_or(false))
                    .bind(opt.is_active.unwrap_or(true))
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }
    }

    tx.commit().await?;

    // Return fresh data
    get_form(pool, operation_id).await
}

async fn get_or_create_form_definition(
    pool: &PgPool,
    operation_id: Uuid,
) -> Result<FormDefinition, AppError> {
    let existing = sqlx::query_as::<_, FormDefinition>(
        "SELECT * FROM lc_form_definitions WHERE operation_id = $1",
    )
    .bind(operation_id)
    .fetch_optional(pool)
    .await?;

    match existing {
        Some(form) => Ok(form),
        None => {
            let form = sqlx::query_as::<_, FormDefinition>(
                "INSERT INTO lc_form_definitions (operation_id) VALUES ($1) RETURNING *",
            )
            .bind(operation_id)
            .fetch_one(pool)
            .await?;
            Ok(form)
        }
    }
}

async fn get_or_create_form_definition_tx(
    tx: &mut sqlx::PgConnection,
    operation_id: Uuid,
) -> Result<FormDefinition, AppError> {
    let existing = sqlx::query_as::<_, FormDefinition>(
        "SELECT * FROM lc_form_definitions WHERE operation_id = $1",
    )
    .bind(operation_id)
    .fetch_optional(&mut *tx)
    .await?;

    match existing {
        Some(form) => Ok(form),
        None => {
            let form = sqlx::query_as::<_, FormDefinition>(
                "INSERT INTO lc_form_definitions (operation_id) VALUES ($1) RETURNING *",
            )
            .bind(operation_id)
            .fetch_one(&mut *tx)
            .await?;
            Ok(form)
        }
    }
}

/// Create a snapshot of the current form definition for versioning
pub async fn create_snapshot(
    pool: &PgPool,
    operation_id: Uuid,
) -> Result<serde_json::Value, AppError> {
    let form = get_form(pool, operation_id).await?;
    let snapshot = serde_json::to_value(&form)
        .map_err(|e| AppError::Internal(format!("Snapshot serialization error: {e}")))?;

    sqlx::query(
        "UPDATE lc_form_definitions SET snapshot = $2, updated_at = NOW() WHERE operation_id = $1",
    )
    .bind(operation_id)
    .bind(&snapshot)
    .execute(pool)
    .await?;

    Ok(snapshot)
}
