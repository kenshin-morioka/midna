use std::io::{self, Write};

use tokio::io::{AsyncBufReadExt, BufReader, Lines, Stdin};

use crate::error::MidnaError;
use crate::permissions::{Permission, Policy};
use crate::providers::{Message, Provider, Role, ToolCall};
use crate::session::Session;
use crate::tools::ToolRegistry;

const SYSTEM_PROMPT: &str = "You are Midna, a local-first coding agent running on the user's machine. \
You can read and write files, list directories, and run shell commands in the current working directory via tools. \
Use tools to inspect the project before answering questions about it, and to carry out coding tasks the user asks for. \
Prefer small, verifiable steps: read before you write, and run relevant checks after changing files. \
Be concise.";

/// 1 ユーザー入力あたりのツール実行往復の上限。
/// モデルがツールを呼び続けて無限ループするのを防ぐ。
const MAX_STEPS: usize = 16;

pub async fn run<P: Provider>(
    provider: &P,
    registry: &ToolRegistry,
    policy: &Policy,
) -> Result<(), MidnaError> {
    let mut session = Session::with_system_prompt(SYSTEM_PROMPT);
    let specs = registry.specs();

    println!(
        "midna agent (provider: {}). Type 'exit' or Ctrl+D to quit.",
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

        // エラー時にこのターン全体を巻き戻すための基準点
        let checkpoint = session.len();
        session.push(Message::user(trimmed));

        if let Err(err) = run_turn(provider, registry, policy, &specs, &mut session, &mut reader).await
        {
            eprintln!("error: {err}");
            // 失敗したターンの履歴（user 発話やツール往復の途中経過）を
            // 次ターンに引き継ぐと壊れた文脈で会話が続くため、丸ごと巻き戻す
            session.truncate(checkpoint);
        }
    }

    Ok(())
}

/// 1 ユーザー入力に対する「LLM 応答 → ツール実行 → 再問い合わせ」のループ
async fn run_turn<P: Provider>(
    provider: &P,
    registry: &ToolRegistry,
    policy: &Policy,
    specs: &[crate::providers::ToolSpec],
    session: &mut Session,
    reader: &mut Lines<BufReader<Stdin>>,
) -> Result<(), MidnaError> {
    for _ in 0..MAX_STEPS {
        let reply = provider.chat(session.messages(), specs).await?;
        if reply.role != Role::Assistant {
            return Err(MidnaError::Provider(
                "provider returned a non-assistant reply".to_string(),
            ));
        }

        let tool_calls = reply.tool_calls.clone().unwrap_or_default();
        session.push(reply.clone());

        if tool_calls.is_empty() {
            println!("{}\n", reply.content);
            return Ok(());
        }

        for call in &tool_calls {
            let result = execute_tool_call(registry, policy, call, reader).await;
            session.push(Message::tool(result));
        }
    }

    Err(MidnaError::Provider(format!(
        "agent did not finish within {MAX_STEPS} tool steps"
    )))
}

/// ツール呼び出し 1 件を権限確認込みで実行する。
/// 失敗や拒否もエラーにせず文字列で返し、LLM が続きを判断できるようにする。
async fn execute_tool_call(
    registry: &ToolRegistry,
    policy: &Policy,
    call: &ToolCall,
    reader: &mut Lines<BufReader<Stdin>>,
) -> String {
    let name = &call.function.name;
    let args = &call.function.arguments;

    let Some(tool) = registry.get(name) else {
        return format!("error: unknown tool `{name}`");
    };

    println!("⚙ {name}({args})");

    match policy.check(tool.risk()) {
        Permission::Allow => {}
        Permission::Deny => return format!("error: tool `{name}` is denied by policy"),
        Permission::Ask => match confirm(reader).await {
            Ok(true) => {}
            Ok(false) => return format!("error: user denied permission to run `{name}`"),
            Err(err) => return format!("error: failed to read confirmation: {err}"),
        },
    }

    match tool.call(args).await {
        Ok(output) => output,
        Err(err) => format!("error: {err}"),
    }
}

async fn confirm(reader: &mut Lines<BufReader<Stdin>>) -> Result<bool, MidnaError> {
    print!("  実行を許可しますか? [y/N] ");
    io::stdout().flush().ok();
    let answer = reader.next_line().await?.unwrap_or_default();
    Ok(matches!(answer.trim(), "y" | "Y" | "yes"))
}
