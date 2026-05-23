# 設計判断の記録 (ADR)

設計に関わる **判断とその理由** を 1 件ずつ残すディレクトリ。書き方や運用ルールは [`CLAUDE.md`](../../CLAUDE.md) を参照。

## ファイル名

`NNNN-kebab-case-title.md`

- `NNNN` は 4 桁の連番。欠番は作らない。
- 既に決めたことを覆すときは新しい記録を起こし、古いほうの「状態」を `Superseded by NNNN` に書き換える。

## 雛形

`.claude/templates/adr.md` をコピーして使う。

