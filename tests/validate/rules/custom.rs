#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn run(file: &str) -> Vec<String> {
    let file = common::project_file("custom", file);
    rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap()
}

fn run_with_script_dir(file: &str) -> Vec<String> {
    let file = common::project_file("custom_script_dir", file);
    rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap()
}

fn run_with_missing_script_dir(file: &str) -> Vec<String> {
    let file = common::project_file("custom_script_dir_missing", file);
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

#[test]
fn script_dir経由でstoriesファイルが存在すればエラーにならない() {
    let result = run_with_script_dir("src/components/Button.tsx");
    assert!(result.is_empty());
}

#[test]
fn script_dir経由でstoriesファイルがなければ違反になる() {
    let result = run_with_script_dir("src/components/TextInput.tsx");
    common::assert_output(
        &result,
        "
        .tsx に対応する .stories.tsx を作成してください: src/components/TextInput.tsx [ not found: TextInput.stories.tsx ]
        ",
    );
}

#[test]
fn script_dirを設定せずにプレースホルダーを使うと設定エラーになる() {
    let result = run_with_missing_script_dir("src/components/Button.tsx");
    common::assert_output(
        &result,
        "
        custom_script_dir_missing/src/components/Button.tsx: {script_dir} placeholder requires script_dir in .rec_lint_config.yaml
        ",
    );
}
