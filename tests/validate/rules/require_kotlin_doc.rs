#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_case(relative: &str) -> Vec<String> {
    let file = common::project_file("require_kotlin_doc", relative);
    rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap()
}

fn expect_ok(relative: &str) {
    let result = validate_case(relative);
    assert!(result.is_empty(), "expected no violations for {relative}, got {result:?}");
}

fn expect_violation(relative: &str, expected: &str) {
    let result = validate_case(relative);
    common::assert_output(&result, expected);
}

// class tests

#[test]
fn class_all指定ですべてのclassにドキュメントがあると違反にならない() {
    expect_ok("class/case01/all指定_すべてのclassにドキュメントがある.kt");
}

#[test]
fn class_all指定で一部のclassにドキュメントがないと違反になる() {
    expect_violation(
        "class/case02/all指定_一部のclassにしかドキュメントがない.kt",
        "KDocを書いてください (class MissingDocClass): require_kotlin_doc/class/case02/all指定_一部のclassにしかドキュメントがない.kt:6:1",
    );
}

// interface tests

#[test]
fn interface_all指定ですべてのinterfaceにドキュメントがあると違反にならない() {
    expect_ok("interface/case01/all指定_すべてのinterfaceにドキュメントがある.kt");
}

#[test]
fn interface_all指定で一部のinterfaceにドキュメントがないと違反になる() {
    expect_violation(
        "interface/case02/all指定_一部のinterfaceにしかドキュメントがない.kt",
        "KDocを書いてください (interface MissingDocInterface): require_kotlin_doc/interface/case02/all指定_一部のinterfaceにしかドキュメントがない.kt:6:1",
    );
}

// function tests

#[test]
fn function_all指定ですべてのfunctionにドキュメントがあると違反にならない() {
    expect_ok("function/case01/all指定_すべてのfunctionにドキュメントがある.kt");
}

#[test]
fn function_all指定で一部のfunctionにドキュメントがないと違反になる() {
    expect_violation(
        "function/case02/all指定_一部のfunctionにしかドキュメントがない.kt",
        "KDocを書いてください (function missingDocFunction): require_kotlin_doc/function/case02/all指定_一部のfunctionにしかドキュメントがない.kt:6:1",
    );
}

// all tests

#[test]
fn all指定ですべての種類にドキュメントがあると違反にならない() {
    expect_ok("all/case01/all指定_複数種類にドキュメントがある.kt");
}

#[test]
fn all指定で複数種類にドキュメントがないと違反になる() {
    expect_violation(
        "all/case02/all指定_複数種類にドキュメントがない.kt",
        r#"
            KDocを書いてください (class MissingClass): require_kotlin_doc/all/case02/all指定_複数種類にドキュメントがない.kt:1:1
            KDocを書いてください (function missingFunction): require_kotlin_doc/all/case02/all指定_複数種類にドキュメントがない.kt:5:1
            KDocを書いてください (interface MissingInterface): require_kotlin_doc/all/case02/all指定_複数種類にドキュメントがない.kt:3:1
        "#,
    );
}
