# require_php_doc

[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)

PHPDoc がないファイルを検出する

## NoPhpDocRule

PHPDoc がないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_php_doc` | o |  |
| option | [phpDocConfig](#phpdocconfig) | - | PHPDoc 検証設定 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](./common.md#matchitem)[] | - | show と validation で対象とするファイルの条件<br>複数指定時は and で結合 |

## PhpDocConfig

PHPDoc 検証設定の定義<br>いずれかひとつは指定が必要<br>サポート対象外: file, property, constant, define, include/require

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| class | [visibility](./common.md#visibility) | - | class 宣言に PHPDoc がないことを認めない |
| interface | [visibility](./common.md#visibility) | - | interface 宣言に PHPDoc がないことを認めない |
| trait | [visibility](./common.md#visibility) | - | trait 宣言に PHPDoc がないことを認めない |
| enum | [visibility](./common.md#visibility) | - | enum 宣言に PHPDoc がないことを認めない (PHP 8.1+) |
| function | [visibility](./common.md#visibility) | - | 関数/メソッド宣言に PHPDoc がないことを認めない |

