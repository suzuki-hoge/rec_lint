# forbidden_patterns

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

特定のキーワードを含むファイルを正規表現で検出する

## RegexRule

特定のキーワードを含むファイルを正規表現で検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `forbidden_patterns` | o |  |
| patterns | string[] | o | validate で探す禁止キーワードの正規表現 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

