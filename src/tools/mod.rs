use async_trait::async_trait;

use crate::error::MidnaError;
use crate::permissions::Risk;
use crate::providers::{ToolFunctionSpec, ToolSpec};

pub mod fs;
pub mod shell;

/// Midna が LLM に提供する 1 機能。
/// LLM には `spec()` の内容（名前・説明・引数スキーマ）だけが渡り、
/// 実行要求が返ってきたら `call()` で実体を動かす。
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    /// 引数の JSON Schema
    fn parameters(&self) -> serde_json::Value;
    /// 権限判定に使う危険度
    fn risk(&self) -> Risk;
    async fn call(&self, args: &serde_json::Value) -> Result<String, MidnaError>;

    fn spec(&self) -> ToolSpec {
        ToolSpec {
            kind: "function",
            function: ToolFunctionSpec {
                name: self.name().to_string(),
                description: self.description().to_string(),
                parameters: self.parameters(),
            },
        }
    }
}

/// 利用可能なツールの一覧。名前で引けるようにする。
pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    /// 組み込みツール一式を持つレジストリ
    pub fn builtin() -> Self {
        Self {
            tools: vec![
                Box::new(fs::ReadFile),
                Box::new(fs::WriteFile),
                Box::new(fs::ListDir),
                Box::new(shell::RunShell),
            ],
        }
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools
            .iter()
            .find(|t| t.name() == name)
            .map(|t| t.as_ref())
    }

    pub fn specs(&self) -> Vec<ToolSpec> {
        self.tools.iter().map(|t| t.spec()).collect()
    }
}

/// ツール引数から必須の文字列フィールドを取り出す共通処理
pub(crate) fn required_str<'a>(
    args: &'a serde_json::Value,
    key: &str,
) -> Result<&'a str, MidnaError> {
    args.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| MidnaError::Tool(format!("missing required string argument `{key}`")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_registry_resolves_tools_by_name() {
        let registry = ToolRegistry::builtin();
        for name in ["read_file", "write_file", "list_dir", "run_shell"] {
            assert!(registry.get(name).is_some(), "missing tool: {name}");
        }
        assert!(registry.get("no_such_tool").is_none());
    }

    #[test]
    fn specs_expose_function_schemas() {
        let registry = ToolRegistry::builtin();
        let specs = registry.specs();
        assert_eq!(specs.len(), 4);
        assert!(specs.iter().all(|s| s.kind == "function"));
        assert!(specs
            .iter()
            .all(|s| s.function.parameters.get("type").is_some()));
    }
}
