use std::io::{self, Write};

use tokio::io::{AsyncBufReadExt, BufReader};

use crate::error::MidnaError;
use crate::providers::{Message, Provider};
use crate::session::Session;

const SYSTEM_PROMPT: &str = "You are Midna, a local-first AI assistant running on the user's machine. Be concise and helpful.";

pub async fn run<P: Provider>(provider: &P) -> Result<(), MidnaError> {
    let mut session = Session::with_system_prompt(SYSTEM_PROMPT);

    println!(
        "midna chat (provider: {}). Type 'exit' or Ctrl+D to quit.",
        provider.name()
    );

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    loop {
        print!("> ");
        // best-effort: prompt未flushでもユーザー入力自体は受け付けられるため失敗は無視
        io::stdout().flush().ok();

        let line = match reader.next_line().await? {
            Some(line) => line,
            None => {
                println!();
                break;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if matches!(trimmed, "exit" | "quit") {
            break;
        }

        session.push(Message::user(trimmed));

        match provider.chat(session.messages()).await {
            Ok(reply) => {
                println!("{}\n", reply.content);
                session.push(reply);
            }
            Err(err) => {
                eprintln!("error: {err}");
                // 直前のユーザー発話を巻き戻し、次ターンが「失敗した質問」を引き継がないようにする
                session.pop();
            }
        }
    }

    Ok(())
}
