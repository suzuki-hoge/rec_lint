#[path = "../common/mod.rs"]
mod common;

use rec_lint::commands::check;
use rec_lint::commands::CheckMode;

#[test]
#[allow(non_snake_case)]
fn list_は_rec_lint_yaml_があるディレクトリのみ表示する() {
    std::env::set_current_dir(common::test_project_path("check/list")).unwrap();
    let result = check::run(CheckMode::List).unwrap();
    common::assert_output(
        &result,
        r#"
            ./.rec_lint.yaml: [ forbidden_texts ]
            src/.rec_lint.yaml: [ custom ]
        "#,
    );
}
