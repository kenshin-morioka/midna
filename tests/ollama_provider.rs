use midna::error::MidnaError;
use midna::providers::ollama::OllamaProvider;
use midna::providers::{Message, Provider, Role};
use midna::tools::ToolRegistry;
use serde_json::json;
use wiremock::matchers::{body_partial_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn parses_assistant_message_from_ollama() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "model": "llama3.1:8b",
            "message": { "role": "assistant", "content": "hi from llama" },
            "done": true
        })))
        .mount(&server)
        .await;

    let provider = OllamaProvider::new(server.uri(), "llama3.1:8b").expect("client");
    let messages = vec![Message::user("hello")];

    let reply = provider.chat(&messages, &[]).await.expect("ok");
    assert_eq!(reply.role, Role::Assistant);
    assert_eq!(reply.content, "hi from llama");
    assert!(!reply.has_tool_calls());
}

#[tokio::test]
async fn sends_tools_and_parses_tool_calls() {
    let server = MockServer::start().await;

    // リクエストに tools が入っていることを body_partial_json で検証する
    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .and(body_partial_json(json!({
            "tools": [{ "type": "function", "function": { "name": "read_file" } }]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "model": "llama3.1:8b",
            "message": {
                "role": "assistant",
                "content": "",
                "tool_calls": [
                    { "function": { "name": "read_file", "arguments": { "path": "src/main.rs" } } }
                ]
            },
            "done": true
        })))
        .expect(1)
        .mount(&server)
        .await;

    let provider = OllamaProvider::new(server.uri(), "llama3.1:8b").expect("client");
    let specs = ToolRegistry::builtin().specs();
    let read_file_spec = specs
        .into_iter()
        .filter(|s| s.function.name == "read_file")
        .collect::<Vec<_>>();

    let reply = provider
        .chat(&[Message::user("read main.rs")], &read_file_spec)
        .await
        .expect("ok");

    assert!(reply.has_tool_calls());
    let calls = reply.tool_calls.expect("tool_calls");
    assert_eq!(calls[0].function.name, "read_file");
    assert_eq!(calls[0].function.arguments["path"], "src/main.rs");
}

#[tokio::test]
async fn tool_role_message_serializes_as_tool() {
    let serialized = serde_json::to_value(Message::tool("file content here")).expect("json");
    assert_eq!(serialized["role"], "tool");
    assert_eq!(serialized["content"], "file content here");
    // tool_calls が None のときはフィールド自体を省く（Ollama への余計な送信を避ける）
    assert!(serialized.get("tool_calls").is_none());
}

#[tokio::test]
async fn returns_provider_error_on_non_2xx() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
        .mount(&server)
        .await;

    let provider = OllamaProvider::new(server.uri(), "llama3.1:8b").expect("client");
    let err = provider
        .chat(&[Message::user("hi")], &[])
        .await
        .expect_err("should fail");

    match err {
        MidnaError::Provider(msg) => {
            assert!(msg.contains("500"), "expected status in error: {msg}");
            assert!(msg.contains("llama3.1:8b"), "expected model hint in error: {msg}");
        }
        other => panic!("expected Provider error, got: {other:?}"),
    }
}

#[tokio::test]
async fn returns_connect_error_when_host_unreachable() {
    // Port 1 is reserved and should not be listening.
    let provider = OllamaProvider::new("http://127.0.0.1:1", "llama3.1:8b").expect("client");
    let err = provider
        .chat(&[Message::user("hi")], &[])
        .await
        .expect_err("should fail");

    match err {
        MidnaError::Connect(msg) => {
            assert!(
                msg.contains("ollama serve"),
                "expected friendly hint in error: {msg}"
            );
        }
        other => panic!("expected Connect error, got: {other:?}"),
    }
}
