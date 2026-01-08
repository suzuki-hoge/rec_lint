# レビュー課題

## require_phpunit_test / require_kotest_test の namespace 検査

### 現状

- ファイルパスのみで「テストが存在する」と判定している
- namespace/package の一致は検査していない

### 問題

Java/Kotlin/PHP では、ディレクトリ構造と namespace/package 宣言がずれることがある。

例:

```
# パスがずれるケース
main/foo/service/FooService.java → test/foo/FooServiceTest.java (service/ が抜けている)

# namespaceがずれるケース
foo.service.FooService → foo.FooServiceTest (package宣言が違う)
```

### 対応方針

- **PHP**: namespace 宣言を解析し、パス + namespace 両方が一致してはじめて「存在する」とみなす
- **Kotlin**: package 宣言を解析し、パス + package 両方が一致してはじめて「存在する」とみなす
- **Rust**: ファイル構造がモジュール構造を決定するため、namespace 検査は不要

### 追加すべきテスト

tmp-tests.txt の該当ルールの部分で指定されている足りないケースすべて

### 影響ファイル

- `src/validate/test/exists/php.rs`
- `src/validate/test/exists/kotlin.rs`
- `tests/validate/rules/require_phpunit_test.rs`
- `tests/validate/rules/require_kotest_test.rs`
- `test-projects/rules/require_phpunit_test/`
- `test-projects/rules/require_kotest_test/`

## ほか

- exclude_dirs.rs
    - rec_tree.rs のように完全一致で検証せよ
    - vendor除外のとき直接ファイル指定は除外されないは「config で vendor を除外し rule で php を対象にしても vendor/*.php は対象にならない」に変更
- require_kotest_test.rs
    - rec_tree.rs のように完全一致で検証せよ
- require_phpunit_test.rs
    - rec_tree.rs のように完全一致で検証せよ
- require_rust_test.rs
    - rec_tree.rs のように完全一致で検証せよ
- tmp-tests.txt で作成を指示したケースで実装してないものすべて ( エッジケースや類似ケースを省略したとのことだが、テスト名をそのまま仕様書にするので略さないで )
