# NoJapaneseCommentRule

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

コメントが日本語のファイルを検出する

## NoJapaneseCommentRule

コメントが日本語のファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| format | [commentConfig](#commentconfig) | o | コメント構文設定 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

## CommentConfig

コメント構文の定義<br>lang と custom のどちらかのみ必ず指定する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| lang | `java` \|<br>`kotlin` \|<br>`rust` | - | コメント構文のプリセットが提供されている言語 |
| custom | [customComment](#customcomment) | - | プリセットが提供されていない場合の指定 |

## CustomComment

コメント構文の定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| lines | string[] | - | 行コメントの開始マーカー<br>e.g. `//`<br>e.g. `#` |
| blocks | [blockComment](#blockcomment)[] | - | ブロックコメント構文の定義 |

## BlockComment

ブロックコメント構文の定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| start | string | o | ブロックコメントの開始マーカー<br>e.g. `/*` |
| end | string | o | ブロックコメントの終了マーカー<br>e.g. `*/` |

