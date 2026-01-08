#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_file(mode: &str, relative: &str) -> Vec<String> {
    let file = common::project_dir("require_rust_test").join(mode).join("src").join(relative);
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
            ユニットテストが必要です: ユニットテストが存在しません: src/テストがない.rs
        "#,
    );
}

#[test]
fn unit_exists指定のときテストモジュールもアトリビュートもなければエラーになる() {
    expect_violation(
        "unit_file_exists",
        "テストがない.rs",
        r#"
            ユニットテストが必要です: ユニットテストが存在しません: src/テストがない.rs
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
            全てのpublic関数にテストが必要です: L4: pub 関数 `delete_user` がテストされていません: src/一部メソッド未テスト.rs:4:1
        "#,
    );
}

// integration_exists mode tests

#[test]
fn integration_exists指定のときmainと同じディレクトリにテストがあるときはエラーにならない() {
    expect_ok("integration_file_exists", "テストがある.rs");
}

#[test]
fn integration_exists指定のときmainと違うディレクトリにテストがあるときはテストなしとなる() {
    expect_violation(
        "integration_file_exists",
        "subdir/テストがない.rs",
        r#"
            インテグレーションテストが必要です: 統合テストファイルが存在しません: tests/subdir/テストがない_test.rs: src/subdir/テストがない.rs
        "#,
    );
}

// integration_all_public mode tests

#[test]
fn integration_all_public指定のときすべてのコードをテストモジュールでテストしていればエラーにならない() {
    expect_ok("integration_all_public", "全メソッドテスト済み.rs");
}

#[test]
fn integration_all_public指定のときすべてのコードをテストアトリビュートでテストしていればエラーにならない() {
    expect_ok("integration_all_public", "テストアトリビュート版.rs");
}

#[test]
fn integration_all_public指定のとき一部のコードをテストがなければエラーになる() {
    expect_violation(
        "integration_all_public",
        "一部メソッド未テスト.rs",
        r#"
            全てのpublic関数にインテグレーションテストが必要です: L4: pub 関数 `delete_user` がテストされていません: src/一部メソッド未テスト.rs:4:1
        "#,
    );
}
