# require_kotest_test

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

Kotest テストファイルの存在を検証する

## KotestTestRule

Kotest テストファイルの存在を検証する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_kotest_test` | o |  |
| option | [externalFileTestConfig](./common.md#externalfiletestconfig) | - | Kotest テスト存在検証設定<br>test_directory のデフォルトは src/test/kotlin |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

