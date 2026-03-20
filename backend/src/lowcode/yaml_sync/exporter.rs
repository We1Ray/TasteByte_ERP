use rust_decimal::prelude::ToPrimitive;
use sqlx::PgPool;

use super::schema::*;
use crate::lowcode::models::Operation;
use crate::shared::AppError;

pub async fn export_operation(pool: &PgPool, operation_code: &str) -> Result<String, AppError> {
    let op = sqlx::query_as::<_, Operation>("SELECT * FROM lc_operations WHERE operation_code = $1")
        .bind(operation_code)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    let project = sqlx::query_scalar::<_, String>(
        "SELECT project_number FROM lc_projects WHERE id = $1",
    )
    .bind(op.project_id)
    .fetch_one(pool)
    .await?;

    let form = crate::lowcode::services::form_builder::get_form(pool, op.id).await?;

    let sections: Vec<SectionDef> = form
        .sections
        .iter()
        .map(|s| SectionDef {
            title: s.section.title.clone(),
            description: s.section.description.clone(),
            columns: Some(s.section.columns),
            is_collapsible: Some(s.section.is_collapsible),
            is_default_collapsed: Some(s.section.is_default_collapsed),
            visibility_rule: s.section.visibility_rule.clone(),
            fields: s
                .fields
                .iter()
                .map(|f| FieldDef {
                    field_name: f.field.field_name.clone(),
                    label: f.field.field_label.clone(),
                    field_type: f.field.field_type.clone(),
                    required: Some(f.field.is_required),
                    unique: Some(f.field.is_unique),
                    searchable: Some(f.field.is_searchable),
                    column_span: Some(f.field.column_span),
                    placeholder: f.field.placeholder.clone(),
                    help_text: f.field.help_text.clone(),
                    default_value: f.field.default_value.clone(),
                    default_value_sql: f.field.default_value_sql.clone(),
                    validation: Some(ValidationDef {
                        regex: f.field.validation_regex.clone(),
                        regex_message: f.field.validation_message.clone(),
                        min_value: f.field.min_value.as_ref().and_then(|v| v.to_f64()),
                        max_value: f.field.max_value.as_ref().and_then(|v| v.to_f64()),
                        min_length: f.field.min_length,
                        max_length: f.field.max_length,
                    }),
                    db_mapping: if f.field.db_table.is_some() || f.field.db_column.is_some() {
                        Some(DbMappingDef {
                            table: f.field.db_table.clone(),
                            column: f.field.db_column.clone(),
                        })
                    } else {
                        None
                    },
                    data_source: if f.field.data_source_sql.is_some() {
                        Some(DataSourceDef {
                            sql: f.field.data_source_sql.clone(),
                            display_column: f.field.display_column.clone(),
                            value_column: f.field.value_column.clone(),
                        })
                    } else {
                        None
                    },
                    depends_on: None,
                    visibility_rule: f.field.visibility_rule.clone(),
                    config: if f.field.field_config != serde_json::json!({}) {
                        Some(f.field.field_config.clone())
                    } else {
                        None
                    },
                    options: if f.options.is_empty() {
                        None
                    } else {
                        Some(
                            f.options
                                .iter()
                                .map(|o| FieldOptionDef {
                                    label: o.option_label.clone(),
                                    value: o.option_value.clone(),
                                    is_default: Some(o.is_default),
                                    is_active: Some(o.is_active),
                                })
                                .collect(),
                        )
                    },
                })
                .collect(),
        })
        .collect();

    let op_def = OperationDef {
        operation_code: op.operation_code,
        name: op.name,
        description: op.description,
        module: op.module.unwrap_or_default(),
        operation_type: Some(op.operation_type),
        project_code: project,
        is_published: Some(op.is_published),
        version: Some(op.version),
        sidebar: Some(SidebarDef {
            icon: op.sidebar_icon,
            sort_order: Some(op.sidebar_sort_order),
        }),
        form: Some(FormDef {
            layout: Some(form.form.layout_config),
            settings: Some(form.form.form_settings),
            sections,
        }),
        cross_field_rules: None,
        calculation_formulas: None,
        approval: None,
        output_rules: None,
        buttons: None,
        form_variants: None,
    };

    serde_yaml::to_string(&op_def)
        .map_err(|e| AppError::Internal(format!("YAML export error: {}", e)))
}
