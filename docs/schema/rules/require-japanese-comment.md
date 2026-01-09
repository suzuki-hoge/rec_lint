# require_japanese_comment

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

コメントが英語のファイルを検出する

## NoEnglishCommentRule

コメントが英語のファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_japanese_comment` | o |  |
| comment | [commentConfig](./require-english-comment.md#commentconfig) | o | コメント構文設定 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

