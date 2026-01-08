#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn run(case: &str, file: &str) -> Vec<String> {
    let file = common::project_file("forbidden_patterns", format!("{case}/{file}"));
    rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap()
}

fn assert_ok(case: &str, file: &str) {
    assert!(run(case, file).is_empty());
}

fn assert_violation(case: &str, file: &str, expected: &str) {
    let result = run(case, file);
    common::assert_output(&result, expected);
}

#[test]
fn 違反がない場合は空の結果を返す() {
    assert_ok("case01", "違反パターンなし.kt");
}

#[test]
fn リテラルパターンがマッチする() {
    assert_violation(
        "case02",
        "違反リテラルパターンあり.kt",
        "パターン違反: forbidden_patterns/case02/違反リテラルパターンあり.kt:1:1",
    );
}

#[test]
fn ワイルドカードパターンがマッチする() {
    assert_violation(
        "case03",
        "違反ワイルドカードパターンあり.kt",
        "パターン違反: forbidden_patterns/case03/違反ワイルドカードパターンあり.kt:1:1",
    );
}

#[test]
fn 文字クラスがマッチする() {
    assert_violation(
        "case04",
        "違反文字クラスパターンあり.kt",
        "パターン違反: forbidden_patterns/case04/違反文字クラスパターンあり.kt:1:2",
    );
}

#[test]
fn 単語境界で完全一致する() {
    assert_violation(
        "case05",
        "違反単語境界パターンあり.kt",
        "パターン違反: forbidden_patterns/case05/違反単語境界パターンあり.kt:1:5",
    );
}

#[test]
fn 単語境界は部分一致しない() {
    assert_ok("case06", "単語境界は部分一致しない.kt");
}

#[test]
fn 行頭アンカーがマッチする() {
    assert_violation(
        "case07",
        "違反行頭アンカーパターンあり.kt",
        "パターン違反: forbidden_patterns/case07/違反行頭アンカーパターンあり.kt:1:1",
    );
}

#[test]
fn 行頭アンカーは行中ではマッチしない() {
    assert_ok("case08", "行中の行頭アンカーパターンあり.kt");
}

#[test]
fn オプショナルパターンが両方にマッチする() {
    assert_violation(
        "case09",
        "違反オプショナルパターンあり.kt",
        r#"
            パターン違反: forbidden_patterns/case09/違反オプショナルパターンあり.kt:1:1
            パターン違反: forbidden_patterns/case09/違反オプショナルパターンあり.kt:2:1
        "#,
    );
}

#[test]
fn 選択パターンがいずれかにマッチする() {
    assert_violation(
        "case10",
        "違反選択パターンあり.kt",
        "パターン違反: forbidden_patterns/case10/違反選択パターンあり.kt:1:1",
    );
}

#[test]
fn 複数行の違反を検出できる() {
    assert_violation(
        "case11",
        "複数行に複数種類の違反パターンあり.kt",
        r#"
            パターン違反: forbidden_patterns/case11/複数行に複数種類の違反パターンあり.kt:1:1
            パターン違反: forbidden_patterns/case11/複数行に複数種類の違反パターンあり.kt:2:1
        "#,
    );
}

#[test]
fn 貪欲マッチでもカラム位置は正しい() {
    assert_violation(
        "case12",
        "違反貪欲マッチパターンあり.kt",
        "パターン違反: forbidden_patterns/case12/違反貪欲マッチパターンあり.kt:1:2",
    );
}

#[test]
fn ドットは任意の1文字にマッチする() {
    assert_violation(
        "case13",
        "違反任意の1文字マッチパターンあり.kt",
        "パターン違反: forbidden_patterns/case13/違反任意の1文字マッチパターンあり.kt:1:1",
    );
}

#[test]
fn デフォルトでは大文字小文字を区別する() {
    assert_violation(
        "case14",
        "大文字と小文字あり.kt",
        "パターン違反: forbidden_patterns/case14/大文字と小文字あり.kt:1:1",
    );
}

#[test]
fn 複数パターンでは先に定義されたパターンが優先される() {
    assert_violation(
        "case15",
        "複数パターンで先に定義されたパターンが優先.kt",
        "パターン違反: forbidden_patterns/case15/複数パターンで先に定義されたパターンが優先.kt:1:1",
    );
}

#[test]
fn 最初のパターンが無ければ次のパターンでマッチする() {
    assert_violation(
        "case16",
        "最初のパターンが無ければ次のパターンでマッチ.kt",
        "パターン違反2: forbidden_patterns/case16/最初のパターンが無ければ次のパターンでマッチ.kt:1:1",
    );
}

#[test]
fn 行番号は1から始まる() {
    assert_violation(
        "case17",
        "行番号は1から始まる.kt",
        "パターン違反: forbidden_patterns/case17/行番号は1から始まる.kt:1:1",
    );
}

#[test]
fn カラム位置は正規表現マッチの開始位置() {
    assert_violation(
        "case18",
        "カラム位置は正規表現マッチの開始位置.kt",
        "パターン違反: forbidden_patterns/case18/カラム位置は正規表現マッチの開始位置.kt:1:5",
    );
}

#[test]
fn 検出結果には行全体が含まれる() {
    assert_violation(
        "case19",
        "検出結果には行全体が含まれる.kt",
        "パターン違反: forbidden_patterns/case19/検出結果には行全体が含まれる.kt:1:8",
    );
}

#[test]
fn 大文字小文字を無視するフラグが使える() {
    assert_violation(
        "case20",
        "大文字小文字を無視するフラグ.kt",
        "パターン違反: forbidden_patterns/case20/大文字小文字を無視するフラグ.kt:1:1",
    );
}
