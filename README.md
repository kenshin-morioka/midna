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
cargo install --path .       # `midna` を PATH に入れる
```

## 使い方

```sh
midna
> このプロジェクトの構成を教えて
```

引数なしで起動するとエージェントモード。ファイルの読み書き・ディレクトリ一覧・シェルコマンド実行ができ、ファイル書き込みとコマンド実行は実行前に y/N で確認される（[ADR 0005](docs/adr/0005-risk-based-permissions.md)）。tool calling 対応モデル（llama3.1 / qwen2.5 など）が必要。

ツールなしの対話だけしたいときは `midna chat`。

`exit` / `quit` / `Ctrl+D` で終了。

オプション:

```sh
midna --model llama3.1:8b --host http://localhost:11434
MIDNA_MODEL=llama3.1:8b MIDNA_OLLAMA_HOST=http://localhost:11434 midna
midna --verbose
```

## 開発

```sh
cargo build
cargo test                  # 外部通信なし、wiremock で完結
cargo run                   # インストール前に試す
```

## ドキュメント

- [docs/design/](docs/design/) — 設計ドキュメント
- [docs/adr/](docs/adr/) — 設計判断の記録 (ADR)
