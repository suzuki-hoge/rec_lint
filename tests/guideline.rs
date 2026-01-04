mod common;

use common::dummy_project_path;

// =============================================================================
// simple: 単一のyamlファイル
// =============================================================================

#[test]
fn ルート直下のガイドライン項目のみが表示される() {
    let dir = dummy_project_path("simple");
    let result = rec_lint::commands::guideline::run(&dir).unwrap();

    // ルート定義なので @ なし
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "guideline: Check logging configuration");
}

// =============================================================================
// nested: 階層的なyamlファイル
// =============================================================================

#[test]
fn サブディレクトリでは親と子のガイドライン項目が順に表示される() {
    let dir = dummy_project_path("nested/sub");
    let result = rec_lint::commands::guideline::run(&dir).unwrap();

    // 親 → 子の順, ルート = @ なし, sub = @ sub
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "guideline: Review error handling");
    assert_eq!(result[1], "guideline: Check for code duplication @ sub");
}

// =============================================================================
// skip_middle: 中間ディレクトリにyamlがない場合
// =============================================================================

#[test]
fn 深い階層でもガイドライン項目は継承され相対パスで表示される() {
    let dir = dummy_project_path("skip_middle/level1/level2/level3");
    let result = rec_lint::commands::guideline::run(&dir).unwrap();

    // 親 → 子の順
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "guideline: Check exception handling");
    assert_eq!(result[1], "guideline: Check null safety @ level1/level2/level3");
}

// =============================================================================
// deep_inherit: ルートyamlのみで深い階層
// =============================================================================

#[test]
fn 深い階層でもルートのガイドライン項目のみ表示される() {
    let dir = dummy_project_path("deep_inherit/a/b/c");
    let result = rec_lint::commands::guideline::run(&dir).unwrap();

    // ルート定義なので @ なし
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "guideline: Check API compatibility");
}
