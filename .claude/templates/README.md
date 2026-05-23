# テンプレ集

新しい設計ドキュメントを書き始めるときに使う雛形。`<...>` の placeholder を埋めて利用する。

## 一覧

- [`adr.md`](adr.md) — 設計判断 1 件を残すための雛形（`docs/adr/NNNN-kebab.md` 用）
- [`design-doc.md`](design-doc.md) — design doc 全体の骨格（`docs/design/<name>.md` 用）

## 使い方

1. テンプレを目的の場所にコピーする
   ```sh
   cp .claude/templates/adr.md docs/adr/0004-<kebab-title>.md
   cp .claude/templates/design-doc.md docs/design/<name>_v0_1.md
   ```
2. `<...>` の placeholder を順に埋める
3. ADR は `docs/adr/README.md` の一覧にもリンクを追加する

## 書くときの方針

- 文章は日本語、普段使わない専門用語っぽいカタカナ語（ロックイン、エコシステム、フロンティア、ペイン、YAGNI、オーバーエンジニアリング 等）は避ける
- 本人視点で書く（「ユーザーは○○と明示」のような第三者目線にしない）
- ADR の「他に検討した案」には本気で比較した案だけを書く
- 詳しい判断は ADR、概観は design doc。design doc から ADR への一方向リンクで繋ぐ
