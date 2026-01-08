#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_case(relative: &str) -> Vec<String> {
    let file = common::project_file("require_php_doc", relative);
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
    expect_ok("class/case01/all指定_すべてのclassにドキュメントがある.php");
}

#[test]
fn class_all指定で一部のclassにドキュメントがないと違反になる() {
    expect_violation(
        "class/case02/all指定_一部のclassにしかドキュメントがない.php",
        "PHPDocを書いてください (class MissingDocClass): require_php_doc/class/case02/all指定_一部のclassにしかドキュメントがない.php:7:1",
    );
}

#[test]
fn class_public指定ですべてのpublicなclassにドキュメントがあると違反にならない() {
    expect_ok("class/case03/public指定_すべてのpublicなclassにドキュメントがある.php");
}

// Note: PHPのclassはファイルレベルで定義されるため、public指定でもすべてのclassが対象になります
// case04テストは、PHPのpublic classの概念が他言語と異なるため省略

// interface tests

#[test]
fn interface_all指定ですべてのinterfaceにドキュメントがあると違反にならない() {
    expect_ok("interface/case01/all指定_すべてのinterfaceにドキュメントがある.php");
}

#[test]
fn interface_all指定で一部のinterfaceにドキュメントがないと違反になる() {
    expect_violation(
        "interface/case02/all指定_一部のinterfaceにしかドキュメントがない.php",
        "PHPDocを書いてください (interface MissingDocInterface): require_php_doc/interface/case02/all指定_一部のinterfaceにしかドキュメントがない.php:7:1",
    );
}

// trait tests

#[test]
fn trait_all指定ですべてのtraitにドキュメントがあると違反にならない() {
    expect_ok("trait/case01/all指定_すべてのtraitにドキュメントがある.php");
}

#[test]
fn trait_all指定で一部のtraitにドキュメントがないと違反になる() {
    expect_violation(
        "trait/case02/all指定_一部のtraitにしかドキュメントがない.php",
        "PHPDocを書いてください (trait MissingDocTrait): require_php_doc/trait/case02/all指定_一部のtraitにしかドキュメントがない.php:7:1",
    );
}

// function tests

#[test]
fn function_all指定ですべてのfunctionにドキュメントがあると違反にならない() {
    expect_ok("function/case01/all指定_すべてのfunctionにドキュメントがある.php");
}

#[test]
fn function_all指定で一部のfunctionにドキュメントがないと違反になる() {
    expect_violation(
        "function/case02/all指定_一部のfunctionにしかドキュメントがない.php",
        "PHPDocを書いてください (function missingDocFunction): require_php_doc/function/case02/all指定_一部のfunctionにしかドキュメントがない.php:7:1",
    );
}

#[test]
fn function指定でもマジックメソッドはエラーにならない() {
    expect_ok("function/case03/all指定_マジックメソッドがある.php");
}

#[test]
fn function指定でもクロージャはエラーにならない() {
    expect_ok("function/case04/all指定_クロージャがある.php");
}

// enum tests

#[test]
fn enum_all指定ですべてのenumにドキュメントがあると違反にならない() {
    expect_ok("enum/case01/all指定_すべてのenumにドキュメントがある.php");
}

#[test]
fn enum_all指定で一部のenumにドキュメントがないと違反になる() {
    expect_violation(
        "enum/case02/all指定_一部のenumにしかドキュメントがない.php",
        "PHPDocを書いてください (enum MissingDocStatus): require_php_doc/enum/case02/all指定_一部のenumにしかドキュメントがない.php:9:1",
    );
}

// all tests

#[test]
fn all指定ですべての種類にドキュメントがあると違反にならない() {
    expect_ok("all/case01/all指定_複数種類にドキュメントがある.php");
}

#[test]
fn all指定で複数種類にドキュメントがないと違反になる() {
    expect_violation(
        "all/case02/all指定_複数種類にドキュメントがない.php",
        r#"
            PHPDocを書いてください (class MissingClass): require_php_doc/all/case02/all指定_複数種類にドキュメントがない.php:2:1
            PHPDocを書いてください (function missingFunction): require_php_doc/all/case02/all指定_複数種類にドキュメントがない.php:6:1
            PHPDocを書いてください (interface MissingInterface): require_php_doc/all/case02/all指定_複数種類にドキュメントがない.php:4:1
        "#,
    );
}
