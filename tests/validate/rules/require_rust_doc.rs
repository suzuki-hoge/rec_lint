#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_case(relative: &str) -> Vec<String> {
    let file = common::project_file("require_rust_doc", relative);
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

#[test]
fn struct_all指定ですべてのstructにドキュメントがあると違反にならない() {
    expect_ok("struct/case01/all指定_すべてのstructにドキュメントがある.rs");
}

#[test]
fn struct_all指定で一部のstructにドキュメントがないと違反になる() {
    expect_violation(
        "struct/case02/all指定_一部のstructにしかドキュメントがない.rs",
        r#"
            RustDocを書いてください (struct MissingDoc): require_rust_doc/struct/case02/all指定_一部のstructにしかドキュメントがない.rs:4:1
            RustDocを書いてください (struct PrivateStruct): require_rust_doc/struct/case02/all指定_一部のstructにしかドキュメントがない.rs:6:1
        "#,
    );
}

#[test]
fn struct_public指定ですべてのpublicなstructにドキュメントがあると違反にならない() {
    expect_ok("struct/case03/public指定_すべてのpublicなstructにドキュメントがある.rs");
}

#[test]
fn struct_public指定ですべてのpublicなstructにドキュメントがないと違反になる() {
    expect_violation(
        "struct/case04/public指定_すべてのpublicなstructにはドキュメントがない.rs",
        r#"
            RustDocを書いてください (struct MissingPublicDoc): require_rust_doc/struct/case04/public指定_すべてのpublicなstructにはドキュメントがない.rs:1:1
        "#,
    );
}

#[test]
fn enum_all指定ですべてのenumにドキュメントがあると違反にならない() {
    expect_ok("enum/case01/all指定_すべてのenumにドキュメントがある.rs");
}

#[test]
fn enum_public指定ですべてのpublicなenumにドキュメントがないと違反になる() {
    expect_violation(
        "enum/case02/public指定_すべてのpublicなenumにはドキュメントがない.rs",
        r#"
            RustDocを書いてください (enum Missing): require_rust_doc/enum/case02/public指定_すべてのpublicなenumにはドキュメントがない.rs:1:1
        "#,
    );
}

#[test]
fn trait_all指定ですべてのtraitにドキュメントがあると違反にならない() {
    expect_ok("trait/case01/all指定_すべてのtraitにドキュメントがある.rs");
}

#[test]
fn trait_public指定ですべてのpublicなtraitにドキュメントがないと違反になる() {
    expect_violation(
        "trait/case02/public指定_すべてのpublicなtraitにはドキュメントがない.rs",
        r#"
            RustDocを書いてください (trait MissingTrait): require_rust_doc/trait/case02/public指定_すべてのpublicなtraitにはドキュメントがない.rs:1:1
        "#,
    );
}

#[test]
fn typealias_all指定ですべてのtypealiasにドキュメントがあると違反にならない() {
    expect_ok("type_alias/case01/all指定_すべてのtypealiasにドキュメントがある.rs");
}

#[test]
fn typealias_public指定ですべてのpublicなtypealiasにドキュメントがないと違反になる() {
    expect_violation(
        "type_alias/case02/public指定_すべてのpublicなtypealiasにはドキュメントがない.rs",
        r#"
            RustDocを書いてください (type MissingAlias): require_rust_doc/type_alias/case02/public指定_すべてのpublicなtypealiasにはドキュメントがない.rs:1:1
        "#,
    );
}

#[test]
fn union_all指定ですべてのunionにドキュメントがあると違反にならない() {
    expect_ok("union/case01/all指定_すべてのunionにドキュメントがある.rs");
}

#[test]
fn union_public指定ですべてのpublicなunionにドキュメントがないと違反になる() {
    expect_violation(
        "union/case02/public指定_すべてのpublicなunionにはドキュメントがない.rs",
        r#"
            RustDocを書いてください (union MissingUnion): require_rust_doc/union/case02/public指定_すべてのpublicなunionにはドキュメントがない.rs:1:1
        "#,
    );
}

#[test]
fn fn_public指定ですべてのpublicな関数にドキュメントがあると違反にならない() {
    expect_ok("fn/case01/public指定_すべてのpublicなfnにドキュメントがある.rs");
}

#[test]
fn fn_public指定ですべてのpublicな関数にドキュメントがないと違反になる() {
    expect_violation(
        "fn/case02/public指定_すべてのpublicなfnにはドキュメントがない.rs",
        r#"
            RustDocを書いてください (fn missing): require_rust_doc/fn/case02/public指定_すべてのpublicなfnにはドキュメントがない.rs:1:1
        "#,
    );
}

#[test]
fn macro_all指定ですべてのmacroにドキュメントがあると違反にならない() {
    expect_ok("macro_rules/case01/all指定_すべてのmacroにドキュメントがある.rs");
}

#[test]
fn macro_all指定ですべてのmacroにドキュメントがないと違反になる() {
    expect_violation(
        "macro_rules/case02/all指定_すべてのmacroにはドキュメントがない.rs",
        r#"
            RustDocを書いてください (macro_rules missing): require_rust_doc/macro_rules/case02/all指定_すべてのmacroにはドキュメントがない.rs:1:1
        "#,
    );
}

#[test]
fn mod_public指定ですべてのpublicなmodにドキュメントがあると違反にならない() {
    expect_ok("mod/case01/public指定_すべてのpublicなmodにドキュメントがある.rs");
}

#[test]
fn mod_public指定ですべてのpublicなmodにドキュメントがないと違反になる() {
    expect_violation(
        "mod/case02/public指定_すべてのpublicなmodにはドキュメントがない.rs",
        r#"
            RustDocを書いてください (mod missing): require_rust_doc/mod/case02/public指定_すべてのpublicなmodにはドキュメントがない.rs:1:1
        "#,
    );
}

#[test]
fn all指定ですべての種類にドキュメントがあると違反にならない() {
    expect_ok("all/case01/all指定_複数種類にドキュメントがある.rs");
}

#[test]
fn all指定で複数種類にドキュメントがないと違反になる() {
    expect_violation(
        "all/case02/all指定_複数種類にドキュメントがない.rs",
        r#"
            RustDocを書いてください (enum MissingEnum): require_rust_doc/all/case02/all指定_複数種類にドキュメントがない.rs:3:1
            RustDocを書いてください (fn missing_fn): require_rust_doc/all/case02/all指定_複数種類にドキュメントがない.rs:5:1
            RustDocを書いてください (struct MissingStruct): require_rust_doc/all/case02/all指定_複数種類にドキュメントがない.rs:1:1
        "#,
    );
}
