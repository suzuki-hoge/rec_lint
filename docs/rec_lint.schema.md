# rec_lint.yaml ドキュメント

## トップレベル

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| deny | [rule](#rule)[] | - | 特定パターンを禁止するルール<br>show: 表示される<br>validate: 検証される<br>review: 表示されない |
| review | [reviewItem](#reviewitem)[] | - | レビューガイドライン<br>show: 表示される<br>validate: 検証されない<br>review: 表示される |

## Rule

deny に列挙するルールの定義<br>type ごとに異なる構造を持つ<br>- type: text<br>- type: regex<br>- type: custom<br>- type: no_java_doc<br>- type: no_kotlin_doc<br>- type: no_rust_doc<br>- type: no_japanese_comment<br>- type: no_english_comment

### TextRule

特定のキーワードを含むファイルを完全一致で検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `text` | o |  |
| keywords | string[] | o | validate で探す禁止キーワード |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| include_exts | string[] | - | show と validation で対象に拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `ts`<br>e.g. `tsx` |
| exclude_exts | string[] | - | show と validation で除外する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `css` |
| exclude_files | [excludeFilter](#excludefilter)[] | - | show と validation で除外するファイルのフィルタ |

### RegexRule

特定のキーワードを含むファイルを正規表現で検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `regex` | o |  |
| keywords | string[] | o | validate で探す禁止キーワードの正規表現 |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| include_exts | string[] | - | show と validation で対象に拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `ts`<br>e.g. `tsx` |
| exclude_exts | string[] | - | show と validation で除外する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `css` |
| exclude_files | [excludeFilter](#excludefilter)[] | - | show と validation で除外するファイルのフィルタ |

### CustomRule

任意のコマンドを実行してファイルを検証する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `custom` | o |  |
| exec | string | o | ファイルに対して実行するコマンド<br>終了コード 0 の場合はエラーなし扱い<br>エラー時は実行コマンドの標準出力がエラーメッセージとして表示される<br>e.g. `ruby path/to/your/checker.rb {path}`<br>e.g. `docker run your:image {path}` |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| include_exts | string[] | - | show と validation で対象に拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `ts`<br>e.g. `tsx` |
| exclude_exts | string[] | - | show と validation で除外する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `css` |
| exclude_files | [excludeFilter](#excludefilter)[] | - | show と validation で除外するファイルのフィルタ |

### NoJavaDocRule

JavaDoc がないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `no_java_doc` | o |  |
| java_doc | [javaDocConfig](#javadocconfig) | - |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| include_exts | string[] | - | show と validation で対象に拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `ts`<br>e.g. `tsx` |
| exclude_exts | string[] | - | show と validation で除外する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `css` |
| exclude_files | [excludeFilter](#excludefilter)[] | - | show と validation で除外するファイルのフィルタ |

### NoKotlinDocRule

KDoc がないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `no_kotlin_doc` | o |  |
| kotlin_doc | [kotlinDocConfig](#kotlindocconfig) | - |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| include_exts | string[] | - | show と validation で対象に拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `ts`<br>e.g. `tsx` |
| exclude_exts | string[] | - | show と validation で除外する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `css` |
| exclude_files | [excludeFilter](#excludefilter)[] | - | show と validation で除外するファイルのフィルタ |

### NoRustDocRule

rustdoc がないファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `no_rust_doc` | o |  |
| rust_doc | [rustDocConfig](#rustdocconfig) | - |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| include_exts | string[] | - | show と validation で対象に拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `ts`<br>e.g. `tsx` |
| exclude_exts | string[] | - | show と validation で除外する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `css` |
| exclude_files | [excludeFilter](#excludefilter)[] | - | show と validation で除外するファイルのフィルタ |

### NoJapaneseCommentRule

コメントが日本語のファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `no_japanese_comment` | o |  |
| comment | [commentConfig](#commentconfig) | o |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| include_exts | string[] | - | show と validation で対象に拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `ts`<br>e.g. `tsx` |
| exclude_exts | string[] | - | show と validation で除外する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `css` |
| exclude_files | [excludeFilter](#excludefilter)[] | - | show と validation で除外するファイルのフィルタ |

### NoEnglishCommentRule

コメントが英語のファイルを検出する

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | `no_english_comment` | o |  |
| comment | [commentConfig](#commentconfig) | o |  |
| label | string | o | show で表示するラベル |
| message | string | o | validation で違反しているときに表示するメッセージ |
| include_exts | string[] | - | show と validation で対象に拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `ts`<br>e.g. `tsx` |
| exclude_exts | string[] | - | show と validation で除外する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `css` |
| exclude_files | [excludeFilter](#excludefilter)[] | - | show と validation で除外するファイルのフィルタ |

## ReviewItem

review に列挙するレビューガイドラインの定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| message | string | o | review で表示するメッセージ |
| include_exts | string[] | - | 検査する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `ts`<br>e.g. `tsx` |
| exclude_exts | string[] | - | 除外する拡張子<br>include_exts と exclude_exts がどちらも未指定の場合は全拡張子が対象になる<br>e.g. `css` |

## ExcludeFilter

ファイル除外フィルタの定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| filter | `file_starts_with` \|<br>`file_ends_with` \|<br>`path_contains` | o | フィルタの種類 |
| keyword | string | o | キーワード |

## JavaDocConfig

JavaDoc 検証設定の定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | [visibility](#visibility) | - | 型の定義に JavaDoc がないことを認めない |
| function | [visibility](#visibility) | - | メソッドの定義に JavaDoc がないことを認めない |

## KotlinDocConfig

KDoc 検証設定の定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | [visibility](#visibility) | - | 型の定義に KDoc がないことを認めない |
| function | [visibility](#visibility) | - | メソッドの定義に KDoc がないことを認めない |

## RustDocConfig

rustdoc 検証設定の定義

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| type | [visibility](#visibility) | - | 型の定義に rustdoc がないことを認めない |
| function | [visibility](#visibility) | - | 関数の定義に rustdoc がないことを認めない |

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

