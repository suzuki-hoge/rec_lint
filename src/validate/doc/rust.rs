use super::{DocKind, DocViolation, RustDocConfig};
use crate::rule::parser::Visibility;

/// Validate Rust file for missing RustDoc
pub fn validate(content: &str, config: &RustDocConfig) -> Vec<DocViolation> {
    let mut violations = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines
        if line.is_empty() {
            i += 1;
            continue;
        }

        // Skip regular comments (not doc comments)
        if line.starts_with("//") && !line.starts_with("///") && !line.starts_with("//!") {
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

        // Check for type declaration
        if let Some(violation) = check_type_declaration(line, i + 1, has_rustdoc, config) {
            violations.push(violation);
        }

        // Check for function declaration
        if let Some(violation) = check_function_declaration(line, i + 1, has_rustdoc, config) {
            violations.push(violation);
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
            let prev = lines[i].trim();
            if prev.starts_with("/**") {
                return true;
            }
            if prev.starts_with("/*") && !prev.starts_with("/**") {
                return false;
            }
            i -= 1;
        }
    }

    false
}

fn check_type_declaration(
    line: &str,
    line_num: usize,
    has_rustdoc: bool,
    config: &RustDocConfig,
) -> Option<DocViolation> {
    let visibility = config.type_visibility.as_ref()?;

    // Rust type keywords
    let patterns = ["struct ", "enum ", "trait ", "type ", "union "];

    for pattern in patterns {
        if line.contains(pattern) {
            let is_public = line.contains("pub ");

            if *visibility == Visibility::Public && !is_public {
                return None;
            }

            if has_rustdoc {
                return None;
            }

            let name = extract_name_after(line, pattern);

            return Some(DocViolation { line: line_num, kind: DocKind::Type, name });
        }
    }

    None
}

fn check_function_declaration(
    line: &str,
    line_num: usize,
    has_rustdoc: bool,
    config: &RustDocConfig,
) -> Option<DocViolation> {
    let visibility = config.function_visibility.as_ref()?;

    // Rust function: fn name or pub fn name
    if !line.contains("fn ") {
        return None;
    }

    // Skip trait method declarations (inside trait block)
    // This is a simplified check - full detection would require tracking context

    let is_public = line.contains("pub ");

    if *visibility == Visibility::Public && !is_public {
        return None;
    }

    if has_rustdoc {
        return None;
    }

    let name = extract_name_after(line, "fn ");

    Some(DocViolation { line: line_num, kind: DocKind::Function, name })
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

#[cfg(test)]
mod tests {
    use super::*;

    fn config_all() -> RustDocConfig {
        RustDocConfig { type_visibility: Some(Visibility::All), function_visibility: Some(Visibility::All) }
    }

    fn config_public() -> RustDocConfig {
        RustDocConfig { type_visibility: Some(Visibility::Public), function_visibility: Some(Visibility::Public) }
    }

    // =========================================================================
    // Type declaration tests
    // =========================================================================

    #[test]
    fn test_struct_without_rustdoc() {
        let content = "pub struct MyStruct {}";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Type);
        assert_eq!(violations[0].name, "MyStruct");
    }

    #[test]
    fn test_struct_with_rustdoc() {
        let content = r#"
/// This is a struct
pub struct MyStruct {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_enum_without_rustdoc() {
        let content = "pub enum MyEnum { A, B }";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Type);
    }

    #[test]
    fn test_trait_without_rustdoc() {
        let content = "pub trait MyTrait {}";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Type);
    }

    #[test]
    fn test_type_alias_without_rustdoc() {
        let content = "pub type MyType = String;";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_private_struct_skipped_with_public_config() {
        let content = "struct MyStruct {}";
        let violations = validate(content, &config_public());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_multiline_rustdoc() {
        let content = r#"
/// First line
/// Second line
pub struct MyStruct {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_block_doc_comment() {
        let content = r#"
/** Block doc comment */
pub struct MyStruct {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Function declaration tests
    // =========================================================================

    #[test]
    fn test_function_without_rustdoc() {
        let content = "pub fn do_something() {}";
        let config = RustDocConfig { type_visibility: None, function_visibility: Some(Visibility::All) };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Function);
        assert_eq!(violations[0].name, "do_something");
    }

    #[test]
    fn test_function_with_rustdoc() {
        let content = r#"
/// Does something
pub fn do_something() {}
"#;
        let config = RustDocConfig { type_visibility: None, function_visibility: Some(Visibility::All) };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_private_function_skipped() {
        let content = "fn helper() {}";
        let config = RustDocConfig { type_visibility: None, function_visibility: Some(Visibility::Public) };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_attribute_before_struct() {
        let content = r#"
/// MyStruct doc
#[derive(Debug)]
pub struct MyStruct {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_regular_comment_not_rustdoc() {
        let content = r#"
// This is not rustdoc
pub struct MyStruct {}
"#;
        let violations = validate(content, &config_all());
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_empty_config_no_violations() {
        let content = "pub struct MyStruct {}";
        let config = RustDocConfig::default();
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_generic_struct() {
        let content = "pub struct Container<T> {}";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "Container");
    }
}
