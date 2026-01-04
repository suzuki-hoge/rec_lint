use super::{DocKind, DocViolation, RustDocConfig};
use crate::rule::parser::Visibility;

/// Validate Rust file for missing RustDoc
pub fn validate(content: &str, config: &RustDocConfig) -> Vec<DocViolation> {
    let mut violations = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines and comments
        if line.is_empty() || is_comment_line(line) {
            i += 1;
            continue;
        }

        // Check for block comment (skip non-doc comments)
        if line.starts_with("/*") && !line.starts_with("/**") && !line.starts_with("/*!") {
            i = skip_block_comment(&lines, i);
            continue;
        }

        // Check if there's a RustDoc before this line
        let has_rustdoc = check_rustdoc_before(&lines, i);

        // Check each element type independently
        if let Some(v) = check_struct(line, i + 1, has_rustdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_enum(line, i + 1, has_rustdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_trait(line, i + 1, has_rustdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_type_alias(line, i + 1, has_rustdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_union(line, i + 1, has_rustdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_fn(line, i + 1, has_rustdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_macro_rules(line, i + 1, has_rustdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_mod(line, i + 1, has_rustdoc, config) {
            violations.push(v);
        }

        i += 1;
    }

    violations
}

fn skip_block_comment(lines: &[&str], start: usize) -> usize {
    let mut i = start;
    while i < lines.len() {
        if lines[i].contains("*/") {
            return i + 1;
        }
        i += 1;
    }
    lines.len()
}

fn check_rustdoc_before(lines: &[&str], current: usize) -> bool {
    if current == 0 {
        return false;
    }

    let mut i = current - 1;

    // Skip attributes
    while i > 0 {
        let line = lines[i].trim();
        if line.starts_with("#[") || line.starts_with("#![") {
            i -= 1;
            continue;
        }
        break;
    }

    let line = lines[i].trim();

    // Check for /// doc comment
    if line.starts_with("///") {
        return true;
    }

    // Check for /** doc comment */
    if line.ends_with("*/") {
        if line.starts_with("/**") {
            return true;
        }
        // Multi-line doc comment
        while i > 0 {
            i -= 1;
            let prev = lines[i].trim();
            if prev.starts_with("/**") {
                return true;
            }
            if prev.starts_with("/*") && !prev.starts_with("/**") {
                return false;
            }
        }
    }

    false
}

fn is_comment_line(line: &str) -> bool {
    // Skip regular comments but NOT doc comments (///, //!, /**, /*!)
    if line.starts_with("///") || line.starts_with("//!") {
        return false;
    }
    if line.starts_with("/**") || line.starts_with("/*!") {
        return false;
    }
    line.starts_with("//") || line.starts_with("/*") || line.starts_with("*")
}

fn check_visibility(line: &str, visibility: &Visibility) -> bool {
    let is_public = line.contains("pub ");
    match visibility {
        Visibility::Public => is_public,
        Visibility::All => true,
    }
}

fn extract_name_after(line: &str, keyword: &str) -> String {
    let pos = line.find(keyword);
    if pos.is_none() {
        return String::new();
    }

    let after = &line[pos.unwrap() + keyword.len()..];
    let trimmed = after.trim();

    // Handle generic: struct Foo<T>
    trimmed.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
}

// ============================================================================
// Individual element checkers
// ============================================================================

fn check_struct(line: &str, line_num: usize, has_rustdoc: bool, config: &RustDocConfig) -> Option<DocViolation> {
    let visibility = config.struct_.as_ref()?;

    if !line.contains("struct ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_rustdoc {
        return None;
    }

    let name = extract_name_after(line, "struct ");
    Some(DocViolation { line: line_num, kind: DocKind::Struct, name })
}

fn check_enum(line: &str, line_num: usize, has_rustdoc: bool, config: &RustDocConfig) -> Option<DocViolation> {
    let visibility = config.enum_.as_ref()?;

    if !line.contains("enum ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_rustdoc {
        return None;
    }

    let name = extract_name_after(line, "enum ");
    Some(DocViolation { line: line_num, kind: DocKind::Enum, name })
}

fn check_trait(line: &str, line_num: usize, has_rustdoc: bool, config: &RustDocConfig) -> Option<DocViolation> {
    let visibility = config.trait_.as_ref()?;

    if !line.contains("trait ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_rustdoc {
        return None;
    }

    let name = extract_name_after(line, "trait ");
    Some(DocViolation { line: line_num, kind: DocKind::Trait, name })
}

fn check_type_alias(line: &str, line_num: usize, has_rustdoc: bool, config: &RustDocConfig) -> Option<DocViolation> {
    let visibility = config.type_alias.as_ref()?;

    // type keyword but not in other contexts
    if !line.contains("type ") {
        return None;
    }

    // Exclude "impl ... for type" patterns
    if line.contains("impl ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_rustdoc {
        return None;
    }

    let name = extract_name_after(line, "type ");
    Some(DocViolation { line: line_num, kind: DocKind::TypeAlias, name })
}

fn check_union(line: &str, line_num: usize, has_rustdoc: bool, config: &RustDocConfig) -> Option<DocViolation> {
    let visibility = config.union.as_ref()?;

    if !line.contains("union ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_rustdoc {
        return None;
    }

    let name = extract_name_after(line, "union ");
    Some(DocViolation { line: line_num, kind: DocKind::Union, name })
}

fn check_fn(line: &str, line_num: usize, has_rustdoc: bool, config: &RustDocConfig) -> Option<DocViolation> {
    let visibility = config.fn_.as_ref()?;

    if !line.contains("fn ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_rustdoc {
        return None;
    }

    let name = extract_name_after(line, "fn ");
    Some(DocViolation { line: line_num, kind: DocKind::Fn, name })
}

fn check_macro_rules(line: &str, line_num: usize, has_rustdoc: bool, config: &RustDocConfig) -> Option<DocViolation> {
    let visibility = config.macro_rules.as_ref()?;

    if !line.contains("macro_rules!") {
        return None;
    }

    // macro_rules visibility is determined by #[macro_export] attribute, not pub
    // For simplicity, we treat all macro_rules as "public" if visibility is Public
    // and check all if visibility is All
    if *visibility == Visibility::Public {
        // Would need to check for #[macro_export] in previous lines
        // For simplicity, skip this check - always check when visibility is All
        return None;
    }

    if has_rustdoc {
        return None;
    }

    let name = extract_name_after(line, "macro_rules! ");
    Some(DocViolation { line: line_num, kind: DocKind::MacroRules, name })
}

fn check_mod(line: &str, line_num: usize, has_rustdoc: bool, config: &RustDocConfig) -> Option<DocViolation> {
    let visibility = config.mod_.as_ref()?;

    if !line.contains("mod ") {
        return None;
    }

    // Exclude "mod tests" and similar test modules
    if line.contains("mod tests") || line.contains("mod test") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_rustdoc {
        return None;
    }

    let name = extract_name_after(line, "mod ");
    // Remove trailing semicolon or brace from name
    let name = name.trim_end_matches(';').trim_end_matches('{').trim().to_string();
    Some(DocViolation { line: line_num, kind: DocKind::Mod, name })
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    // =========================================================================
    // struct
    // =========================================================================

    #[test]
    fn RustDocがないpub_structは違反になる() {
        let content = "pub struct MyStruct {}";
        let config = RustDocConfig { struct_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Struct);
        assert_eq!(violations[0].name, "MyStruct");
    }

    #[test]
    fn RustDocがあるstructは違反にならない() {
        let content = "/// Doc\npub struct MyStruct {}";
        let config = RustDocConfig { struct_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時は非pub_structをスキップする() {
        let content = "struct MyStruct {}";
        let config = RustDocConfig { struct_: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn struct検査が無効の場合は違反にならない() {
        let content = "pub struct MyStruct {}";
        let config = RustDocConfig { struct_: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn ジェネリックstructの名前を正しく抽出できる() {
        let content = "pub struct Container<T> {}";
        let config = RustDocConfig { struct_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "Container");
    }

    // =========================================================================
    // enum
    // =========================================================================

    #[test]
    fn RustDocがないpub_enumは違反になる() {
        let content = "pub enum MyEnum { A, B }";
        let config = RustDocConfig { enum_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Enum);
        assert_eq!(violations[0].name, "MyEnum");
    }

    #[test]
    fn RustDocがあるenumは違反にならない() {
        let content = "/// Doc\npub enum MyEnum { A }";
        let config = RustDocConfig { enum_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時は非pub_enumをスキップする() {
        let content = "enum MyEnum { A }";
        let config = RustDocConfig { enum_: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // trait
    // =========================================================================

    #[test]
    fn RustDocがないpub_traitは違反になる() {
        let content = "pub trait MyTrait {}";
        let config = RustDocConfig { trait_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Trait);
        assert_eq!(violations[0].name, "MyTrait");
    }

    #[test]
    fn RustDocがあるtraitは違反にならない() {
        let content = "/// Doc\npub trait MyTrait {}";
        let config = RustDocConfig { trait_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時は非pub_traitをスキップする() {
        let content = "trait MyTrait {}";
        let config = RustDocConfig { trait_: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // 型エイリアス
    // =========================================================================

    #[test]
    fn RustDocがないpub_type_aliasは違反になる() {
        let content = "pub type MyType = String;";
        let config = RustDocConfig { type_alias: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::TypeAlias);
        assert_eq!(violations[0].name, "MyType");
    }

    #[test]
    fn RustDocがある型エイリアスは違反にならない() {
        let content = "/// Doc\npub type MyType = String;";
        let config = RustDocConfig { type_alias: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時は非pub型エイリアスをスキップする() {
        let content = "type MyType = String;";
        let config = RustDocConfig { type_alias: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // union
    // =========================================================================

    #[test]
    fn RustDocがないpub_unionは違反になる() {
        let content = "pub union MyUnion { a: i32, b: f32 }";
        let config = RustDocConfig { union: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Union);
        assert_eq!(violations[0].name, "MyUnion");
    }

    #[test]
    fn RustDocがあるunionは違反にならない() {
        let content = "/// Doc\npub union MyUnion { a: i32 }";
        let config = RustDocConfig { union: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // 関数
    // =========================================================================

    #[test]
    fn RustDocがないpub_fnは違反になる() {
        let content = "pub fn do_something() {}";
        let config = RustDocConfig { fn_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Fn);
        assert_eq!(violations[0].name, "do_something");
    }

    #[test]
    fn RustDocがある関数は違反にならない() {
        let content = "/// Doc\npub fn do_something() {}";
        let config = RustDocConfig { fn_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時は非pub関数をスキップする() {
        let content = "fn do_something() {}";
        let config = RustDocConfig { fn_: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn 関数検査が無効の場合は違反にならない() {
        let content = "pub fn do_something() {}";
        let config = RustDocConfig { fn_: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // macro_rules
    // =========================================================================

    #[test]
    fn RustDocがないmacro_rulesは違反になる() {
        let content = "macro_rules! my_macro { () => {} }";
        let config = RustDocConfig { macro_rules: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::MacroRules);
        assert_eq!(violations[0].name, "my_macro");
    }

    #[test]
    fn RustDocがあるmacro_rulesは違反にならない() {
        let content = "/// Doc\nmacro_rules! my_macro { () => {} }";
        let config = RustDocConfig { macro_rules: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn macro_rules検査が無効の場合は違反にならない() {
        let content = "macro_rules! my_macro { () => {} }";
        let config = RustDocConfig { macro_rules: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // モジュール
    // =========================================================================

    #[test]
    fn RustDocがないpubモジュールは違反になる() {
        let content = "pub mod mymodule;";
        let config = RustDocConfig { mod_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Mod);
        assert_eq!(violations[0].name, "mymodule");
    }

    #[test]
    fn RustDocがあるモジュールは違反にならない() {
        let content = "/// Doc\npub mod mymodule;";
        let config = RustDocConfig { mod_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn public指定時は非pubモジュールをスキップする() {
        let content = "mod mymodule;";
        let config = RustDocConfig { mod_: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn testsモジュールは検査対象外() {
        let content = "mod tests {}";
        let config = RustDocConfig { mod_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn インラインモジュールも検査対象() {
        let content = "pub mod mymodule {}";
        let config = RustDocConfig { mod_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "mymodule");
    }

    // =========================================================================
    // エッジケース
    // =========================================================================

    #[test]
    fn アトリビュート付きstructのRustDocを認識する() {
        let content = "/// Doc\n#[derive(Debug)]\npub struct MyStruct {}";
        let config = RustDocConfig { struct_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn 複数行RustDocを認識する() {
        let content = "/// First line\n/// Second line\npub struct MyStruct {}";
        let config = RustDocConfig { struct_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn ブロックドキュメントコメントを認識する() {
        let content = "/** Block doc */\npub struct MyStruct {}";
        let config = RustDocConfig { struct_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn 通常のコメントはRustDocとして認識されない() {
        let content = "// Not rustdoc\npub struct MyStruct {}";
        let config = RustDocConfig { struct_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn 空の設定では違反が検出されない() {
        let content = "pub struct MyStruct {}\npub fn foo() {}";
        let config = RustDocConfig::default();
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn 複数の要素を同時に検査できる() {
        let content = "pub struct A {}\npub enum B {}\npub trait C {}";
        let config = RustDocConfig {
            struct_: Some(Visibility::All),
            enum_: Some(Visibility::All),
            trait_: Some(Visibility::All),
            ..Default::default()
        };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 3);
    }
}
