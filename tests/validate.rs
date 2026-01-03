mod common;

use common::dummy_project_path;
use rec_lint::SortMode;

// =============================================================================
// simple: Single yaml at root
// =============================================================================

#[test]
fn test_simple_validate_sort_rule() {
    let file = dummy_project_path("simple/Sample.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    // --sort rule: message: file:line:col
    assert_eq!(result, vec!["Use logger instead: Sample.java:7:9"]);
}

#[test]
fn test_simple_validate_sort_file() {
    let file = dummy_project_path("simple/Sample.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::File).unwrap();

    // --sort file: file:line:col: message
    assert_eq!(result, vec!["Sample.java:7:9: Use logger instead"]);
}

// =============================================================================
// nested: Hierarchical yaml files
// =============================================================================

#[test]
fn test_nested_validate_root_clean() {
    let file = dummy_project_path("nested/Root.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    assert!(result.is_empty(), "Expected no violations, got: {:?}", result);
}

#[test]
fn test_nested_validate_sub_sort_rule() {
    let file = dummy_project_path("nested/sub/Sub.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    // --sort rule: sorted by message, then file, then line
    assert_eq!(result, vec!["Avoid wildcard imports: sub/Sub.java:1:1", "Use LocalDate instead: sub/Sub.java:5:9",]);
}

#[test]
fn test_nested_validate_sub_sort_file() {
    let file = dummy_project_path("nested/sub/Sub.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::File).unwrap();

    // --sort file: sorted by file, then line, then col
    assert_eq!(result, vec!["sub/Sub.java:1:1: Avoid wildcard imports", "sub/Sub.java:5:9: Use LocalDate instead",]);
}

// =============================================================================
// skip_middle: Intermediate directories without yaml
// =============================================================================

#[test]
fn test_skip_middle_validate_sort_rule() {
    let file = dummy_project_path("skip_middle/level1/level2/level3/Deep.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    // --sort rule: sorted by message
    // col は keyword の開始位置 (1-based)
    assert_eq!(
        result,
        vec![
            "Do not use System.exit: level1/level2/level3/Deep.java:10:9",
            "Use generic types instead of raw types: level1/level2/level3/Deep.java:1:18",
            "Use generic types instead of raw types: level1/level2/level3/Deep.java:2:18",
            "Use generic types instead of raw types: level1/level2/level3/Deep.java:6:13",
            "Use generic types instead of raw types: level1/level2/level3/Deep.java:7:13",
        ]
    );
}

#[test]
fn test_skip_middle_validate_sort_file() {
    let file = dummy_project_path("skip_middle/level1/level2/level3/Deep.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::File).unwrap();

    // --sort file: sorted by line
    assert_eq!(
        result,
        vec![
            "level1/level2/level3/Deep.java:1:18: Use generic types instead of raw types",
            "level1/level2/level3/Deep.java:2:18: Use generic types instead of raw types",
            "level1/level2/level3/Deep.java:6:13: Use generic types instead of raw types",
            "level1/level2/level3/Deep.java:7:13: Use generic types instead of raw types",
            "level1/level2/level3/Deep.java:10:9: Do not use System.exit",
        ]
    );
}

// =============================================================================
// deep_inherit: Deep directory with only root yaml
// =============================================================================

#[test]
fn test_deep_inherit_validate() {
    let file = dummy_project_path("deep_inherit/a/b/c/Target.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    assert_eq!(result, vec!["Remove deprecated code: a/b/c/Target.java:2:5"]);
}

// =============================================================================
// validate: Directory recursive scan
// =============================================================================

#[test]
fn test_validate_directory_recursive() {
    let dir = dummy_project_path("nested");
    let result = rec_lint::commands::validate::run(&[dir], SortMode::File).unwrap();

    assert_eq!(result, vec!["sub/Sub.java:1:1: Avoid wildcard imports", "sub/Sub.java:5:9: Use LocalDate instead",]);
}

#[test]
fn test_validate_multiple_paths_sort_rule() {
    let file1 = dummy_project_path("simple/Sample.java");
    let file2 = dummy_project_path("nested/sub/Sub.java");
    let result = rec_lint::commands::validate::run(&[file1, file2], SortMode::Rule).unwrap();

    // --sort rule: sorted by message, then file
    assert_eq!(
        result,
        vec![
            "Avoid wildcard imports: sub/Sub.java:1:1",
            "Use LocalDate instead: sub/Sub.java:5:9",
            "Use logger instead: Sample.java:7:9",
        ]
    );
}

#[test]
fn test_validate_multiple_paths_sort_file() {
    let file1 = dummy_project_path("simple/Sample.java");
    let file2 = dummy_project_path("nested/sub/Sub.java");
    let result = rec_lint::commands::validate::run(&[file1, file2], SortMode::File).unwrap();

    // --sort file: sorted by file, then line
    assert_eq!(
        result,
        vec![
            "Sample.java:7:9: Use logger instead",
            "sub/Sub.java:1:1: Avoid wildcard imports",
            "sub/Sub.java:5:9: Use LocalDate instead",
        ]
    );
}

// =============================================================================
// no_root: No rec_lint_config.yaml found in any ancestor
// =============================================================================

#[test]
fn test_no_root_validate_error() {
    let file = dummy_project_path("no_root/child/Sample.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    assert_eq!(result.len(), 1);
    assert!(result[0].contains("rec_lint_config.yaml"));
}
