mod common;

use common::dummy_project_path;

// =============================================================================
// simple: 単一のyamlファイル
// =============================================================================

#[test]
fn ルート直下のレビュー項目のみが表示される() {
    let dir = dummy_project_path("simple");
    let result = rec_lint::commands::review::run(&dir).unwrap();

    // ルート定義なので @ なし
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "review: Check logging configuration");
}

// =============================================================================
// nested: 階層的なyamlファイル
// =============================================================================

#[test]
fn サブディレクトリでは親と子のレビュー項目が順に表示される() {
    let dir = dummy_project_path("nested/sub");
    let result = rec_lint::commands::review::run(&dir).unwrap();

    // 親 → 子の順, ルート = @ なし, sub = @ sub
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "review: Review error handling");
    assert_eq!(result[1], "review: Check for code duplication @ sub");
}

// =============================================================================
// skip_middle: 中間ディレクトリにyamlがない場合
// =============================================================================

#[test]
fn 深い階層でもレビュー項目は継承され相対パスで表示される() {
    let dir = dummy_project_path("skip_middle/level1/level2/level3");
    let result = rec_lint::commands::review::run(&dir).unwrap();

    // 親 → 子の順
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "review: Check exception handling");
    assert_eq!(result[1], "review: Check null safety @ level1/level2/level3");
}

// =============================================================================
// deep_inherit: ルートyamlのみで深い階層
// =============================================================================

#[test]
fn 深い階層でもルートのレビュー項目のみ表示される() {
    let dir = dummy_project_path("deep_inherit/a/b/c");
    let result = rec_lint::commands::review::run(&dir).unwrap();

    // ルート定義なので @ なし
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "review: Check API compatibility");
}
