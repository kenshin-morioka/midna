use midna::error::MidnaError;
use midna::providers::ollama::OllamaProvider;
use midna::providers::{Message, Provider, Role};
use serde_json::json;
use wiremock::matchers::{method, path};
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

    let reply = provider.chat(&messages).await.expect("ok");
    assert_eq!(reply.role, Role::Assistant);
    assert_eq!(reply.content, "hi from llama");
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
        .chat(&[Message::user("hi")])
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
        .chat(&[Message::user("hi")])
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
