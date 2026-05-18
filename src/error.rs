use thiserror::Error;

#[derive(Debug, Error)]
pub enum MidnaError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("could not connect to LLM provider: {0}")]
    Connect(String),

    #[error("provider error: {0}")]
    Provider(String),
}
