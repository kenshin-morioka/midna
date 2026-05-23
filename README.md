# Midna

ローカルで動く、自分専用の AI エージェント。

推論はオフラインで完結する（[ADR 0002](docs/adr/0002-offline-first-with-browsing-exception.md)）。例外として、Web に情報を取りに行くツールにだけ将来的に外部通信を許可する。

## 前提

- macOS / Linux
- Rust (stable) — `rustup` 経由を推奨
- [Ollama](https://ollama.com/)（ローカル LLM ランタイム）

```sh
brew install ollama
ollama serve &              # バックグラウンドで起動
ollama pull llama3.1:8b     # モデル取得（初回のみ wifi 必要）
```

## セットアップ

```sh
cargo build
cargo test                  # 外部通信なし、wiremock で完結
```

## 使い方

```sh
cargo run -- chat
> こんにちは
```

`exit` / `quit` / `Ctrl+D` で終了。

オプション:

```sh
cargo run -- chat --model llama3.1:8b --host http://localhost:11434
MIDNA_MODEL=llama3.1:8b MIDNA_OLLAMA_HOST=http://localhost:11434 cargo run -- chat
cargo run -- --verbose chat
```

## ドキュメント

- [docs/design/midna_design_doc_v0_1.md](docs/design/midna_design_doc_v0_1.md)
- [docs/adr/0001-use-rust.md](docs/adr/0001-use-rust.md)
- [docs/adr/0002-offline-first-with-browsing-exception.md](docs/adr/0002-offline-first-with-browsing-exception.md)
- [docs/adr/0003-provider-abstraction-from-v0.md](docs/adr/0003-provider-abstraction-from-v0.md)
