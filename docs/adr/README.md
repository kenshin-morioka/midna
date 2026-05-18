# Architecture Decision Records (ADR)

このディレクトリには midna の設計判断 (Architecture Decision Records) を記録する。

## 方針

- 方針・トレードオフを伴う判断は ADR として残す。実装上の細かい選択は対象外。
- `docs/design/` 配下の design doc には判断の中身を書かず、ADR への相対リンクのみを置く。
- 1 つの判断につき 1 ファイル。

## 命名

`NNNN-kebab-case-title.md`

- `NNNN` は連番（ゼロ埋め 4 桁）。欠番は作らない。
- 既存 ADR を覆す場合は新規 ADR を起こし、旧 ADR の `Status` を `Superseded by NNNN` に更新する。

## フォーマット（MADR ライク）

```markdown
# NNNN - <タイトル>

- Status: Accepted | Superseded by NNNN | Deprecated
- Date: YYYY-MM-DD
- Deciders: <氏名>

## Context
（何を決めようとしているか、背景・制約）

## Decision
（採用する選択肢）

## Rationale
（なぜそれを選んだか）

## Consequences
- Positive: ...
- Negative / Trade-offs: ...

## Alternatives Considered
（検討した他案と却下理由）
```

## 一覧

- [0001 - 実装言語に Rust を採用](0001-use-rust.md)
- [0002 - オフライン動作を基本とし、ブラウジング系ツールのみ外部通信を許可](0002-offline-first-with-browsing-exception.md)
- [0003 - Provider 抽象を v0 から導入](0003-provider-abstraction-from-v0.md)
