#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn run(file: &str) -> Vec<String> {
    let file = common::project_file("custom", file);
    rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap()
}

fn assert_ok(file: &str) {
    assert!(run(file).is_empty());
}

fn assert_violation(file: &str, expected: &str) {
    let result = run(file);
    common::assert_output(&result, expected);
}

#[test]
fn 任意コマンドが違反を返さない場合はエラーにならない() {
    assert_ok("違反なし.kt");
}

#[test]
fn 任意コマンドが違反を返した場合はそのままエラーメッセージになる() {
    assert_violation("違反あり.kt", "カスタムルール違反: custom/違反あり.kt");
}
