use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::MidnaError;

pub mod ollama;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    /// ツール実行結果を LLM に返すときの role（Ollama の tool calling 仕様）
    Tool,
}

/// LLM が「このツールをこの引数で呼びたい」と返してくる要求。
/// Ollama の応答では `message.tool_calls[].function` に入る。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    /// 引数は JSON オブジェクト（例: {"path": "src/main.rs"}）
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
            tool_calls: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
            tool_calls: None,
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
            tool_calls: None,
        }
    }

    pub fn tool(content: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: content.into(),
            tool_calls: None,
        }
    }

    /// LLM がツール実行を要求しているか
    pub fn has_tool_calls(&self) -> bool {
        self.tool_calls
            .as_ref()
            .is_some_and(|calls| !calls.is_empty())
    }
}

/// LLM に「こういうツールが使える」と伝えるための定義。
/// リクエストの `tools` にそのまま serialize される（OpenAI 互換の function 形式）。
#[derive(Debug, Clone, Serialize)]
pub struct ToolSpec {
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub function: ToolFunctionSpec,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolFunctionSpec {
    pub name: String,
    pub description: String,
    /// 引数の JSON Schema
    pub parameters: serde_json::Value,
}

#[async_trait]
pub trait Provider: Send + Sync {
    /// 会話履歴と利用可能ツールを渡して 1 応答を得る。
    /// ツールを使わない場合は `tools` に空スライスを渡す。
    async fn chat(&self, messages: &[Message], tools: &[ToolSpec])
        -> Result<Message, MidnaError>;
    fn name(&self) -> &'static str;
}
