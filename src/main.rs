use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use midna::cli::{Cli, Command};
use midna::modes;
use midna::providers::ollama::OllamaProvider;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    match cli.command {
        Command::Chat { model, host } => {
            let provider = OllamaProvider::new(host, model)?;
            modes::chat::run(&provider).await?;
        }
    }

    Ok(())
}
