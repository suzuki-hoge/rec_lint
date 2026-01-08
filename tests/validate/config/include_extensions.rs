#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

#[test]
fn php対象のときphpは対象になる() {
    let file = common::test_project_path("config/include_extensions/invalid2.php");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();
    common::assert_output(&result, "include対象テスト: invalid2.php:1:13");
}

#[test]
fn php対象のときruleでrsを指定しても対象にならない() {
    let file = common::test_project_path("config/include_extensions/invalid1.rs");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();
    assert!(result.is_empty());
}
