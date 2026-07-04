use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use midna::cli::{Cli, Command};
use midna::modes;
use midna::permissions::Policy;
use midna::providers::ollama::OllamaProvider;
use midna::tools::ToolRegistry;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let provider = OllamaProvider::new(cli.host, cli.model)?;

    match cli.command.unwrap_or(Command::Agent) {
        Command::Agent => {
            let registry = ToolRegistry::builtin();
            let policy = Policy::new();
            modes::agent::run(&provider, &registry, &policy).await?;
        }
        Command::Chat => {
            modes::chat::run(&provider).await?;
        }
    }

    Ok(())
}
