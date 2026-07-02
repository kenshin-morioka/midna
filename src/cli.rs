use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "midna",
    version,
    about = "Local-first AI agent",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Enable verbose tracing output (debug level).
    #[arg(long, short, global = true)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Start an interactive chat REPL backed by a local LLM provider.
    Chat {
        /// Model name to use with the provider.
        #[arg(long, env = "MIDNA_MODEL", default_value = "llama3.1:8b")]
        model: String,

        /// Host URL of the Ollama-compatible runtime.
        #[arg(long, env = "MIDNA_OLLAMA_HOST", default_value = "http://localhost:11434")]
        host: String,
    },
}
