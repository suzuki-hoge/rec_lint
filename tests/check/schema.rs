#[path = "../common/mod.rs"]
mod common;

use rec_lint::commands::check;
use rec_lint::commands::CheckMode;

#[test]
#[allow(non_snake_case)]
fn schema_は_不正なyamlファイルをエラー報告する() {
    std::env::set_current_dir(common::test_project_path("check/schema")).unwrap();
    let result = check::run(CheckMode::Schema).unwrap();
    common::assert_output(
        &result,
        r#"
            Invalid: invalid/.rec_lint.yaml
              - Additional properties are not allowed ('unknown_type' was unexpected) at /rule/0
        "#,
    );
}
