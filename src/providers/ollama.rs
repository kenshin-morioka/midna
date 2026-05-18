use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::MidnaError;
use crate::providers::{Message, Provider};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120);

pub struct OllamaProvider {
    host: String,
    model: String,
    http: reqwest::Client,
}

impl OllamaProvider {
    pub fn new(host: impl Into<String>, model: impl Into<String>) -> Result<Self, MidnaError> {
        let http = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .build()?;
        Ok(Self {
            host: host.into(),
            model: model.into(),
            http,
        })
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn host(&self) -> &str {
        &self.host
    }
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
    stream: bool,
}

#[derive(Deserialize)]
struct ChatResponse {
    message: Message,
}

#[async_trait]
impl Provider for OllamaProvider {
    async fn chat(&self, messages: &[Message]) -> Result<Message, MidnaError> {
        let url = format!("{}/api/chat", self.host.trim_end_matches('/'));
        let body = ChatRequest {
            model: &self.model,
            messages,
            stream: false,
        };

        let resp = self.http.post(&url).json(&body).send().await.map_err(|e| {
            if e.is_connect() || e.is_timeout() {
                MidnaError::Connect(format!(
                    "{} ({}). Is `ollama serve` running and reachable?",
                    e, url
                ))
            } else {
                MidnaError::Http(e)
            }
        })?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(MidnaError::Provider(format!(
                "Ollama returned {}: {}. Have you pulled the model `{}` (try `ollama pull {}`)?",
                status, text, self.model, self.model
            )));
        }

        let parsed: ChatResponse = resp.json().await?;
        Ok(parsed.message)
    }

    fn name(&self) -> &'static str {
        "ollama"
    }
}
