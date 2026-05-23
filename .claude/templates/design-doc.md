# <プロジェクト名> Design Document v<X.Y>

## Overview

<このシステムが何を目指すのか、どんな課題を解こうとするのかを 3〜5 行で>

<最終的に目指す像を 1 行のキャッチで>

---

# Core Philosophy

## 1. <方針 1 のタイトル>

<その方針の中身を簡潔に。詳細な引き換えや判断根拠は ADR 側に書き、ここからリンクする>

詳細は [ADR NNNN](../adr/NNNN-...md) を参照。

---

## 2. <方針 2 のタイトル>

<同上>

---

## 3. <方針 3 のタイトル>

<同上>

---

# High-Level Architecture

```txt
            ┌─────────────────┐
            │      CLI        │
            └────────┬────────┘
                     │
            ┌────────▼────────┐
            │  Session Layer  │
            └────────┬────────┘
                     │
        （以下、レイヤ図を続ける）
```

---

# Repository Structure

<実装スタックに合わせたディレクトリ構成を書く。スタック選定は ADR 側へリンクする>

```txt
<project>/
├── ...
```

`(future)` の付いたモジュールは現バージョン未実装。実際に必要になったタイミングで追加する。

---

# Permission System

<権限の仕組みが必要なら書く。なければセクションごと削る>

| 状態 | 意味 |
| --- | --- |
| allow | 自動で実行 |
| ask | 都度確認 |
| deny | 禁止 |

例:

```yaml
permissions:
  read_file: allow
  write_file: ask
  shell: ask
```

---

# CLI Examples

```bash
<project> <subcommand>
```

---

# Final Goal

<最終的に到達したい像を引用形式で 1〜2 行>

> <ビジョンを 1 文で>

---

# 設計判断の記録

設計に関わる判断は `docs/adr/` 配下に 1 件ずつ残す。この design doc には判断の中身を書かず、リンクだけ置く方針。

主な記録:

- [ADR 0001 - <タイトル>](../adr/0001-...md)
- [ADR 0002 - <タイトル>](../adr/0002-...md)

記録のやり方は [docs/adr/README.md](../adr/README.md) を参照。
