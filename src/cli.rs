use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "midna",
    version,
    about = "Local-first AI agent",
    long_about = None
)]
pub struct Cli {
    /// 省略時はエージェントモード（ツール実行あり）で起動する
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Model name to use with the provider.
    #[arg(
        long,
        env = "MIDNA_MODEL",
        default_value = "llama3.1:8b",
        global = true
    )]
    pub model: String,

    /// Host URL of the Ollama-compatible runtime.
    #[arg(
        long,
        env = "MIDNA_OLLAMA_HOST",
        default_value = "http://localhost:11434",
        global = true
    )]
    pub host: String,

    /// Enable verbose tracing output (debug level).
    #[arg(long, short, global = true)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Start an agent REPL that can read/write files and run shell commands (default).
    Agent,

    /// Start a plain chat REPL without tools.
    Chat,
}
