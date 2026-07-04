use async_trait::async_trait;
use serde_json::json;

use crate::error::MidnaError;
use crate::permissions::Risk;
use crate::tools::{required_str, Tool};

/// 巨大ファイルで LLM のコンテキストを溢れさせないための上限
const MAX_READ_BYTES: usize = 64 * 1024;

pub struct ReadFile;

#[async_trait]
impl Tool for ReadFile {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn description(&self) -> &'static str {
        "Read a UTF-8 text file and return its content."
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path to the file to read" }
            },
            "required": ["path"]
        })
    }

    fn risk(&self) -> Risk {
        Risk::ReadOnly
    }

    async fn call(&self, args: &serde_json::Value) -> Result<String, MidnaError> {
        let path = required_str(args, "path")?;
        let mut content = tokio::fs::read_to_string(path).await.map_err(|e| {
            MidnaError::Tool(format!("failed to read `{path}`: {e}"))
        })?;
        if content.len() > MAX_READ_BYTES {
            let mut end = MAX_READ_BYTES;
            // UTF-8 の文字境界の途中で切ると文字列として不正になるため境界まで戻す
            while !content.is_char_boundary(end) {
                end -= 1;
            }
            content.truncate(end);
            content.push_str("\n[... truncated ...]");
        }
        Ok(content)
    }
}

pub struct WriteFile;

#[async_trait]
impl Tool for WriteFile {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn description(&self) -> &'static str {
        "Write content to a file, creating it (and parent directories) if needed, or overwriting it."
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path to the file to write" },
                "content": { "type": "string", "description": "Full content to write" }
            },
            "required": ["path", "content"]
        })
    }

    fn risk(&self) -> Risk {
        Risk::Write
    }

    async fn call(&self, args: &serde_json::Value) -> Result<String, MidnaError> {
        let path = required_str(args, "path")?;
        let content = required_str(args, "content")?;
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| {
                    MidnaError::Tool(format!("failed to create directories for `{path}`: {e}"))
                })?;
            }
        }
        tokio::fs::write(path, content).await.map_err(|e| {
            MidnaError::Tool(format!("failed to write `{path}`: {e}"))
        })?;
        Ok(format!("wrote {} bytes to {path}", content.len()))
    }
}

pub struct ListDir;

#[async_trait]
impl Tool for ListDir {
    fn name(&self) -> &'static str {
        "list_dir"
    }

    fn description(&self) -> &'static str {
        "List entries in a directory. Directories are suffixed with '/'."
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Directory path (defaults to '.')" }
            }
        })
    }

    fn risk(&self) -> Risk {
        Risk::ReadOnly
    }

    async fn call(&self, args: &serde_json::Value) -> Result<String, MidnaError> {
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let mut dir = tokio::fs::read_dir(path).await.map_err(|e| {
            MidnaError::Tool(format!("failed to list `{path}`: {e}"))
        })?;
        let mut entries = Vec::new();
        while let Some(entry) = dir.next_entry().await.map_err(MidnaError::Io)? {
            let name = entry.file_name().to_string_lossy().into_owned();
            let is_dir = entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false);
            entries.push(if is_dir { format!("{name}/") } else { name });
        }
        entries.sort();
        Ok(entries.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn write_then_read_roundtrip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("nested/hello.txt");
        let path_str = path.to_string_lossy().into_owned();

        let written = WriteFile
            .call(&json!({ "path": path_str, "content": "こんにちは" }))
            .await
            .expect("write ok");
        assert!(written.contains(&path_str));

        let content = ReadFile
            .call(&json!({ "path": path_str }))
            .await
            .expect("read ok");
        assert_eq!(content, "こんにちは");
    }

    #[tokio::test]
    async fn list_dir_marks_directories() {
        let dir = tempfile::tempdir().expect("tempdir");
        tokio::fs::create_dir(dir.path().join("sub")).await.unwrap();
        tokio::fs::write(dir.path().join("a.txt"), "x").await.unwrap();

        let listed = ListDir
            .call(&json!({ "path": dir.path().to_string_lossy() }))
            .await
            .expect("list ok");
        assert_eq!(listed, "a.txt\nsub/");
    }

    #[tokio::test]
    async fn read_missing_file_returns_tool_error() {
        let err = ReadFile
            .call(&json!({ "path": "/no/such/file/exists" }))
            .await
            .expect_err("should fail");
        assert!(matches!(err, MidnaError::Tool(_)));
    }

    #[tokio::test]
    async fn missing_required_argument_is_rejected() {
        let err = ReadFile.call(&json!({})).await.expect_err("should fail");
        assert!(matches!(err, MidnaError::Tool(_)));
    }
}
