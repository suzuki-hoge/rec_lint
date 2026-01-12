# RustUnitTestRule

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

Rust ユニットテストの存在を検証する

## RustUnitTestRule

Rust ユニットテストの存在を検証する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| option | [sameFileTestConfig](./common.md#samefiletestconfig) | - | Rust ユニットテスト存在検証設定 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

