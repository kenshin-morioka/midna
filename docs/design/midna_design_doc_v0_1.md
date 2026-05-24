# 背景

自分専用に手元で動かせる AI エージェントが欲しい。Claude Code のような体験を、特定の提供元（プロバイダ）に縛られず、推論はオフラインで完結する形で再構成したい。最初の一歩として v0 を立ち上げる。

# 目的

- 手元の LLM ランタイム（Ollama 等）と対話できる CLI を作る
- 推論はオフラインで完結させる（例外は [ADR 0002](../adr/0002-offline-first-with-browsing-exception.md)）
- 将来の拡張（tools / 他モード / 他 provider）を阻害しない構造で始める

# スコープ

## ゴール

- `midna chat` で Ollama 経由のローカル LLM と対話できる
- Rust の Cargo project として scaffold する（[ADR 0001](../adr/0001-use-rust.md)）
- Provider 抽象を v0 から入れる（[ADR 0003](../adr/0003-provider-abstraction-from-v0.md)）
- 設計判断は ADR に残す

## ノンゴール

- chat 以外のモード（code / image / video / multimodal）
- tools（filesystem / shell / git / vision / media / network）の実装
- agents / skills / commands / prompts の仕組み
- Ollama 以外の provider（Anthropic / OpenAI / llama.cpp 直接 等）
- ストリーミング応答
- 並列エージェント実行ランタイム本体（async/tokio の足場だけ用意）
- メモリ永続化、artifacts、router
- ブラウジング系ツール本体（[ADR 0002](../adr/0002-offline-first-with-browsing-exception.md) で方針のみ）

# ユビキタス言語

[ユビキタス言語](../ubiquitous-language.md)

# 実装方針

## DB 設計

なし

## エンドポイント設計

なし

## エンドポイント以外の設計

- 全体構成（モジュール）
  ```mermaid
  flowchart TD
      CLI[CLI / clap]
      Session[Session]
      ChatMode[modes::chat]
      Provider[Provider trait]
      Ollama[OllamaProvider]
      Error[MidnaError]
      Perm[Permissions / Policy]

      CLI --> ChatMode
      ChatMode --> Session
      ChatMode --> Provider
      Provider --> Ollama
      Ollama --> Error
      ChatMode --> Perm
  ```
- `Provider` trait（`async fn chat(&[Message]) -> Result<Message, MidnaError>`、`Send + Sync`）
- `OllamaProvider` が `reqwest` で `/api/chat` を呼ぶ。接続失敗は `MidnaError::Connect` に変換して親切なメッセージを返す
- `Session` は `Vec<Message>` を保持し `push` / `pop` / `messages` を提供
- `modes::chat::run` は stdin を 1 行ずつ読んで provider に投げる REPL。`exit` / `quit` / EOF で終了
- `MidnaError`（thiserror）でライブラリ境界のエラーを公開
- `Permissions` は `Allow` / `Ask` / `Deny` の enum と固定 `Allow` を返す `Policy`（tools 導入時に拡張）
- `Cli`（clap derive）で `midna chat [--model] [--host] [--verbose]`、env は `MIDNA_MODEL` / `MIDNA_OLLAMA_HOST`
- 主要な依存: tokio (full), reqwest (rustls-tls), serde, serde_json, clap, anyhow, thiserror, async-trait, tracing。dev に wiremock

## スクリプト

なし

## 外部ツール

- [Ollama](https://ollama.com/) — 手元の LLM ランタイム。`ollama serve` を起動し `ollama pull llama3.1:8b` 等でモデルを取得しておく
- Cargo（Rust toolchain）— ビルド・テスト・実行
- 将来: Web 取得・Web 検索のための外部 API（[ADR 0002](../adr/0002-offline-first-with-browsing-exception.md) の例外として通信を許可）

# 懸念点

- 初回のモデル取得と Ollama 導入はネット接続が必要。完全オフライン環境では事前準備が要る
- クラウドの最新の高性能モデルに追従するには、後で明示的な切り替え機構を足す必要がある（v0 ではスコープ外）
- `async_trait` は v0 で使うが、trait の async fn が言語標準で安定したら剥がす
- ブラウジング系ツールを実装するときは、許可する通信先のルールを別途決める（権限の仕組み側で扱う）
- v0 では Ctrl+C のシグナル処理を実装しない（`exit` / `quit` / EOF で抜ける運用）
