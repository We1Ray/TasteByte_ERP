use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::lowcode::models::*;
use crate::lowcode::services::ai_service::{ChatContent, ChatMessage, ContentBlock};
use crate::lowcode::services::ai_tools::{
    build_system_prompt, execute_tool, generate_change_summary, get_tool_definitions,
    ToolExecutionContext,
};
use crate::lowcode::services::diff_engine::json_diff;
use crate::lowcode::services::permission_resolver::{PlatformDeveloper, RequirePlatformRole};
use crate::shared::types::AppState;
use crate::shared::{ApiResponse, AppError};

pub async fn chat(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
    Path(operation_id): Path<Uuid>,
    Json(input): Json<AiChatRequest>,
) -> Result<Json<ApiResponse<AiChatResponse>>, AppError> {
    let llm = state
        .llm_client
        .as_ref()
        .ok_or_else(|| AppError::Validation("AI assistant is not enabled".to_string()))?;

    // Load operation
    let operation: Operation = sqlx::query_as(
        "SELECT * FROM lc_operations WHERE id = $1",
    )
    .bind(operation_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Operation not found".to_string()))?;

    let context_type = input
        .context_type
        .as_deref()
        .unwrap_or(match operation.operation_type.to_lowercase().as_str() {
            "list" => "list",
            "dashboard" => "dashboard",
            _ => "form",
        });

    // Load table list for context
    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables \
         WHERE table_schema = 'public' ORDER BY table_name",
    )
    .fetch_all(&state.pool)
    .await?;
    let table_names: Vec<String> = tables.into_iter().map(|(n,)| n).collect();

    // Load current state for the context type
    let current_state = load_current_state(&state.pool, operation_id, context_type).await?;

    // Build system prompt
    let system_prompt =
        build_system_prompt(&operation, context_type, &current_state, &table_names);

    // Build tool definitions
    let tool_defs = get_tool_definitions(context_type);

    // Build messages from conversation history
    let mut messages: Vec<ChatMessage> = Vec::new();
    if let Some(history) = &input.conversation_history {
        for msg in history {
            messages.push(ChatMessage {
                role: msg.role.clone(),
                content: ChatContent::Text(msg.content.clone()),
            });
        }
    }
    // Add current user message
    messages.push(ChatMessage {
        role: "user".to_string(),
        content: ChatContent::Text(input.message.clone()),
    });

    // Create tool execution context
    let mut ctx = ToolExecutionContext::new(state.pool.clone(), operation_id);

    // Tool loop (max 5 rounds)
    let mut tool_calls_executed = Vec::new();
    let mut final_text = String::new();

    for _ in 0..5 {
        let response = llm
            .send_message(&system_prompt, &messages, &tool_defs)
            .await?;

        // Collect text blocks and tool uses
        let mut text_parts = Vec::new();
        let mut tool_uses = Vec::new();
        for block in &response.content {
            match block {
                ContentBlock::Text { text } => text_parts.push(text.clone()),
                ContentBlock::ToolUse { id, name, input } => {
                    tool_uses.push((id.clone(), name.clone(), input.clone()));
                }
                _ => {}
            }
        }

        if !text_parts.is_empty() {
            final_text = text_parts.join("\n");
        }

        if tool_uses.is_empty() || response.stop_reason != "tool_use" {
            break;
        }

        // Add assistant message with tool_use blocks
        messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: ChatContent::Blocks(response.content.clone()),
        });

        // Execute tools and collect results
        let mut tool_results = Vec::new();
        for (id, name, args) in &tool_uses {
            tool_calls_executed.push(name.clone());
            let result = execute_tool(&mut ctx, name, args, &state.pool).await;
            let result_str = match result {
                Ok(val) => serde_json::to_string(&val).unwrap_or_else(|_| "{}".to_string()),
                Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
            };
            tool_results.push(ContentBlock::ToolResult {
                tool_use_id: id.clone(),
                content: result_str,
            });
        }

        // Add tool results as user message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: ChatContent::Blocks(tool_results),
        });
    }

    // Build proposed changes
    let proposed_changes = build_proposed_changes(&ctx, &current_state, context_type);

    Ok(Json(ApiResponse::success(AiChatResponse {
        message: final_text,
        proposed_changes,
        tool_calls_executed,
    })))
}

async fn load_current_state(
    pool: &sqlx::PgPool,
    operation_id: Uuid,
    context_type: &str,
) -> Result<serde_json::Value, AppError> {
    match context_type {
        "form" => {
            let form =
                crate::lowcode::services::form_builder::get_form(pool, operation_id).await?;
            Ok(serde_json::to_value(&form).unwrap_or_default())
        }
        "list" => {
            let list: Option<ListDefinitionRow> =
                sqlx::query_as("SELECT * FROM lc_list_definitions WHERE operation_id = $1")
                    .bind(operation_id)
                    .fetch_optional(pool)
                    .await?;
            if let Some(list) = list {
                let columns: Vec<ListColumnRow> = sqlx::query_as(
                    "SELECT * FROM lc_list_columns WHERE list_id = $1 ORDER BY sort_order",
                )
                .bind(list.id)
                .fetch_all(pool)
                .await?;
                let actions: Vec<ListActionRow> = sqlx::query_as(
                    "SELECT * FROM lc_list_actions WHERE list_id = $1 ORDER BY sort_order",
                )
                .bind(list.id)
                .fetch_all(pool)
                .await?;
                Ok(serde_json::to_value(&ListResponse {
                    list,
                    columns,
                    actions,
                })
                .unwrap_or_default())
            } else {
                Ok(serde_json::Value::Null)
            }
        }
        "dashboard" => {
            let dash: Option<DashboardDefinitionRow> = sqlx::query_as(
                "SELECT * FROM lc_dashboard_definitions WHERE operation_id = $1",
            )
            .bind(operation_id)
            .fetch_optional(pool)
            .await?;
            if let Some(dash) = dash {
                let widgets: Vec<DashboardWidgetRow> = sqlx::query_as(
                    "SELECT * FROM lc_dashboard_widgets WHERE dashboard_id = $1 ORDER BY sort_order",
                )
                .bind(dash.id)
                .fetch_all(pool)
                .await?;
                Ok(serde_json::to_value(&DashboardResponse {
                    dashboard: dash,
                    widgets,
                })
                .unwrap_or_default())
            } else {
                Ok(serde_json::Value::Null)
            }
        }
        _ => Ok(serde_json::Value::Null),
    }
}

fn build_proposed_changes(
    ctx: &ToolExecutionContext,
    current_state: &serde_json::Value,
    context_type: &str,
) -> Option<ProposedChanges> {
    if let Some(proposed) = &ctx.proposed_form {
        let diff = json_diff(current_state, proposed);
        let summary = generate_change_summary(current_state, proposed, "form");
        return Some(ProposedChanges {
            change_type: "form".to_string(),
            current: current_state.clone(),
            proposed: proposed.clone(),
            diff,
            summary,
        });
    }
    if let Some(proposed) = &ctx.proposed_list {
        let current = if context_type == "list" {
            current_state
        } else {
            &serde_json::Value::Null
        };
        let diff = json_diff(current, proposed);
        let summary = generate_change_summary(current, proposed, "list");
        return Some(ProposedChanges {
            change_type: "list".to_string(),
            current: current.clone(),
            proposed: proposed.clone(),
            diff,
            summary,
        });
    }
    if let Some(proposed) = &ctx.proposed_dashboard {
        let current = if context_type == "dashboard" {
            current_state
        } else {
            &serde_json::Value::Null
        };
        let diff = json_diff(current, proposed);
        let summary = generate_change_summary(current, proposed, "dashboard");
        return Some(ProposedChanges {
            change_type: "dashboard".to_string(),
            current: current.clone(),
            proposed: proposed.clone(),
            diff,
            summary,
        });
    }
    if let Some(proposed) = &ctx.proposed_buttons {
        let diff = json_diff(&serde_json::Value::Null, proposed);
        let summary = generate_change_summary(&serde_json::Value::Null, proposed, "buttons");
        return Some(ProposedChanges {
            change_type: "buttons".to_string(),
            current: serde_json::Value::Null,
            proposed: proposed.clone(),
            diff,
            summary,
        });
    }
    None
}

pub async fn ai_status(
    State(state): State<AppState>,
    _guard: RequirePlatformRole<PlatformDeveloper>,
) -> Result<Json<ApiResponse<AiStatusResponse>>, AppError> {
    let enabled = state.llm_client.is_some();
    Ok(Json(ApiResponse::success(AiStatusResponse {
        enabled,
        provider: if enabled {
            Some(state.settings.ai_provider.clone())
        } else {
            None
        },
        model: if enabled {
            Some(state.settings.ai_model.clone())
        } else {
            None
        },
    })))
}
