mod common;

use common::dummy_project_path;

// =============================================================================
// simple: Single yaml at root
// =============================================================================

#[test]
fn test_simple_review() {
    let dir = dummy_project_path("simple");
    let result = rec_lint::commands::review::run(&dir).unwrap();

    // ルート定義なので @ なし
    assert_eq!(result, vec!["review: Check logging configuration"]);
}

// =============================================================================
// nested: Hierarchical yaml files
// =============================================================================

#[test]
fn test_nested_review_sub() {
    let dir = dummy_project_path("nested/sub");
    let result = rec_lint::commands::review::run(&dir).unwrap();

    // 親 → 子の順, ルート = @ なし, sub = @ sub
    assert_eq!(result, vec!["review: Review error handling", "review: Check for code duplication @ sub",]);
}

// =============================================================================
// skip_middle: Intermediate directories without yaml
// =============================================================================

#[test]
fn test_skip_middle_review_level3() {
    let dir = dummy_project_path("skip_middle/level1/level2/level3");
    let result = rec_lint::commands::review::run(&dir).unwrap();

    // 親 → 子の順
    assert_eq!(result, vec!["review: Check exception handling", "review: Check null safety @ level1/level2/level3",]);
}

// =============================================================================
// deep_inherit: Deep directory with only root yaml
// =============================================================================

#[test]
fn test_deep_inherit_review() {
    let dir = dummy_project_path("deep_inherit/a/b/c");
    let result = rec_lint::commands::review::run(&dir).unwrap();

    // ルート定義なので @ なし
    assert_eq!(result, vec!["review: Check API compatibility"]);
}

// =============================================================================
// no_root: No rec_lint_config.yaml found in any ancestor
// =============================================================================

#[test]
fn test_no_root_review_error() {
    let dir = dummy_project_path("no_root/child");
    let err = rec_lint::commands::review::run(&dir).unwrap_err();

    assert!(err.to_string().contains("rec_lint_config.yaml"));
}
