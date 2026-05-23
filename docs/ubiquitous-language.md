# ユビキタス言語

| 用語 | 説明 | 補足 | コード上の表現 |
| --- | --- | --- | --- |
| Midna | AI エージェント名 | ドキュメント本文では `Midna` | `midna` |
| Mode | Midna の動作モード | chat / code / image / video / multimodal | `src/modes/` |
| Tool | Midna が LLM に提供する機能 | filesystem / shell / web fetch / git / vision / media など | - |
| Agent | 自律的に動く処理単位 | - | - |
| Skill | 再利用可能な手順 | - | - |
| Command | ユーザーが定義する拡張コマンド | - | - |
| Prompt | 再利用可能なプロンプト | - | - |
| Session | 会話履歴の単位 | - | `Session` |
| Permission Policy | 権限ポリシー | allow / ask / deny | `Policy` |
| Artifact | Midna が生成する成果物 | - | - |
| Provider | LLM の提供元の抽象 | - | `Provider` |
| Memory | 永続化される記憶 | - | - |
