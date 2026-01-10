# .rec_lint.yaml ドキュメント

## トップレベル

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|:---:|------|
| rule | [RuleItem](#rule-types)[] | - | 特定パターンを禁止するルール<br>show: 表示される<br>validate: 検証される<br>guideline: 表示されない |
| guideline | [guidelineItem](./rules/guideline.md#guidelineitem)[] | - | レビューガイドライン<br>show: 表示される<br>validate: 検証されない<br>guideline: 表示される |

## Rule Types

| type | 説明 | ドキュメント |
|------|------|--------------|
| `forbidden_texts` | 禁止キーワードを完全一致で検出 | [詳細](./rules/forbidden-texts.md) |
| `forbidden_patterns` | 禁止パターンを正規表現で検出 | [詳細](./rules/forbidden-patterns.md) |
| `custom` | 任意のコマンドを実行して検証 | [詳細](./rules/custom.md) |
| `require_php_doc` | PHPDoc がないファイルを検出 | [詳細](./rules/require-php-doc.md) |
| `require_kotlin_doc` | KDoc がないファイルを検出 | [詳細](./rules/require-kotlin-doc.md) |
| `require_rust_doc` | rustdoc がないファイルを検出 | [詳細](./rules/require-rust-doc.md) |
| `require_english_comment` | コメントが日本語のファイルを検出 | [詳細](./rules/require-english-comment.md) |
| `require_japanese_comment` | コメントが英語のファイルを検出 | [詳細](./rules/require-japanese-comment.md) |
| `require_japanese_phpunit_test_name` | PHPUnit テスト名が日本語でないファイルを検出 | [詳細](./rules/require-japanese-phpunit-test-name.md) |
| `require_japanese_kotest_test_name` | Kotest テスト名が日本語でないファイルを検出 | [詳細](./rules/require-japanese-kotest-test-name.md) |
| `require_japanese_rust_test_name` | Rust テスト名が日本語でないファイルを検出 | [詳細](./rules/require-japanese-rust-test-name.md) |
| `require_phpunit_test` | PHPUnit テストファイルの存在を検証 | [詳細](./rules/require-phpunit-test.md) |
| `require_kotest_test` | Kotest テストファイルの存在を検証 | [詳細](./rules/require-kotest-test.md) |
| `require_rust_unit_test` | Rust ユニットテストの存在を検証 | [詳細](./rules/require-rust-unit-test.md) |

## 共通定義

[共通定義ドキュメント](./rules/common.md) を参照

- [RuleBase](./rules/common.md#rulebase) - ルールの共通フィールド
- [MatchItem](./rules/common.md#matchitem) - ファイルマッチ条件
- [MatchPattern](./rules/common.md#matchpattern) - マッチパターンの種類
- [MatchCond](./rules/common.md#matchcond) - keywords の結合条件
- [Visibility](./rules/common.md#visibility) - Doc コメントを強制する対象の可視性
- [TestRequireLevelExternalFile](./rules/common.md#testrequirelevelexternalfile) - テスト存在検証レベル (外部ファイル)
- [TestRequireLevelSameFile](./rules/common.md#testrequirelevelsamefile) - テスト存在検証レベル (同一ファイル)
