#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_file(mode: &str, relative: &str) -> Vec<String> {
    let file = common::project_dir("require_phpunit_test").join(mode).join("src").join(relative);
    rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap()
}

fn expect_ok(mode: &str, relative: &str) {
    let result = validate_file(mode, relative);
    assert!(result.is_empty(), "expected no violations, got {result:?}");
}

fn expect_violation(mode: &str, relative: &str, expected: &str) {
    let result = validate_file(mode, relative);
    common::assert_output(&result, expected);
}

// exists mode tests

#[test]
fn exists指定のときmainと同じディレクトリ同じnamespaceにテストがあるクラスはエラーにならない() {
    expect_ok("file_exists", "テストがある.php");
}

#[test]
fn exists指定のときmainと同じディレクトリ違うnamespaceにテストがあるクラスはテストなしとなる() {
    expect_violation(
        "namespace_check",
        "違うnamespace.php",
        r#"
            テストファイルが必要です: テストファイルが存在しません: tests/App/Service/OrderServiceTest.php: src/違うnamespace.php
        "#,
    );
}

#[test]
fn exists指定のときmainと違うディレクトリ同じnamespaceにテストがあるクラスはテストなしとなる() {
    expect_violation(
        "different_dir",
        "App/Service/UserService.php",
        r#"
            テストファイルが必要です: テストファイルが存在しません: tests/App/Service/UserServiceTest.php: src/App/Service/UserService.php
        "#,
    );
}

// all_public mode tests

#[test]
fn all_public指定のときすべてのコードをテストしていればエラーにならない() {
    expect_ok("all_public", "全メソッドテスト済み.php");
}

#[test]
fn all_public指定のとき一部のコードをテストがなければエラーになる() {
    expect_violation(
        "all_public",
        "一部メソッド未テスト.php",
        r#"
            テストファイルが必要です: L9: public メソッド `deleteUser` がテストされていません: src/一部メソッド未テスト.php:9:1
        "#,
    );
}
