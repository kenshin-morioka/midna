# 0003 - Provider 抽象を v0 から導入

- Status: Accepted
- Date: 2026-05-18
- Deciders: kenshin-morioka

## Context

v0 では LLM プロバイダとして Ollama (`/api/chat`) 1 つしか使わない予定。「実装は 1 つしかないのに最初から抽象化するのはオーバーエンジニアリングではないか」という選択肢と、「将来別ランタイムに乗り換えたい意図が既にあるので最初から抽象を入れる」という選択肢のどちらを採るかが論点。

ユーザーは「Ollama より良いオープンモデル/ランタイムが出てきたら乗り換えたい」と既に意思表示しており、provider ロックインを避けたい意向が言語選定時にも表明されている（ADR 0001 参照）。

## Decision

**v0 から `Provider` trait を入れる。** Ollama 実装はこの trait を介して呼び出す。

```rust
#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    async fn chat(&self, messages: &[Message]) -> Result<Message, MidnaError>;
    fn name(&self) -> &'static str;
}
```

`Message { role: Role, content: String }` を共通インターフェイスとする。

## Rationale

- ユーザーが将来別ランタイムへ乗り換える意向を明示している。後から抽象化を入れる手戻りより、最初から trait を切る方が痛みが小さい。
- テスタビリティが上がる（mock provider を実装して session/CLI のテストができる）。
- 抽象のコストが小さい — Rust の trait + `async_trait` で記述量はごく少ない。
- 並列エージェント稼働を見据えると、provider が `Send + Sync` であることを型レベルで保証しておく価値が高い。

## Consequences

- Positive
  - 別 provider（llama.cpp 直接、Anthropic、OpenAI、ローカル別ランタイム）の追加が新ファイル 1 つで済む
  - `Send + Sync` 制約により並列実行の安全性が型で担保される
  - テスト容易性が高い（mock を差し込める）
- Negative / Trade-offs
  - 1 実装しかない時点での抽象化は YAGNI に見える可能性がある（オーバーエンジニアリング批判）
  - `async_trait` クレートを v0 から依存に入れる（trait の async fn が安定化したら剥がす予定）

## Alternatives Considered

- **Ollama 直書き** — `OllamaClient` を session が直接呼ぶ最小構成。コードは短くなるが、provider 切り替え時に session / cli / tests を広範囲に書き換える必要が出るため却下。
- **複数 provider を v0 から実装（Ollama + Anthropic 等）** — 抽象を正当化する材料は揃うが v0 スコープ（最小 chat ループ）を逸脱するため却下。抽象は入れて実装は 1 つだけにする中庸を選んだ。
