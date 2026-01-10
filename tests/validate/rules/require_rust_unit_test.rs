#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_file(mode: &str, relative: &str) -> Vec<String> {
    let file = common::project_dir("require_rust_unit_test").join(mode).join("src").join(relative);
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

// unit_exists mode tests

#[test]
fn unit_exists指定のときテストモジュールがあればエラーにならない() {
    expect_ok("unit_file_exists", "テストがある.rs");
}

#[test]
fn unit_exists指定のときテストアトリビュートがあればエラーにならない() {
    expect_ok("unit_file_exists", "テストアトリビュートがある.rs");
}

#[test]
fn unit_exists指定のときテストモジュールがなければエラーになる() {
    expect_violation(
        "unit_file_exists",
        "テストがない.rs",
        r#"
            ユニットテストが必要です: src/テストがない.rs [ found: ユニットテストが存在しません ]
        "#,
    );
}

#[test]
fn unit_exists指定のときテストモジュールもアトリビュートもなければエラーになる() {
    expect_violation(
        "unit_file_exists",
        "テストがない.rs",
        r#"
            ユニットテストが必要です: src/テストがない.rs [ found: ユニットテストが存在しません ]
        "#,
    );
}

#[test]
fn unit_exists指定のときasync関数をasyncテストアトリビュートでテストすればエラーにならない() {
    expect_ok("unit_file_exists", "asyncテストアトリビュート.rs");
}

#[test]
fn unit_exists指定のときtokioテストアトリビュートでテストすればエラーにならない() {
    expect_ok("unit_file_exists", "tokioテストアトリビュート.rs");
}

// unit_all_public mode tests

#[test]
fn unit_all_public指定のとき単一のテストモジュールですべての関数がテストされていればエラーにならない() {
    expect_ok("unit_all_public", "全メソッドテスト済み.rs");
}

#[test]
fn unit_all_public指定のとき複数のテストモジュールですべての関数がテストされていればエラーにならない() {
    expect_ok("unit_all_public", "複数のテストモジュール.rs");
}

#[test]
fn unit_all_public指定のとき単数のテストアトリビュートですべての関数がテストされていればエラーにならない() {
    expect_ok("unit_all_public", "単数テストアトリビュート.rs");
}

#[test]
fn unit_all_public指定のとき複数のテストアトリビュートですべての関数がテストされていればエラーにならない() {
    expect_ok("unit_all_public", "複数のテストアトリビュート.rs");
}

#[test]
fn unit_all_public指定でもテストモジュール内の関数は対象にならない() {
    expect_ok("unit_all_public", "テストモジュール内の関数.rs");
}

#[test]
fn unit_all_public指定のとき一部のテストがなければエラーになる() {
    expect_violation(
        "unit_all_public",
        "一部メソッド未テスト.rs",
        r#"
            全てのpublic関数にテストが必要です: src/一部メソッド未テスト.rs:4:1 [ found: L4: pub 関数 `delete_user` がテストされていません ]
        "#,
    );
}
