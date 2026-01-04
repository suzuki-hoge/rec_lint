mod common;

use common::dummy_project_path;

// =============================================================================
// simple: 単一のyamlファイル
// =============================================================================

#[test]
fn ルート直下のyamlのみが表示される() {
    let dir = dummy_project_path("simple");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // rule → guideline の順, ルート定義なので @ なし
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "rule: no-println [ System.out.println ]");
    assert_eq!(result[1], "guideline: Check logging configuration");
}

// =============================================================================
// nested: 階層的なyamlファイル
// =============================================================================

#[test]
fn ルートディレクトリでは親ルールのみ表示される() {
    let dir = dummy_project_path("nested");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // rule → guideline の順, ルート定義なので @ なし
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "rule: no-legacy-date [ java.util.Date ]");
    assert_eq!(result[1], "guideline: Review error handling");
}

#[test]
fn サブディレクトリでは親と子のルールが順に表示される() {
    let dir = dummy_project_path("nested/sub");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // 順序: rule → guideline, 親 → 子
    // ルート定義 = @ なし, sub 定義 = @ sub
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], "rule: no-legacy-date [ java.util.Date ]");
    assert_eq!(result[1], r"rule: no-wildcard-import [ import.*\*; ] @ sub");
    assert_eq!(result[2], "guideline: Review error handling");
    assert_eq!(result[3], "guideline: Check for code duplication @ sub");
}

// =============================================================================
// skip_middle: 中間ディレクトリにyamlがない場合
// =============================================================================

#[test]
fn 中間ディレクトリにyamlがなくてもルートルールは継承される() {
    let dir = dummy_project_path("skip_middle");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "rule: no-system-exit [ System.exit ]");
    assert_eq!(result[1], "guideline: Check exception handling");
}

#[test]
fn 深い階層でもルートルールは継承され相対パスで表示される() {
    let dir = dummy_project_path("skip_middle/level1/level2/level3");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // 親 (root) → 子 (level3) の順, @ は相対パス
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], "rule: no-system-exit [ System.exit ]");
    assert_eq!(result[1], "rule: no-raw-types [ List[^<], Map[^<] ] @ level1/level2/level3");
    assert_eq!(result[2], "guideline: Check exception handling");
    assert_eq!(result[3], "guideline: Check null safety @ level1/level2/level3");
}

#[test]
fn yamlがない中間ディレクトリではルートルールのみ表示される() {
    let dir = dummy_project_path("skip_middle/level1/level2");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // level2 has no yaml, should only have root rules
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "rule: no-system-exit [ System.exit ]");
    assert_eq!(result[1], "guideline: Check exception handling");
}

// =============================================================================
// deep_inherit: ルートyamlのみで深い階層
// =============================================================================

#[test]
fn 深い階層でもルートルールのみ継承される() {
    let dir = dummy_project_path("deep_inherit/a/b/c");
    let result = rec_lint::commands::show::run(&dir).unwrap();

    // ルート定義なので @ なし
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "rule: no-deprecated [ @Deprecated ]");
    assert_eq!(result[1], "guideline: Check API compatibility");
}
