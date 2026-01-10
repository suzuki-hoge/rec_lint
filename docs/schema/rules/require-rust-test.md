# require_rust_test

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

Rust テストの存在を検証する

## RustTestRule

Rust テストの存在を検証する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_rust_test` | o |  |
| rust_test | [rustTestConfig](#rusttestconfig) | - | Rust テスト存在検証設定 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

## RustTestConfig

Rust テスト存在検証設定

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| unit | [rustUnitTestConfig](#rustunittestconfig) | - | ユニットテスト設定 |

## RustUnitTestConfig

Rust ユニットテスト設定

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| require | [testRequireLevelSameFile](./common.md#testrequirelevelsamefile) | - | 検証レベル |

