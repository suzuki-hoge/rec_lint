# require_kotlin_doc

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

KDoc がないファイルを検出する

## NoKotlinDocRule

KDoc がないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_kotlin_doc` | o |  |
| kotlin_doc | [kotlinDocConfig](#kotlindocconfig) | - | KDoc 検証設定 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

## KotlinDocConfig

KDoc 検証設定の定義<br>いずれかひとつは指定が必要<br>サポート対象外: property, constructor, enum_entry, companion_object

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| class | [visibility](./common.md#visibility) | - | class 宣言に KDoc がないことを認めない |
| interface | [visibility](./common.md#visibility) | - | interface 宣言に KDoc がないことを認めない |
| object | [visibility](./common.md#visibility) | - | object 宣言に KDoc がないことを認めない |
| enum_class | [visibility](./common.md#visibility) | - | enum class 宣言に KDoc がないことを認めない |
| sealed_class | [visibility](./common.md#visibility) | - | sealed class 宣言に KDoc がないことを認めない |
| sealed_interface | [visibility](./common.md#visibility) | - | sealed interface 宣言に KDoc がないことを認めない |
| data_class | [visibility](./common.md#visibility) | - | data class 宣言に KDoc がないことを認めない |
| value_class | [visibility](./common.md#visibility) | - | value class 宣言に KDoc がないことを認めない |
| annotation_class | [visibility](./common.md#visibility) | - | annotation class 宣言に KDoc がないことを認めない |
| typealias | [visibility](./common.md#visibility) | - | typealias 宣言に KDoc がないことを認めない |
| function | [visibility](./common.md#visibility) | - | fun 宣言に KDoc がないことを認めない |

