#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

#[test]
fn vendor除外のときvendorは対象にならない() {
    let dir = common::test_project_path("config/exclude_dirs");
    let result = rec_lint::commands::validate::run(&[dir], SortMode::Rule).unwrap();
    common::assert_output(
        &result,
        r#"
            exclude対象テスト: invalid1.php:1:13
        "#,
    );
}

#[test]
fn vendor除外のときruleでvendorを有効しても対象にならない() {
    // ruleでvendorパスにマッチする設定にしても、exclude_dirsで除外されていれば対象にならない
    let dir = common::test_project_path("config/exclude_dirs");
    let result = rec_lint::commands::validate::run(&[dir], SortMode::Rule).unwrap();
    // vendor/invalid2.phpがマッチしないことを確認（exclude_dirsが優先）
    common::assert_output(
        &result,
        r#"
            exclude対象テスト: invalid1.php:1:13
        "#,
    );
}
