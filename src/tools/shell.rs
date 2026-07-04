use std::time::Duration;

use async_trait::async_trait;
use serde_json::json;

use crate::error::MidnaError;
use crate::permissions::Risk;
use crate::tools::{required_str, Tool};

/// ハングしたコマンドでエージェントループ全体が止まらないようにする上限
const COMMAND_TIMEOUT: Duration = Duration::from_secs(60);
/// 出力で LLM のコンテキストを溢れさせないための上限
const MAX_OUTPUT_BYTES: usize = 32 * 1024;

pub struct RunShell;

#[async_trait]
impl Tool for RunShell {
    fn name(&self) -> &'static str {
        "run_shell"
    }

    fn description(&self) -> &'static str {
        "Run a shell command in the current working directory and return stdout, stderr, and the exit code."
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "Shell command to run (passed to `sh -c`)" }
            },
            "required": ["command"]
        })
    }

    fn risk(&self) -> Risk {
        Risk::Execute
    }

    async fn call(&self, args: &serde_json::Value) -> Result<String, MidnaError> {
        let command = required_str(args, "command")?;

        let output = tokio::time::timeout(
            COMMAND_TIMEOUT,
            tokio::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .output(),
        )
        .await
        .map_err(|_| {
            MidnaError::Tool(format!(
                "command timed out after {}s: {command}",
                COMMAND_TIMEOUT.as_secs()
            ))
        })?
        .map_err(|e| MidnaError::Tool(format!("failed to run command: {e}")))?;

        let mut result = String::new();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stdout.is_empty() {
            result.push_str(&stdout);
        }
        if !stderr.is_empty() {
            result.push_str("\n[stderr]\n");
            result.push_str(&stderr);
        }
        if result.len() > MAX_OUTPUT_BYTES {
            let mut end = MAX_OUTPUT_BYTES;
            while !result.is_char_boundary(end) {
                end -= 1;
            }
            result.truncate(end);
            result.push_str("\n[... truncated ...]");
        }
        let code = output
            .status
            .code()
            .map_or("unknown".to_string(), |c| c.to_string());
        result.push_str(&format!("\n[exit code: {code}]"));
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn captures_stdout_and_exit_code() {
        let out = RunShell
            .call(&json!({ "command": "echo hello" }))
            .await
            .expect("ok");
        assert!(out.contains("hello"));
        assert!(out.contains("[exit code: 0]"));
    }

    #[tokio::test]
    async fn captures_stderr_and_nonzero_exit() {
        let out = RunShell
            .call(&json!({ "command": "echo oops >&2; exit 3" }))
            .await
            .expect("ok");
        assert!(out.contains("[stderr]"));
        assert!(out.contains("oops"));
        assert!(out.contains("[exit code: 3]"));
    }
}
