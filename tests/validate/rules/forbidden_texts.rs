#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn run(case: &str, file: &str) -> Vec<String> {
    let file = common::project_file("forbidden_texts", format!("{case}/{file}"));
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
    assert_ok("case01", "違反キーワードなし.kt");
}

#[test]
fn キーワードがマッチすると違反が検出される() {
    assert_violation(
        "case02",
        "行中に違反キーワードあり.kt",
        "テキスト違反: forbidden_texts/case02/行中に違反キーワードあり.kt:1:1",
    );
}

#[test]
fn 行中のキーワードも検出される() {
    assert_violation(
        "case03",
        "同一行に複数の違反キーワードあり.kt",
        "テキスト違反: forbidden_texts/case03/同一行に複数の違反キーワードあり.kt:1:1",
    );
}

#[test]
fn 複数キーワードでは先に定義されたものが優先される() {
    assert_violation(
        "case04",
        "複数キーワード指定で２つめの違反キーワードあり.kt",
        "テキスト違反: forbidden_texts/case04/複数キーワード指定で２つめの違反キーワードあり.kt:1:1",
    );
}

#[test]
fn 複数行の違反を検出できる() {
    assert_violation(
        "case05",
        "複数行に複数種類の違反キーワードあり.kt",
        r#"
            テキスト違反: forbidden_texts/case05/複数行に複数種類の違反キーワードあり.kt:1:1
            テキスト違反: forbidden_texts/case05/複数行に複数種類の違反キーワードあり.kt:2:1
        "#,
    );
}

#[test]
fn 行末のキーワードも検出される() {
    assert_violation(
        "case06",
        "行末に違反キーワードあり.kt",
        "テキスト違反: forbidden_texts/case06/行末に違反キーワードあり.kt:1:6",
    );
}

#[test]
fn 大文字小文字を区別する() {
    assert_violation(
        "case07",
        "大文字キーワードと小文字キーワードあり.kt",
        "テキスト違反: forbidden_texts/case07/大文字キーワードと小文字キーワードあり.kt:2:1",
    );
}

#[test]
fn 部分文字列もマッチする() {
    assert_violation(
        "case08",
        "キーワードを含む長い文字列あり.kt",
        "テキスト違反: forbidden_texts/case08/キーワードを含む長い文字列あり.kt:1:8",
    );
}
