#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_file(mode: &str, relative: &str) -> Vec<String> {
    let file = common::project_dir("require_kotest_test").join(mode).join("src/main/kotlin").join(relative);
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
fn exists指定のときmainと同じディレクトリ同じpackageにテストがあるクラスはエラーにならない() {
    expect_ok("file_exists", "テストがある.kt");
}

#[test]
fn exists指定のときmainと同じディレクトリ違うpackageにテストがあるクラスはテストなしとなる() {
    expect_violation(
        "package_check",
        "違うpackage.kt",
        r#"
            テストファイルが必要です: src/main/kotlin/違うpackage.kt [ found: テストファイルが存在しません: src/test/kotlin/com/example/service/OrderServiceTest.kt ]
        "#,
    );
}

#[test]
fn exists指定のときmainと違うディレクトリ同じpackageにテストがあるクラスはテストなしとなる() {
    expect_violation(
        "different_dir",
        "com/example/service/UserService.kt",
        r#"
            テストファイルが必要です: src/main/kotlin/com/example/service/UserService.kt [ found: テストファイルが存在しません: src/test/kotlin/com/example/service/UserServiceTest.kt ]
        "#,
    );
}

// all_public mode tests

#[test]
fn all_public指定のときすべてのコードをテストしていればエラーにならない() {
    expect_ok("all_public", "全メソッドテスト済み.kt");
}

#[test]
fn all_public指定のとき一部のコードをテストがなければエラーになる() {
    expect_violation(
        "all_public",
        "一部メソッド未テスト.kt",
        r#"
            テストファイルが必要です: src/main/kotlin/一部メソッド未テスト.kt:5:1 [ found: L5: public メソッド `deleteUser` がテストされていません ]
        "#,
    );
}

// no_public mode tests

#[test]
fn exists指定でもクラス定義のみのファイルはエラーにならない() {
    expect_ok("no_public", "クラス定義のみ.kt");
}

#[test]
fn exists指定でもpublicメソッドがないクラスはエラーにならない() {
    expect_ok("no_public", "publicメソッドなし.kt");
}
