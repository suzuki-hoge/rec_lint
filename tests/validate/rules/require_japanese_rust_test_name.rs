#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_case(relative: &str) -> Vec<String> {
    let file = common::project_file("require_japanese_rust_test_name", relative);
    rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap()
}

fn expect_ok(relative: &str) {
    let result = validate_case(relative);
    assert!(result.is_empty(), "expected no violations for {relative}, got {result:?}");
}

fn expect_violation(relative: &str, expected: &str) {
    let result = validate_case(relative);
    common::assert_output(&result, expected);
}

#[test]
fn 日本語テスト名は違反にならない() {
    expect_ok("日本語テスト名.rs");
}

#[test]
fn 英語テスト名は違反になる() {
    expect_violation(
        "英語テスト名.rs",
        r#"
            テスト名を日本語にしてください: "test_create_user": require_japanese_rust_test_name/英語テスト名.rs:1:1
            テスト名を日本語にしてください: "test_validate_email": require_japanese_rust_test_name/英語テスト名.rs:5:1
        "#,
    );
}

#[test]
fn tokio_testで日本語は違反にならない() {
    expect_ok("tokio_test日本語.rs");
}

#[test]
fn tokio_testで英語は違反になる() {
    expect_violation(
        "tokio_test英語.rs",
        r#"
            テスト名を日本語にしてください: "test_async_create_user": require_japanese_rust_test_name/tokio_test英語.rs:1:1
        "#,
    );
}

#[test]
fn 複数属性で日本語は違反にならない() {
    expect_ok("複数属性日本語.rs");
}

#[test]
fn 複数属性で英語は違反になる() {
    expect_violation(
        "複数属性英語.rs",
        r#"
            テスト名を日本語にしてください: "test_should_panic": require_japanese_rust_test_name/複数属性英語.rs:1:1
        "#,
    );
}

#[test]
fn テストではない関数は検出されない() {
    expect_ok("テストではない関数.rs");
}

#[test]
fn 日本語と英語が混在する場合は英語のみ違反になる() {
    expect_violation(
        "混在.rs",
        r#"
            テスト名を日本語にしてください: "test_validate_email": require_japanese_rust_test_name/混在.rs:5:1
        "#,
    );
}
