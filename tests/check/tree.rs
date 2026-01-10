#[path = "../common/mod.rs"]
mod common;

use rec_lint::commands::check;
use rec_lint::commands::CheckMode;

#[test]
#[allow(non_snake_case)]
fn tree_は_ルールがあるディレクトリのみ表示し_除外ディレクトリは表示しない() {
    std::env::set_current_dir(common::test_project_path("check/tree")).unwrap();
    let result = check::run(CheckMode::Tree).unwrap();
    common::assert_output(
        &result,
        r#"
            .          [ forbidden_texts ]
            `-- src    [ custom ]
        "#,
    );
}
