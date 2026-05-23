# ユビキタス言語

このプロジェクトで使う用語の正式表記と意味を残す。新しい用語が出てきたら追記する。

| 用語 | 説明 | 補足 | コード上の表現 |
| --- | --- | --- | --- |
| Midna | このプロジェクト | ドキュメント本文では `Midna` と表記する | `midna`（パッケージ名 / CLI コマンド名 / 識別子 / URL / ファイル名） |
| ADR | Architecture Decision Record。設計判断を 1 件ずつ残す形式 | `docs/adr/NNNN-kebab.md` に置く | - |
| design doc | 機能や改修の作業単位の設計ドキュメント | `docs/design/<name>.md` に置く | - |
| Provider | LLM の提供元の抽象 | - | `Provider` trait（`src/providers/mod.rs`） |
| 手元 / ローカル | ユーザーのマシン上で動くこと | wifi 不要で完結することを含意 | - |
