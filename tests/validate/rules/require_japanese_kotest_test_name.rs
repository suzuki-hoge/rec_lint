#![allow(non_snake_case)]

#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_case(relative: &str) -> Vec<String> {
    let file = common::project_file("require_japanese_kotest_test_name", relative);
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
    expect_ok("日本語テスト名.kt");
}

#[test]
fn 英語テスト名は違反になる() {
    expect_violation(
        "英語テスト名.kt",
        r#"
            テスト名を日本語にしてください: require_japanese_kotest_test_name/英語テスト名.kt:2:1 [ found: should create user ]
            テスト名を日本語にしてください: require_japanese_kotest_test_name/英語テスト名.kt:5:1 [ found: should validate email ]
        "#,
    );
}

#[test]
fn describeパターンで日本語は違反にならない() {
    expect_ok("describeパターン日本語.kt");
}

#[test]
fn describeパターンで英語は違反になる() {
    expect_violation(
        "describeパターン英語.kt",
        r#"
            テスト名を日本語にしてください: require_japanese_kotest_test_name/describeパターン英語.kt:2:1 [ found: UserService ]
            テスト名を日本語にしてください: require_japanese_kotest_test_name/describeパターン英語.kt:3:1 [ found: should create user ]
        "#,
    );
}

#[test]
fn behaviorパターンで日本語は違反にならない() {
    expect_ok("behaviorパターン日本語.kt");
}

#[test]
fn behaviorパターンで英語は違反になる() {
    expect_violation(
        "behaviorパターン英語.kt",
        r#"
            テスト名を日本語にしてください: require_japanese_kotest_test_name/behaviorパターン英語.kt:2:1 [ found: a logged in user ]
            テスト名を日本語にしてください: require_japanese_kotest_test_name/behaviorパターン英語.kt:3:1 [ found: updating profile ]
            テスト名を日本語にしてください: require_japanese_kotest_test_name/behaviorパターン英語.kt:4:1 [ found: should succeed ]
        "#,
    );
}

#[test]
fn 日本語と英語が混在する場合は英語のみ違反になる() {
    expect_violation(
        "混在.kt",
        r#"
            テスト名を日本語にしてください: require_japanese_kotest_test_name/混在.kt:5:1 [ found: should validate email ]
        "#,
    );
}

#[test]
fn contextパターンで日本語は違反にならない() {
    expect_ok("contextパターン日本語.kt");
}

#[test]
fn contextパターンで英語は違反になる() {
    expect_violation(
        "contextパターン英語.kt",
        r#"
            テスト名を日本語にしてください: require_japanese_kotest_test_name/contextパターン英語.kt:1:1 [ found: User registration ]
            テスト名を日本語にしてください: require_japanese_kotest_test_name/contextパターン英語.kt:2:1 [ found: should create user ]
        "#,
    );
}

#[test]
fn shouldパターンで日本語は違反にならない() {
    expect_ok("shouldパターン日本語.kt");
}

#[test]
fn shouldパターンで英語は違反にならない() {
    // should構文は現在検出対象外
    expect_ok("shouldパターン英語.kt");
}

#[test]
fn given_when_thenパターンで日本語は違反にならない() {
    expect_ok("given_when_thenパターン日本語.kt");
}

#[test]
fn given_when_thenパターンで英語は違反になる() {
    expect_violation(
        "given_when_thenパターン英語.kt",
        r#"
            テスト名を日本語にしてください: require_japanese_kotest_test_name/given_when_thenパターン英語.kt:1:1 [ found: a valid user ]
            テスト名を日本語にしてください: require_japanese_kotest_test_name/given_when_thenパターン英語.kt:3:1 [ found: user should be created ]
        "#,
    );
}

#[test]
fn 複数のDSLパターンを同時に検出できる() {
    expect_violation(
        "複数のDSLパターン.kt",
        r#"
            テスト名を日本語にしてください: require_japanese_kotest_test_name/複数のDSLパターン.kt:1:1 [ found: User service ]
            テスト名を日本語にしてください: require_japanese_kotest_test_name/複数のDSLパターン.kt:2:1 [ found: when creating user ]
            テスト名を日本語にしてください: require_japanese_kotest_test_name/複数のDSLパターン.kt:3:1 [ found: should succeed ]
        "#,
    );
}
