# .rec_lint.yaml ドキュメント

## トップレベル

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| rule | [ruleItem](#ruleitem)[] | - | 特定パターンを禁止するルール<br>show: 表示される<br>validate: 検証される<br>guideline: 表示されない |
| guideline | [guidelineItem](#guidelineitem)[] | - | レビューガイドライン<br>show: 表示される<br>validate: 検証されない<br>guideline: 表示される |

## RuleItem

rule に列挙するルールの定義<br>type ごとに異なる構造を持つ<br>- type: forbidden_texts<br>- type: forbidden_patterns<br>- type: custom<br>- type: require_java_doc<br>- type: require_kotlin_doc<br>- type: require_rust_doc<br>- type: require_english_comment<br>- type: require_japanese_comment

### TextRule

特定のキーワードを含むファイルを完全一致で検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `forbidden_texts` | o |  |
| keywords | string[] | o | validate で探す禁止キーワード |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | show と validation で対象とするファイルのマッチ条件<br>複数指定時は and で結合 |

### RegexRule

特定のキーワードを含むファイルを正規表現で検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `forbidden_patterns` | o |  |
| keywords | string[] | o | validate で探す禁止キーワードの正規表現 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | show と validation で対象とするファイルのマッチ条件<br>複数指定時は and で結合 |

### CustomRule

任意のコマンドを実行してファイルを検証する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `custom` | o |  |
| exec | string | o | ファイルに対して実行するコマンド<br>終了コード 0 の場合はエラーなし扱い<br>エラー時は実行コマンドの標準出力がエラーメッセージとして表示される<br>e.g. `ruby path/to/your/checker.rb {path}`<br>e.g. `docker run your:image {path}` |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | show と validation で対象とするファイルのマッチ条件<br>複数指定時は and で結合 |

### NoJavaDocRule

JavaDoc がないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_java_doc` | o |  |
| java_doc | [javaDocConfig](#javadocconfig) | - |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | show と validation で対象とするファイルのマッチ条件<br>複数指定時は and で結合 |

### NoKotlinDocRule

KDoc がないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_kotlin_doc` | o |  |
| kotlin_doc | [kotlinDocConfig](#kotlindocconfig) | - |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | show と validation で対象とするファイルのマッチ条件<br>複数指定時は and で結合 |

### NoRustDocRule

rustdoc がないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_rust_doc` | o |  |
| rust_doc | [rustDocConfig](#rustdocconfig) | - |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | show と validation で対象とするファイルのマッチ条件<br>複数指定時は and で結合 |

### NoJapaneseCommentRule

コメントが日本語のファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_english_comment` | o |  |
| comment | [commentConfig](#commentconfig) | o |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | show と validation で対象とするファイルのマッチ条件<br>複数指定時は and で結合 |

### NoEnglishCommentRule

コメントが英語のファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `require_japanese_comment` | o |  |
| comment | [commentConfig](#commentconfig) | o |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | show と validation で対象とするファイルのマッチ条件<br>複数指定時は and で結合 |

## GuidelineItem

guideline に列挙するレビューガイドラインの定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| message | string | o | guideline で表示するメッセージ |
| match | [matchItem](#matchitem)[] | - | 対象とするファイルのマッチ条件<br>複数指定時は and で結合 |

## MatchItem

ファイルマッチ条件の定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| pattern | [matchPattern](#matchpattern) | o |  |
| keywords | string[] | o | マッチ対象のキーワード |
| cond | [matchCond](#matchcond) | - | 省略時は and |

## MatchPattern

マッチパターンの種類

| 値 | 説明 |
|----|------|
| `file_starts_with` | ファイル名が指定文字列で始まる |
| `file_ends_with` | ファイル名が指定文字列で終わる |
| `path_contains` | パスに指定文字列が含まれる |
| `file_not_starts_with` | ファイル名が指定文字列で始まらない |
| `file_not_ends_with` | ファイル名が指定文字列で終わらない |
| `path_not_contains` | パスに指定文字列が含まれない |

## MatchCond

keywords の結合条件

| 値 | 説明 |
|----|------|
| `and` | すべての keyword に一致 |
| `or` | いずれかの keyword に一致 |

## JavaDocConfig

JavaDoc 検証設定の定義<br>いずれかひとつは指定が必要<br>サポート対象外: constructor, field, enum_constant

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| class | [visibility](#visibility) | - | class 宣言に JavaDoc がないことを認めない |
| interface | [visibility](#visibility) | - | interface 宣言に JavaDoc がないことを認めない |
| enum | [visibility](#visibility) | - | enum 宣言に JavaDoc がないことを認めない |
| record | [visibility](#visibility) | - | record 宣言に JavaDoc がないことを認めない |
| annotation | [visibility](#visibility) | - | @interface 宣言に JavaDoc がないことを認めない |
| method | [visibility](#visibility) | - | メソッド宣言に JavaDoc がないことを認めない |

## KotlinDocConfig

KDoc 検証設定の定義<br>いずれかひとつは指定が必要<br>サポート対象外: property, constructor, enum_entry, companion_object

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| class | [visibility](#visibility) | - | class 宣言に KDoc がないことを認めない |
| interface | [visibility](#visibility) | - | interface 宣言に KDoc がないことを認めない |
| object | [visibility](#visibility) | - | object 宣言に KDoc がないことを認めない |
| enum_class | [visibility](#visibility) | - | enum class 宣言に KDoc がないことを認めない |
| sealed_class | [visibility](#visibility) | - | sealed class 宣言に KDoc がないことを認めない |
| sealed_interface | [visibility](#visibility) | - | sealed interface 宣言に KDoc がないことを認めない |
| data_class | [visibility](#visibility) | - | data class 宣言に KDoc がないことを認めない |
| value_class | [visibility](#visibility) | - | value class 宣言に KDoc がないことを認めない |
| annotation_class | [visibility](#visibility) | - | annotation class 宣言に KDoc がないことを認めない |
| typealias | [visibility](#visibility) | - | typealias 宣言に KDoc がないことを認めない |
| function | [visibility](#visibility) | - | fun 宣言に KDoc がないことを認めない |

## RustDocConfig

rustdoc 検証設定の定義<br>いずれかひとつは指定が必要<br>サポート対象外: const, static, struct_field, enum_variant, impl, trait_impl

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| struct | [visibility](#visibility) | - | struct 宣言に rustdoc がないことを認めない |
| enum | [visibility](#visibility) | - | enum 宣言に rustdoc がないことを認めない |
| trait | [visibility](#visibility) | - | trait 宣言に rustdoc がないことを認めない |
| type_alias | [visibility](#visibility) | - | type 宣言に rustdoc がないことを認めない |
| union | [visibility](#visibility) | - | union 宣言に rustdoc がないことを認めない |
| fn | [visibility](#visibility) | - | fn 宣言に rustdoc がないことを認めない |
| macro_rules | [visibility](#visibility) | - | macro_rules! 宣言に rustdoc がないことを認めない |
| mod | [visibility](#visibility) | - | mod 宣言に rustdoc がないことを認めない |

## CommentConfig

コメント構文の定義<br>lang と custom のどちらかのみ必ず指定する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| lang | `java` \|<br>`kotlin` \|<br>`rust` | - | コメント構文のプリセットが提供されている言語 |
| custom | [customComment](#customcomment) | - | プリセットが提供されていない場合の指定 |

## Visibility

Doc コメントを強制する対象の可視性

| 値 | 説明 |
|----|------|
| `public` | その言語における public のコードのみ検証 |
| `all` | すべての可視性を検証 |

## CustomComment

コメント構文の定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| lines | string[] | - | 行コメントの開始マーカー<br>e.g. `//`<br>e.g. `#` |
| blocks | [blockComment](#blockcomment)[] | - | ブロックコメント構文の定義 |

## BlockComment

ブロックコメント構文の定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| start | string | o | ブロックコメントの開始マーカー<br>e.g. `/*` |
| end | string | o | ブロックコメントの終了マーカー<br>e.g. `*/` |

