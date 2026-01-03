mod common;

use common::dummy_project_path;
use rec_lint::commands::SortMode;

// =============================================================================
// simple: 単一のyamlファイル
// =============================================================================

#[test]
fn ルールでソートすると違反がメッセージ順に表示される() {
    let file = dummy_project_path("simple/Sample.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    // --sort rule: message: file:line:col
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "Use logger instead: Sample.java:7:9");
}

#[test]
fn ファイルでソートすると違反がファイル順に表示される() {
    let file = dummy_project_path("simple/Sample.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::File).unwrap();

    // --sort file: file:line:col: message
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "Sample.java:7:9: Use logger instead");
}

// =============================================================================
// nested: 階層的なyamlファイル
// =============================================================================

#[test]
fn 違反がないファイルは空の結果を返す() {
    let file = dummy_project_path("nested/Root.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    assert_eq!(result.len(), 0);
}

#[test]
fn サブディレクトリの違反はルールでソートされる() {
    let file = dummy_project_path("nested/sub/Sub.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    // --sort rule: sorted by message, then file, then line
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "Avoid wildcard imports: sub/Sub.java:1:1");
    assert_eq!(result[1], "Use LocalDate instead: sub/Sub.java:5:9");
}

#[test]
fn サブディレクトリの違反はファイルでソートされる() {
    let file = dummy_project_path("nested/sub/Sub.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::File).unwrap();

    // --sort file: sorted by file, then line, then col
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "sub/Sub.java:1:1: Avoid wildcard imports");
    assert_eq!(result[1], "sub/Sub.java:5:9: Use LocalDate instead");
}

// =============================================================================
// skip_middle: 中間ディレクトリにyamlがない場合
// =============================================================================

#[test]
fn 深い階層の違反はルールでソートされる() {
    let file = dummy_project_path("skip_middle/level1/level2/level3/Deep.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    // --sort rule: sorted by message
    // col は keyword の開始位置 (1-based)
    assert_eq!(result.len(), 5);
    assert_eq!(result[0], "Do not use System.exit: level1/level2/level3/Deep.java:10:9");
    assert_eq!(result[1], "Use generic types instead of raw types: level1/level2/level3/Deep.java:1:18");
    assert_eq!(result[2], "Use generic types instead of raw types: level1/level2/level3/Deep.java:2:18");
    assert_eq!(result[3], "Use generic types instead of raw types: level1/level2/level3/Deep.java:6:13");
    assert_eq!(result[4], "Use generic types instead of raw types: level1/level2/level3/Deep.java:7:13");
}

#[test]
fn 深い階層の違反はファイルでソートされる() {
    let file = dummy_project_path("skip_middle/level1/level2/level3/Deep.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::File).unwrap();

    // --sort file: sorted by line
    assert_eq!(result.len(), 5);
    assert_eq!(result[0], "level1/level2/level3/Deep.java:1:18: Use generic types instead of raw types");
    assert_eq!(result[1], "level1/level2/level3/Deep.java:2:18: Use generic types instead of raw types");
    assert_eq!(result[2], "level1/level2/level3/Deep.java:6:13: Use generic types instead of raw types");
    assert_eq!(result[3], "level1/level2/level3/Deep.java:7:13: Use generic types instead of raw types");
    assert_eq!(result[4], "level1/level2/level3/Deep.java:10:9: Do not use System.exit");
}

// =============================================================================
// deep_inherit: ルートyamlのみで深い階層
// =============================================================================

#[test]
fn 深い階層でもルートルールで検証される() {
    let file = dummy_project_path("deep_inherit/a/b/c/Target.java");
    let result = rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "Remove deprecated code: a/b/c/Target.java:2:5");
}

// =============================================================================
// validate: ディレクトリの再帰的スキャン
// =============================================================================

#[test]
fn ディレクトリを指定すると再帰的にファイルを検証する() {
    let dir = dummy_project_path("nested");
    let result = rec_lint::commands::validate::run(&[dir], SortMode::File).unwrap();

    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "sub/Sub.java:1:1: Avoid wildcard imports");
    assert_eq!(result[1], "sub/Sub.java:5:9: Use LocalDate instead");
}

#[test]
fn 複数パスを指定するとルールでソートされる() {
    let file1 = dummy_project_path("simple/Sample.java");
    let file2 = dummy_project_path("nested/sub/Sub.java");
    let result = rec_lint::commands::validate::run(&[file1, file2], SortMode::Rule).unwrap();

    // --sort rule: sorted by message, then file
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "Avoid wildcard imports: sub/Sub.java:1:1");
    assert_eq!(result[1], "Use LocalDate instead: sub/Sub.java:5:9");
    assert_eq!(result[2], "Use logger instead: Sample.java:7:9");
}

#[test]
fn 複数パスを指定するとファイルでソートされる() {
    let file1 = dummy_project_path("simple/Sample.java");
    let file2 = dummy_project_path("nested/sub/Sub.java");
    let result = rec_lint::commands::validate::run(&[file1, file2], SortMode::File).unwrap();

    // --sort file: sorted by file, then line
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "Sample.java:7:9: Use logger instead");
    assert_eq!(result[1], "sub/Sub.java:1:1: Avoid wildcard imports");
    assert_eq!(result[2], "sub/Sub.java:5:9: Use LocalDate instead");
}
