# TextRule

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

特定のキーワードを含むファイルを完全一致で検出する

## TextRule

特定のキーワードを含むファイルを完全一致で検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| texts | string[] | o | validate で探す禁止キーワード |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

