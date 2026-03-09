use serde_json::Value;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use sqlx::PgPool;
use uuid::Uuid;

use crate::lowcode::models::Operation;
use crate::lowcode::services::ai_service::ToolDefinition;
use crate::shared::AppError;

/// Execution context that accumulates proposed changes from tool calls.
/// Write-tools modify these fields; they are NOT persisted to DB until the user applies them.
pub struct ToolExecutionContext {
    pub pool: PgPool,
    pub operation_id: Uuid,
    pub proposed_form: Option<Value>,
    pub proposed_list: Option<Value>,
    pub proposed_dashboard: Option<Value>,
    pub proposed_buttons: Option<Value>,
}

impl ToolExecutionContext {
    pub fn new(pool: PgPool, operation_id: Uuid) -> Self {
        Self {
            pool,
            operation_id,
            proposed_form: None,
            proposed_list: None,
            proposed_dashboard: None,
            proposed_buttons: None,
        }
    }
}

/// Return tool definitions relevant to the given context type.
pub fn get_tool_definitions(context_type: &str) -> Vec<ToolDefinition> {
    let mut tools = vec![
        // ── READ tools ──────────────────────────────────────────────
        ToolDefinition {
            name: "list_tables".into(),
            description: "List all public tables in the database.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        ToolDefinition {
            name: "get_table_schema".into(),
            description: "Get column definitions for a specific table.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "table_name": { "type": "string", "description": "The table name to inspect" }
                },
                "required": ["table_name"]
            }),
        },
        ToolDefinition {
            name: "validate_sql".into(),
            description: "Validate a SQL SELECT statement for correctness.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "sql": { "type": "string", "description": "SQL statement to validate" }
                },
                "required": ["sql"]
            }),
        },
    ];

    // Context-specific read tools
    match context_type {
        "form" => {
            tools.push(ToolDefinition {
                name: "get_current_form".into(),
                description: "Get the current form definition for this operation.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            });
            // Form write tools
            tools.push(ToolDefinition {
                name: "set_form_definition".into(),
                description: "Set the complete proposed form definition. Use this when you want to replace the entire form.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "form": { "type": "object", "description": "Complete form definition with sections and fields" }
                    },
                    "required": ["form"]
                }),
            });
            tools.push(ToolDefinition {
                name: "add_section".into(),
                description: "Add a new section to the proposed form.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "title": { "type": "string" },
                        "description": { "type": "string" },
                        "columns": { "type": "integer", "default": 2 },
                        "sort_order": { "type": "integer" },
                        "fields": {
                            "type": "array",
                            "items": { "type": "object" },
                            "description": "Fields to include in this section"
                        }
                    },
                    "required": ["title", "sort_order"]
                }),
            });
            tools.push(ToolDefinition {
                name: "add_field".into(),
                description: "Add a field to a section in the proposed form. Specify the section by title or index.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "section_title": { "type": "string", "description": "Title of the section to add the field to" },
                        "section_index": { "type": "integer", "description": "Index of the section (0-based), used if section_title not found" },
                        "field": {
                            "type": "object",
                            "description": "Field definition",
                            "properties": {
                                "field_name": { "type": "string" },
                                "field_label": { "type": "string" },
                                "field_type": { "type": "string" },
                                "db_column": { "type": "string" },
                                "is_required": { "type": "boolean" },
                                "placeholder": { "type": "string" },
                                "help_text": { "type": "string" },
                                "sort_order": { "type": "integer" },
                                "column_span": { "type": "integer" },
                                "options": { "type": "array", "items": { "type": "object" } }
                            },
                            "required": ["field_name", "field_label", "field_type"]
                        }
                    },
                    "required": ["field"]
                }),
            });
            tools.push(ToolDefinition {
                name: "update_field".into(),
                description: "Update properties of an existing field in the proposed form.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "field_name": { "type": "string", "description": "Name of the field to update" },
                        "updates": { "type": "object", "description": "Key-value pairs of properties to update" }
                    },
                    "required": ["field_name", "updates"]
                }),
            });
            tools.push(ToolDefinition {
                name: "remove_field".into(),
                description: "Remove a field from the proposed form by field_name.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "field_name": { "type": "string", "description": "Name of the field to remove" }
                    },
                    "required": ["field_name"]
                }),
            });
        }
        "list" => {
            tools.push(ToolDefinition {
                name: "get_current_list".into(),
                description: "Get the current list definition for this operation.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            });
            tools.push(ToolDefinition {
                name: "set_list_definition".into(),
                description: "Set the complete proposed list definition.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "list": { "type": "object", "description": "Complete list definition with columns and actions" }
                    },
                    "required": ["list"]
                }),
            });
        }
        "dashboard" => {
            tools.push(ToolDefinition {
                name: "get_current_dashboard".into(),
                description: "Get the current dashboard definition for this operation.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            });
            tools.push(ToolDefinition {
                name: "set_dashboard_definition".into(),
                description: "Set the complete proposed dashboard definition.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "dashboard": { "type": "object", "description": "Complete dashboard definition with widgets" }
                    },
                    "required": ["dashboard"]
                }),
            });
        }
        _ => {}
    }

    // Buttons tool is always available
    tools.push(ToolDefinition {
        name: "set_operation_buttons".into(),
        description: "Set the proposed operation buttons (toolbar actions).".into(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "buttons": {
                    "type": "array",
                    "items": { "type": "object" },
                    "description": "Array of button definitions"
                }
            },
            "required": ["buttons"]
        }),
    });

    tools
}

/// Execute a tool by name. Returns the tool result as JSON Value.
pub async fn execute_tool(
    ctx: &mut ToolExecutionContext,
    name: &str,
    input: &Value,
    pool: &PgPool,
) -> Result<Value, AppError> {
    match name {
        // ── READ tools ──────────────────────────────────────────
        "list_tables" => {
            let rows: Vec<(String,)> = sqlx::query_as(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' ORDER BY table_name",
            )
            .fetch_all(pool)
            .await?;
            let tables: Vec<String> = rows.into_iter().map(|(n,)| n).collect();
            Ok(serde_json::json!({ "tables": tables }))
        }

        "get_table_schema" => {
            let table_name = input["table_name"]
                .as_str()
                .ok_or_else(|| AppError::Validation("table_name is required".into()))?;
            let rows: Vec<(String, String, String, Option<String>)> = sqlx::query_as(
                "SELECT column_name, data_type, is_nullable, column_default \
                 FROM information_schema.columns \
                 WHERE table_name = $1 \
                 ORDER BY ordinal_position",
            )
            .bind(table_name)
            .fetch_all(pool)
            .await?;

            let columns: Vec<Value> = rows
                .into_iter()
                .map(|(name, dtype, nullable, default)| {
                    serde_json::json!({
                        "column_name": name,
                        "data_type": dtype,
                        "is_nullable": nullable == "YES",
                        "column_default": default,
                    })
                })
                .collect();
            Ok(serde_json::json!({ "table_name": table_name, "columns": columns }))
        }

        "validate_sql" => {
            let sql = input["sql"]
                .as_str()
                .ok_or_else(|| AppError::Validation("sql is required".into()))?;
            let dialect = PostgreSqlDialect {};
            match Parser::parse_sql(&dialect, sql) {
                Ok(stmts) => {
                    let all_select = stmts.iter().all(|s| {
                        matches!(s, sqlparser::ast::Statement::Query(_))
                    });
                    if all_select {
                        Ok(serde_json::json!({ "valid": true }))
                    } else {
                        Ok(serde_json::json!({ "valid": false, "error": "Only SELECT statements are allowed" }))
                    }
                }
                Err(e) => Ok(serde_json::json!({ "valid": false, "error": e.to_string() })),
            }
        }

        "get_current_form" => {
            let form = crate::lowcode::services::form_builder::get_form(pool, ctx.operation_id).await?;
            let val = serde_json::to_value(&form)
                .map_err(|e| AppError::Internal(format!("Serialization error: {e}")))?;
            Ok(val)
        }

        "get_current_list" => {
            let list: Option<crate::lowcode::models::ListDefinitionRow> = sqlx::query_as(
                "SELECT * FROM lc_list_definitions WHERE operation_id = $1",
            )
            .bind(ctx.operation_id)
            .fetch_optional(pool)
            .await?;

            if let Some(list) = list {
                let columns: Vec<crate::lowcode::models::ListColumnRow> = sqlx::query_as(
                    "SELECT * FROM lc_list_columns WHERE list_id = $1 ORDER BY sort_order",
                )
                .bind(list.id)
                .fetch_all(pool)
                .await?;
                let actions: Vec<crate::lowcode::models::ListActionRow> = sqlx::query_as(
                    "SELECT * FROM lc_list_actions WHERE list_id = $1 ORDER BY sort_order",
                )
                .bind(list.id)
                .fetch_all(pool)
                .await?;
                let resp = crate::lowcode::models::ListResponse {
                    list,
                    columns,
                    actions,
                };
                Ok(serde_json::to_value(&resp).unwrap_or_default())
            } else {
                Ok(Value::Null)
            }
        }

        "get_current_dashboard" => {
            let dash: Option<crate::lowcode::models::DashboardDefinitionRow> = sqlx::query_as(
                "SELECT * FROM lc_dashboard_definitions WHERE operation_id = $1",
            )
            .bind(ctx.operation_id)
            .fetch_optional(pool)
            .await?;

            if let Some(dash) = dash {
                let widgets: Vec<crate::lowcode::models::DashboardWidgetRow> = sqlx::query_as(
                    "SELECT * FROM lc_dashboard_widgets WHERE dashboard_id = $1 ORDER BY sort_order",
                )
                .bind(dash.id)
                .fetch_all(pool)
                .await?;
                let resp = crate::lowcode::models::DashboardResponse {
                    dashboard: dash,
                    widgets,
                };
                Ok(serde_json::to_value(&resp).unwrap_or_default())
            } else {
                Ok(Value::Null)
            }
        }

        // ── WRITE tools (modify proposed_* only, not DB) ────────
        "set_form_definition" => {
            let form = input
                .get("form")
                .ok_or_else(|| AppError::Validation("form is required".into()))?;
            ctx.proposed_form = Some(form.clone());
            Ok(serde_json::json!({ "status": "ok", "message": "Form definition set as proposed change" }))
        }

        "set_list_definition" => {
            let list = input
                .get("list")
                .ok_or_else(|| AppError::Validation("list is required".into()))?;
            ctx.proposed_list = Some(list.clone());
            Ok(serde_json::json!({ "status": "ok", "message": "List definition set as proposed change" }))
        }

        "set_dashboard_definition" => {
            let dashboard = input
                .get("dashboard")
                .ok_or_else(|| AppError::Validation("dashboard is required".into()))?;
            ctx.proposed_dashboard = Some(dashboard.clone());
            Ok(serde_json::json!({ "status": "ok", "message": "Dashboard definition set as proposed change" }))
        }

        "set_operation_buttons" => {
            let buttons = input
                .get("buttons")
                .ok_or_else(|| AppError::Validation("buttons is required".into()))?;
            ctx.proposed_buttons = Some(buttons.clone());
            Ok(serde_json::json!({ "status": "ok", "message": "Buttons set as proposed change" }))
        }

        "add_section" => {
            // Initialize proposed_form from current if not set
            if ctx.proposed_form.is_none() {
                let form = crate::lowcode::services::form_builder::get_form(pool, ctx.operation_id).await?;
                ctx.proposed_form = Some(
                    serde_json::to_value(&form)
                        .map_err(|e| AppError::Internal(format!("Serialization error: {e}")))?,
                );
            }
            let proposed = ctx.proposed_form.as_mut().unwrap();

            let new_section = serde_json::json!({
                "title": input["title"].as_str().unwrap_or("New Section"),
                "description": input.get("description"),
                "columns": input["columns"].as_i64().unwrap_or(2),
                "sort_order": input["sort_order"].as_i64().unwrap_or(0),
                "fields": input.get("fields").cloned().unwrap_or(serde_json::json!([])),
            });

            if let Some(sections) = proposed.get_mut("sections") {
                if let Some(arr) = sections.as_array_mut() {
                    arr.push(new_section);
                }
            } else {
                proposed["sections"] = serde_json::json!([new_section]);
            }

            Ok(serde_json::json!({ "status": "ok", "message": "Section added to proposed form" }))
        }

        "add_field" => {
            if ctx.proposed_form.is_none() {
                let form = crate::lowcode::services::form_builder::get_form(pool, ctx.operation_id).await?;
                ctx.proposed_form = Some(
                    serde_json::to_value(&form)
                        .map_err(|e| AppError::Internal(format!("Serialization error: {e}")))?,
                );
            }
            let proposed = ctx.proposed_form.as_mut().unwrap();
            let field = input
                .get("field")
                .ok_or_else(|| AppError::Validation("field is required".into()))?
                .clone();

            let section_title = input["section_title"].as_str();
            let section_index = input["section_index"].as_u64();

            if let Some(sections) = proposed.get_mut("sections") {
                if let Some(arr) = sections.as_array_mut() {
                    let target = if let Some(title) = section_title {
                        arr.iter_mut().find(|s| s["title"].as_str() == Some(title))
                    } else if let Some(idx) = section_index {
                        arr.get_mut(idx as usize)
                    } else {
                        arr.first_mut()
                    };

                    if let Some(section) = target {
                        if let Some(fields) = section.get_mut("fields") {
                            if let Some(field_arr) = fields.as_array_mut() {
                                field_arr.push(field);
                            }
                        } else {
                            section["fields"] = serde_json::json!([field]);
                        }
                        return Ok(serde_json::json!({ "status": "ok", "message": "Field added" }));
                    }
                }
            }

            Err(AppError::Validation("Target section not found".into()))
        }

        "update_field" => {
            if ctx.proposed_form.is_none() {
                let form = crate::lowcode::services::form_builder::get_form(pool, ctx.operation_id).await?;
                ctx.proposed_form = Some(
                    serde_json::to_value(&form)
                        .map_err(|e| AppError::Internal(format!("Serialization error: {e}")))?,
                );
            }
            let proposed = ctx.proposed_form.as_mut().unwrap();
            let field_name = input["field_name"]
                .as_str()
                .ok_or_else(|| AppError::Validation("field_name is required".into()))?;
            let updates = input
                .get("updates")
                .ok_or_else(|| AppError::Validation("updates is required".into()))?;

            let mut found = false;
            if let Some(sections) = proposed.get_mut("sections") {
                if let Some(sections_arr) = sections.as_array_mut() {
                    for section in sections_arr.iter_mut() {
                        if let Some(fields) = section.get_mut("fields") {
                            if let Some(fields_arr) = fields.as_array_mut() {
                                for field in fields_arr.iter_mut() {
                                    if field["field_name"].as_str() == Some(field_name) {
                                        if let Some(obj) = updates.as_object() {
                                            if let Some(field_obj) = field.as_object_mut() {
                                                for (k, v) in obj {
                                                    field_obj.insert(k.clone(), v.clone());
                                                }
                                            }
                                        }
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }
                        if found {
                            break;
                        }
                    }
                }
            }

            if found {
                Ok(serde_json::json!({ "status": "ok", "message": format!("Field '{}' updated", field_name) }))
            } else {
                Err(AppError::Validation(format!(
                    "Field '{}' not found in proposed form",
                    field_name
                )))
            }
        }

        "remove_field" => {
            if ctx.proposed_form.is_none() {
                let form = crate::lowcode::services::form_builder::get_form(pool, ctx.operation_id).await?;
                ctx.proposed_form = Some(
                    serde_json::to_value(&form)
                        .map_err(|e| AppError::Internal(format!("Serialization error: {e}")))?,
                );
            }
            let proposed = ctx.proposed_form.as_mut().unwrap();
            let field_name = input["field_name"]
                .as_str()
                .ok_or_else(|| AppError::Validation("field_name is required".into()))?;

            let mut found = false;
            if let Some(sections) = proposed.get_mut("sections") {
                if let Some(sections_arr) = sections.as_array_mut() {
                    for section in sections_arr.iter_mut() {
                        if let Some(fields) = section.get_mut("fields") {
                            if let Some(fields_arr) = fields.as_array_mut() {
                                let before = fields_arr.len();
                                fields_arr.retain(|f| f["field_name"].as_str() != Some(field_name));
                                if fields_arr.len() < before {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if found {
                Ok(serde_json::json!({ "status": "ok", "message": format!("Field '{}' removed", field_name) }))
            } else {
                Err(AppError::Validation(format!(
                    "Field '{}' not found",
                    field_name
                )))
            }
        }

        _ => Err(AppError::Validation(format!("Unknown tool: {name}"))),
    }
}

/// Build the system prompt with operation context and platform information.
pub fn build_system_prompt(
    operation: &Operation,
    context_type: &str,
    current_state: &Value,
    table_names: &[String],
) -> String {
    let tables_list = if table_names.len() > 50 {
        let sample: Vec<&str> = table_names.iter().take(50).map(|s| s.as_str()).collect();
        format!("{} (showing first 50 of {})", sample.join(", "), table_names.len())
    } else {
        table_names.join(", ")
    };

    let current_state_str = serde_json::to_string_pretty(current_state).unwrap_or_default();
    let current_state_snippet = if current_state_str.len() > 3000 {
        format!("{}...(truncated)", &current_state_str[..3000])
    } else {
        current_state_str
    };

    format!(
        r#"You are an AI assistant for TasteByte ERP's Low-Code Builder platform.
You help developers create and modify form, list, and dashboard definitions.

## Current Operation
- Name: {name}
- Code: {code}
- Type: {op_type}
- Target Table: {target_table}

## Context Type: {context_type}

## Available Field Types
text, number, textarea, checkbox, toggle, dropdown, multi_select, radio, date, datetime, time,
email, phone, url, currency, percentage, file, image, rich_text, code_editor, color, rating

## Form Structure
A form consists of sections, each containing fields:
- Section: title, description, columns (grid layout), sort_order, fields[]
- Field: field_name, field_label, field_type, db_column, is_required, placeholder, help_text,
  validation_regex, min_value, max_value, sort_order, column_span, options[]

## Rules
1. ALWAYS use tool calls to read current state before making changes.
2. Propose changes using the write tools — they do NOT modify the database directly.
3. Use db_column to map fields to database columns when the operation has a target table.
4. Field names should be snake_case, labels should be human-readable.
5. Use appropriate field types based on the data (e.g., currency for money, date for dates).
6. Set is_required=true for mandatory fields.
7. Group related fields into sections logically.

## Available Tables
{tables}

## Current {context_type} Definition
{current_state}
"#,
        name = operation.name,
        code = operation.operation_code,
        op_type = operation.operation_type,
        target_table = operation.target_table.as_deref().unwrap_or("(none)"),
        context_type = context_type,
        tables = tables_list,
        current_state = current_state_snippet,
    )
}

/// Generate a human-readable summary of changes between current and proposed definitions.
pub fn generate_change_summary(current: &Value, proposed: &Value, change_type: &str) -> Vec<String> {
    let mut summary = Vec::new();

    match change_type {
        "form" => {
            let current_sections = current
                .get("sections")
                .and_then(|s| s.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            let proposed_sections = proposed
                .get("sections")
                .and_then(|s| s.as_array())
                .map(|a| a.len())
                .unwrap_or(0);

            if proposed_sections != current_sections {
                summary.push(format!(
                    "Sections: {} -> {}",
                    current_sections, proposed_sections
                ));
            }

            let count_fields = |v: &Value| -> usize {
                v.get("sections")
                    .and_then(|s| s.as_array())
                    .map(|sections| {
                        sections
                            .iter()
                            .map(|s| {
                                s.get("fields")
                                    .and_then(|f| f.as_array())
                                    .map(|a| a.len())
                                    .unwrap_or(0)
                            })
                            .sum()
                    })
                    .unwrap_or(0)
            };

            let current_fields = count_fields(current);
            let proposed_fields = count_fields(proposed);
            if proposed_fields != current_fields {
                summary.push(format!(
                    "Total fields: {} -> {}",
                    current_fields, proposed_fields
                ));
            }
        }
        "list" => {
            let current_cols = current
                .get("columns")
                .and_then(|c| c.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            let proposed_cols = proposed
                .get("columns")
                .and_then(|c| c.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            if proposed_cols != current_cols {
                summary.push(format!("Columns: {} -> {}", current_cols, proposed_cols));
            }
        }
        "dashboard" => {
            let current_widgets = current
                .get("widgets")
                .and_then(|w| w.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            let proposed_widgets = proposed
                .get("widgets")
                .and_then(|w| w.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            if proposed_widgets != current_widgets {
                summary.push(format!(
                    "Widgets: {} -> {}",
                    current_widgets, proposed_widgets
                ));
            }
        }
        "buttons" => {
            let count = proposed
                .as_array()
                .map(|a| a.len())
                .unwrap_or(0);
            summary.push(format!("{} buttons defined", count));
        }
        _ => {}
    }

    if summary.is_empty() {
        summary.push("Definition updated".to_string());
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tool_definitions_include_common_tools() {
        let tools = get_tool_definitions("form");
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"list_tables"));
        assert!(names.contains(&"get_table_schema"));
        assert!(names.contains(&"validate_sql"));
        assert!(names.contains(&"get_current_form"));
        assert!(names.contains(&"set_form_definition"));
        assert!(names.contains(&"add_section"));
        assert!(names.contains(&"add_field"));
        assert!(names.contains(&"update_field"));
        assert!(names.contains(&"remove_field"));
        assert!(names.contains(&"set_operation_buttons"));
    }

    #[test]
    fn list_tools_include_list_specific() {
        let tools = get_tool_definitions("list");
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"get_current_list"));
        assert!(names.contains(&"set_list_definition"));
        assert!(!names.contains(&"get_current_form"));
    }

    #[test]
    fn dashboard_tools_include_dashboard_specific() {
        let tools = get_tool_definitions("dashboard");
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"get_current_dashboard"));
        assert!(names.contains(&"set_dashboard_definition"));
    }

    #[test]
    fn change_summary_form() {
        let current = json!({ "sections": [{ "fields": [{"a": 1}] }] });
        let proposed = json!({ "sections": [{ "fields": [{"a": 1}, {"b": 2}] }, { "fields": [] }] });
        let summary = generate_change_summary(&current, &proposed, "form");
        assert!(!summary.is_empty());
    }

    #[test]
    fn change_summary_buttons() {
        let current = json!(null);
        let proposed = json!([{"label": "Save"}, {"label": "Delete"}]);
        let summary = generate_change_summary(&current, &proposed, "buttons");
        assert!(summary[0].contains("2 buttons"));
    }
}
