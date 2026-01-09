# require_japanese_phpunit_test_name

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

PHPUnit テスト名が日本語でないファイルを検出する

## JapanesePhpUnitTestNameRule

PHPUnit テスト名が日本語でないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_japanese_phpunit_test_name` | o |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

