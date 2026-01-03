mod common;

use common::dummy_project_path;

// =============================================================================
// simple: Single yaml at root
// =============================================================================

#[test]
fn test_simple_show() {
    let dir = dummy_project_path("simple");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // deny → review の順, ルート定義なので @ なし
    assert_eq!(result, vec!["deny: no-println [ System.out.println ]", "review: Check logging configuration",]);
}

// =============================================================================
// nested: Hierarchical yaml files
// =============================================================================

#[test]
fn test_nested_show_root() {
    let dir = dummy_project_path("nested");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // required → deny → review の順, ルート定義なので @ なし
    assert_eq!(
        result,
        vec![
            "required: check-todo [ TODO: ]",
            "deny: no-legacy-date [ java.util.Date ]",
            "review: Review error handling",
        ]
    );
}

#[test]
fn test_nested_show_sub() {
    let dir = dummy_project_path("nested/sub");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // 順序: required → deny → review, 親 → 子
    // ルート定義 = @ なし, sub 定義 = @ sub
    assert_eq!(
        result,
        vec![
            "required: check-todo [ TODO: ]",
            "deny: no-legacy-date [ java.util.Date ]",
            "deny: no-wildcard-import [ import.*\\*; ] @ sub",
            "review: Review error handling",
            "review: Check for code duplication @ sub",
        ]
    );
}

// =============================================================================
// skip_middle: Intermediate directories without yaml
// =============================================================================

#[test]
fn test_skip_middle_show_root() {
    let dir = dummy_project_path("skip_middle");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    assert_eq!(result, vec!["deny: no-system-exit [ System.exit ]", "review: Check exception handling",]);
}

#[test]
fn test_skip_middle_show_level3() {
    let dir = dummy_project_path("skip_middle/level1/level2/level3");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // 親 (root) → 子 (level3) の順, @ は相対パス
    assert_eq!(
        result,
        vec![
            "deny: no-system-exit [ System.exit ]",
            "deny: no-raw-types [ List[^<], Map[^<] ] @ level1/level2/level3",
            "review: Check exception handling",
            "review: Check null safety @ level1/level2/level3",
        ]
    );
}

#[test]
fn test_skip_middle_show_level2_no_yaml() {
    let dir = dummy_project_path("skip_middle/level1/level2");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // level2 has no yaml, should only have root rules
    assert_eq!(result, vec!["deny: no-system-exit [ System.exit ]", "review: Check exception handling",]);
}

// =============================================================================
// deep_inherit: Deep directory with only root yaml
// =============================================================================

#[test]
fn test_deep_inherit_show() {
    let dir = dummy_project_path("deep_inherit/a/b/c");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // Should inherit root rules through a/b/c with no yaml in between
    // ルート定義なので @ なし
    assert_eq!(result, vec!["deny: no-deprecated [ @Deprecated ]", "review: Check API compatibility",]);
}

// =============================================================================
// no_root: No rec_lint_config.yaml found in any ancestor
// =============================================================================

#[test]
fn test_no_root_show_error() {
    let dir = dummy_project_path("no_root/child");
    let err = rec_lint::commands::show::run(&dir).unwrap_err();

    assert!(err.to_string().contains("rec_lint_config.yaml"));
}
