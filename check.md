# テスト刷新計画

tmp-test-projects.txt に示された text-projects 構成を作り、それを使って tmp-tests.txt の構想に従い tests/ にテストを実現する

# test-projects チェックリスト

## config

- [x] include_extensions (.rec_lint_config.yaml, .rec_lint.yaml, invalid1.rs, invalid2.php)
- [x] exclude_dirs (.rec_lint_config.yaml, .rec_lint.yaml, invalid1.php, vendor/invalid2.php)

## rec_tree

- [x] .rec_lint_config.yaml と dir1→dir2→dir3 の YAML/ファイル構成

## rules

- [x] forbidden_texts (case01〜08, 各 .rec_lint.yaml + サンプルファイル)
- [ ] forbidden_patterns (casexx 群)
- [ ] custom (.rec_lint.yaml, 違反なし/違反あり、checker.sh)
- [ ] require_php_doc（class/interface/trait/function/enum/all セクション一式）
- [ ] require_kotlin_doc（class/interface/object/.../all）
- [x] require_rust_doc（struct/enum/trait/type_alias/union/fn/macro_rules/mod/all）
- [x] require_english_comment（java/python/rust × line/block）
- [ ] require_japanese_comment（java/python/rust × line/block）
- [ ] require_japanese_phpunit_test_name
- [ ] require_japanese_kotest_test_name
- [ ] require_japanese_rust_test_name
- [ ] require_phpunit_test
- [ ] require_kotest_test
- [ ] require_rust_test（unit/integration 全ケース）

# tests チェックリスト

## show

- [x] show/rec_tree.rs (dir1/file1 など6ケース)

## validate/config

- [x] include_extensions.rs（php対象/rs除外）
- [x] exclude_dirs.rs（vendor除外）

## validate/rules

- [x] forbidden_texts.rs（case01〜08）
- [x] require_rust_doc.rs（struct〜all）
- [x] require_english_comment.rs（java/python/rust × line/block）
- [ ] forbidden_patterns.rs
- [ ] custom.rs
- [ ] require_php_doc.rs
- [ ] require_kotlin_doc.rs
- [ ] require_japanese_comment.rs
- [ ] require_japanese_phpunit_test_name.rs
- [ ] require_japanese_kotest_test_name.rs
- [ ] require_japanese_rust_test_name.rs
- [ ] require_phpunit_test.rs
- [ ] require_kotest_test.rs
- [ ] require_rust_test.rs

# 現状まとめ

- config/include_extensions・exclude_dirs、rec_tree のテスト対象と validate/show の検証コードを整備済み。
- rules 配下では forbidden_texts、require_rust_doc、require_english_comment のケースと対応するテストを実装済み。
- そのほかのルール (forbidden_patterns, custom, require_php_doc など) は未着手で、`tests/validate/rules` への実装が必要。

# 申し送り

- unit test は完全刷新のあと抹消する

