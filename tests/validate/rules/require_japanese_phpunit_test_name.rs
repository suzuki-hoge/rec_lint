#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_case(relative: &str) -> Vec<String> {
    let file = common::project_file("require_japanese_phpunit_test_name", relative);
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
    expect_ok("日本語テスト名.php");
}

#[test]
fn 英語テスト名は違反になる() {
    expect_violation(
        "英語テスト名.php",
        r#"
            テスト名を日本語にしてください: require_japanese_phpunit_test_name/英語テスト名.php:5:1 [ found: testCreateUser ]
            テスト名を日本語にしてください: require_japanese_phpunit_test_name/英語テスト名.php:9:1 [ found: testValidateEmail ]
        "#,
    );
}

#[test]
fn アノテーションで日本語テスト名は違反にならない() {
    expect_ok("アノテーションで日本語.php");
}

#[test]
fn アノテーションで英語テスト名は違反になる() {
    expect_violation(
        "アノテーションで英語.php",
        r#"
            テスト名を日本語にしてください: require_japanese_phpunit_test_name/アノテーションで英語.php:5:1 [ found: shouldCreateUser ]
        "#,
    );
}

#[test]
fn アトリビュートで日本語テスト名は違反にならない() {
    expect_ok("アトリビュートで日本語.php");
}

#[test]
fn アトリビュートで英語テスト名は違反になる() {
    expect_violation(
        "アトリビュートで英語.php",
        r#"
            テスト名を日本語にしてください: require_japanese_phpunit_test_name/アトリビュートで英語.php:5:1 [ found: shouldCreateUser ]
        "#,
    );
}

#[test]
fn テストではないメソッドは検出されない() {
    expect_ok("テストではないメソッド.php");
}

#[test]
fn プライベートメソッドでもtestプレフィックスがあれば検出する() {
    expect_violation(
        "プライベートメソッドでtestプレフィックス.php",
        r#"
            テスト名を日本語にしてください: require_japanese_phpunit_test_name/プライベートメソッドでtestプレフィックス.php:3:1 [ found: testPrivateMethod ]
        "#,
    );
}

#[test]
fn 単一行PHPDocのtestアノテーションを検出できる() {
    expect_violation(
        "単一行PHPDocアノテーション.php",
        r#"
            テスト名を日本語にしてください: require_japanese_phpunit_test_name/単一行PHPDocアノテーション.php:3:1 [ found: shouldCreateUser ]
        "#,
    );
}

#[test]
fn 複数のアトリビュートがある場合も検出できる() {
    expect_violation(
        "複数のアトリビュート.php",
        r#"
            テスト名を日本語にしてください: require_japanese_phpunit_test_name/複数のアトリビュート.php:3:1 [ found: shouldCreateUser ]
        "#,
    );
}
