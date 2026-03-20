use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Settings;
use crate::shared::AppError;

#[derive(Debug, Clone, Copy)]
pub enum LlmProvider {
    Claude,
    OpenAi,
}

#[derive(Debug, Clone)]
pub struct LlmClient {
    http: Client,
    provider: LlmProvider,
    api_key: String,
    model: String,
    max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: ChatContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
    },
}

#[derive(Debug, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug)]
pub struct LlmResponse {
    pub content: Vec<ContentBlock>,
    pub stop_reason: String,
}

impl LlmClient {
    pub fn new(settings: &Settings) -> Option<Self> {
        if !settings.ai_enabled {
            return None;
        }
        let api_key = settings.ai_api_key.as_ref()?.clone();
        if api_key.is_empty() {
            return None;
        }
        let provider = match settings.ai_provider.to_lowercase().as_str() {
            "openai" => LlmProvider::OpenAi,
            _ => LlmProvider::Claude,
        };
        Some(Self {
            http: Client::new(),
            provider,
            api_key,
            model: settings.ai_model.clone(),
            max_tokens: settings.ai_max_tokens,
        })
    }

    pub async fn send_message(
        &self,
        system: &str,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
    ) -> Result<LlmResponse, AppError> {
        match self.provider {
            LlmProvider::Claude => self.send_claude(system, messages, tools).await,
            LlmProvider::OpenAi => self.send_openai(system, messages, tools).await,
        }
    }

    async fn send_claude(
        &self,
        system: &str,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
    ) -> Result<LlmResponse, AppError> {
        let claude_tools: Vec<Value> = tools
            .iter()
            .map(|t| {
                serde_json::json!({
                    "name": t.name,
                    "description": t.description,
                    "input_schema": t.input_schema,
                })
            })
            .collect();

        let claude_messages: Vec<Value> = messages
            .iter()
            .map(|m| match &m.content {
                ChatContent::Text(text) => serde_json::json!({
                    "role": m.role,
                    "content": text,
                }),
                ChatContent::Blocks(blocks) => {
                    let content_blocks: Vec<Value> = blocks
                        .iter()
                        .map(|b| match b {
                            ContentBlock::Text { text } => {
                                serde_json::json!({"type": "text", "text": text})
                            }
                            ContentBlock::ToolUse { id, name, input } => {
                                serde_json::json!({"type": "tool_use", "id": id, "name": name, "input": input})
                            }
                            ContentBlock::ToolResult {
                                tool_use_id,
                                content,
                            } => {
                                serde_json::json!({"type": "tool_result", "tool_use_id": tool_use_id, "content": content})
                            }
                        })
                        .collect();
                    serde_json::json!({
                        "role": m.role,
                        "content": content_blocks,
                    })
                }
            })
            .collect();

        let mut body = serde_json::json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "system": system,
            "messages": claude_messages,
        });
        if !tools.is_empty() {
            body["tools"] = serde_json::json!(claude_tools);
        }

        let resp = self
            .http
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("LLM request failed: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "LLM API error ({status}): {text}"
            )));
        }

        let data: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse LLM response: {e}")))?;

        let stop_reason = data["stop_reason"]
            .as_str()
            .unwrap_or("end_turn")
            .to_string();
        let content_array = data["content"]
            .as_array()
            .ok_or_else(|| AppError::Internal("Invalid LLM response: missing content".into()))?;

        let mut content = Vec::new();
        for block in content_array {
            match block["type"].as_str() {
                Some("text") => {
                    content.push(ContentBlock::Text {
                        text: block["text"].as_str().unwrap_or("").to_string(),
                    });
                }
                Some("tool_use") => {
                    content.push(ContentBlock::ToolUse {
                        id: block["id"].as_str().unwrap_or("").to_string(),
                        name: block["name"].as_str().unwrap_or("").to_string(),
                        input: block["input"].clone(),
                    });
                }
                _ => {}
            }
        }

        Ok(LlmResponse {
            content,
            stop_reason,
        })
    }

    async fn send_openai(
        &self,
        system: &str,
        messages: &[ChatMessage],
        tools: &[ToolDefinition],
    ) -> Result<LlmResponse, AppError> {
        let openai_tools: Vec<Value> = tools
            .iter()
            .map(|t| {
                serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.input_schema,
                    }
                })
            })
            .collect();

        let mut openai_messages: Vec<Value> =
            vec![serde_json::json!({"role": "system", "content": system})];
        for m in messages {
            match &m.content {
                ChatContent::Text(text) => {
                    openai_messages.push(serde_json::json!({"role": m.role, "content": text}));
                }
                ChatContent::Blocks(blocks) => {
                    for block in blocks {
                        match block {
                            ContentBlock::Text { text } => {
                                openai_messages
                                    .push(serde_json::json!({"role": m.role, "content": text}));
                            }
                            ContentBlock::ToolResult {
                                tool_use_id,
                                content,
                            } => {
                                openai_messages.push(serde_json::json!({
                                    "role": "tool",
                                    "tool_call_id": tool_use_id,
                                    "content": content,
                                }));
                            }
                            ContentBlock::ToolUse { id, name, input } => {
                                openai_messages.push(serde_json::json!({
                                    "role": "assistant",
                                    "tool_calls": [{
                                        "id": id,
                                        "type": "function",
                                        "function": {"name": name, "arguments": input.to_string()}
                                    }]
                                }));
                            }
                        }
                    }
                }
            }
        }

        let mut body = serde_json::json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": openai_messages,
        });
        if !tools.is_empty() {
            body["tools"] = serde_json::json!(openai_tools);
        }

        let resp = self
            .http
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("LLM request failed: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "LLM API error ({status}): {text}"
            )));
        }

        let data: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse LLM response: {e}")))?;

        let choice = &data["choices"][0];
        let message = &choice["message"];
        let finish_reason = choice["finish_reason"].as_str().unwrap_or("stop");

        let mut content = Vec::new();
        if let Some(text) = message["content"].as_str() {
            if !text.is_empty() {
                content.push(ContentBlock::Text {
                    text: text.to_string(),
                });
            }
        }
        if let Some(tool_calls) = message["tool_calls"].as_array() {
            for tc in tool_calls {
                let args_str = tc["function"]["arguments"].as_str().unwrap_or("{}");
                let input: Value =
                    serde_json::from_str(args_str).unwrap_or(Value::Object(Default::default()));
                content.push(ContentBlock::ToolUse {
                    id: tc["id"].as_str().unwrap_or("").to_string(),
                    name: tc["function"]["name"].as_str().unwrap_or("").to_string(),
                    input,
                });
            }
        }

        let stop_reason = if finish_reason == "tool_calls" {
            "tool_use".to_string()
        } else {
            "end_turn".to_string()
        };
        Ok(LlmResponse {
            content,
            stop_reason,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_none_when_disabled() {
        let settings = Settings {
            database_url: String::new(),
            jwt_secret: String::new(),
            jwt_expiry_hours: 24,
            access_token_expiry_minutes: 15,
            refresh_token_expiry_days: 7,
            server_host: "0.0.0.0".to_string(),
            server_port: 8000,
            ai_provider: "claude".to_string(),
            ai_api_key: Some("test-key".to_string()),
            ai_model: "claude-sonnet-4-20250514".to_string(),
            ai_max_tokens: 4096,
            ai_enabled: false,
        };
        assert!(LlmClient::new(&settings).is_none());
    }

    #[test]
    fn new_returns_none_when_no_key() {
        let settings = Settings {
            database_url: String::new(),
            jwt_secret: String::new(),
            jwt_expiry_hours: 24,
            access_token_expiry_minutes: 15,
            refresh_token_expiry_days: 7,
            server_host: "0.0.0.0".to_string(),
            server_port: 8000,
            ai_provider: "claude".to_string(),
            ai_api_key: None,
            ai_model: "claude-sonnet-4-20250514".to_string(),
            ai_max_tokens: 4096,
            ai_enabled: true,
        };
        assert!(LlmClient::new(&settings).is_none());
    }

    #[test]
    fn new_returns_some_when_enabled_with_key() {
        let settings = Settings {
            database_url: String::new(),
            jwt_secret: String::new(),
            jwt_expiry_hours: 24,
            access_token_expiry_minutes: 15,
            refresh_token_expiry_days: 7,
            server_host: "0.0.0.0".to_string(),
            server_port: 8000,
            ai_provider: "claude".to_string(),
            ai_api_key: Some("test-key".to_string()),
            ai_model: "claude-sonnet-4-20250514".to_string(),
            ai_max_tokens: 4096,
            ai_enabled: true,
        };
        let client = LlmClient::new(&settings);
        assert!(client.is_some());
    }

    #[test]
    fn openai_provider_detected() {
        let settings = Settings {
            database_url: String::new(),
            jwt_secret: String::new(),
            jwt_expiry_hours: 24,
            access_token_expiry_minutes: 15,
            refresh_token_expiry_days: 7,
            server_host: "0.0.0.0".to_string(),
            server_port: 8000,
            ai_provider: "openai".to_string(),
            ai_api_key: Some("sk-test".to_string()),
            ai_model: "gpt-4".to_string(),
            ai_max_tokens: 4096,
            ai_enabled: true,
        };
        let client = LlmClient::new(&settings).unwrap();
        assert!(matches!(client.provider, LlmProvider::OpenAi));
    }
}
