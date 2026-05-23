# ユビキタス言語

このプロジェクトで使う用語の **正式表記** と意味を残す。新しい用語が出てきたら追記する。

## プロジェクト名

| 文脈 | 表記 |
| --- | --- |
| ドキュメント本文・固有名詞として使うとき | **Midna** |
| コード（識別子、`Cargo.toml` のパッケージ名、CLI コマンド名、URL、ファイル名） | midna |

例:
- ドキュメント: 「Midna は手元の LLM ランタイムと対話する CLI ツール」
- コード: `cargo run -- chat`、`name = "midna"`、`use midna::cli::Cli`

## その他の用語

| 用語 | 意味 |
| --- | --- |
| ADR (Architecture Decision Record) | 設計判断を 1 件ずつ残す形式。`docs/adr/NNNN-kebab.md` |
| design doc | 機能や改修の作業単位の設計ドキュメント。`docs/design/<name>.md` |
| Provider | LLM の提供元の抽象。`src/providers/mod.rs` で trait として定義 |
| 手元 / ローカル | ユーザーのマシン上で動くこと。wifi 不要で完結することを含意 |
