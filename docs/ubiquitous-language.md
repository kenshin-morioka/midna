# ユビキタス言語

このプロジェクトで使う用語の正式表記と意味を残す。新しい用語が出てきたら追記する。

| 用語 | 意味 |
| --- | --- |
| Midna / midna | このプロジェクト。ドキュメント本文では `Midna`、コード（識別子、`Cargo.toml` のパッケージ名、CLI コマンド名、URL、ファイル名）では `midna` |
| ADR | Architecture Decision Record。設計判断を 1 件ずつ残す形式。`docs/adr/NNNN-kebab.md` |
| design doc | 機能や改修の作業単位の設計ドキュメント。`docs/design/<name>.md` |
| Provider | LLM の提供元の抽象。`src/providers/mod.rs` で trait として定義 |
| 手元 / ローカル | ユーザーのマシン上で動くこと。wifi 不要で完結することを含意 |
