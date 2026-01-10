# require_rust_doc

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

rustdoc がないファイルを検出する

## NoRustDocRule

rustdoc がないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_rust_doc` | o |  |
| option | [rustDocConfig](#rustdocconfig) | - | rustdoc 検証設定 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

## RustDocConfig

rustdoc 検証設定の定義<br>いずれかひとつは指定が必要<br>サポート対象外: const, static, struct_field, enum_variant, impl, trait_impl

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| struct | [visibility](./common.md#visibility) | - | struct 宣言に rustdoc がないことを認めない |
| enum | [visibility](./common.md#visibility) | - | enum 宣言に rustdoc がないことを認めない |
| trait | [visibility](./common.md#visibility) | - | trait 宣言に rustdoc がないことを認めない |
| type_alias | [visibility](./common.md#visibility) | - | type 宣言に rustdoc がないことを認めない |
| union | [visibility](./common.md#visibility) | - | union 宣言に rustdoc がないことを認めない |
| fn | [visibility](./common.md#visibility) | - | fn 宣言に rustdoc がないことを認めない |
| macro_rules | [visibility](./common.md#visibility) | - | macro_rules! 宣言に rustdoc がないことを認めない |
| mod | [visibility](./common.md#visibility) | - | mod 宣言に rustdoc がないことを認めない |

