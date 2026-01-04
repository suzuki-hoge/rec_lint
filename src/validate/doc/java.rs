use super::{DocKind, DocViolation, JavaDocConfig};
use crate::rule::parser::Visibility;

/// Validate Java file for missing JavaDoc
pub fn validate(content: &str, config: &JavaDocConfig) -> Vec<DocViolation> {
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

        // Check for block comment (skip non-javadoc comments)
        if line.starts_with("/*") && !line.starts_with("/**") {
            i = skip_block_comment(&lines, i);
            continue;
        }

        // Check if there's a JavaDoc before this line
        let has_javadoc = check_javadoc_before(&lines, i);

        // Check each element type independently
        if let Some(v) = check_class(line, i + 1, has_javadoc, config) {
            violations.push(v);
        } else if let Some(v) = check_interface(line, i + 1, has_javadoc, config) {
            violations.push(v);
        } else if let Some(v) = check_enum(line, i + 1, has_javadoc, config) {
            violations.push(v);
        } else if let Some(v) = check_record(line, i + 1, has_javadoc, config) {
            violations.push(v);
        } else if let Some(v) = check_annotation(line, i + 1, has_javadoc, config) {
            violations.push(v);
        } else if let Some(v) = check_method(line, i + 1, has_javadoc, config) {
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

fn check_javadoc_before(lines: &[&str], current: usize) -> bool {
    if current == 0 {
        return false;
    }

    // Look backwards for JavaDoc (/** ... */)
    let mut i = current - 1;

    // Skip annotations
    while i > 0 {
        let line = lines[i].trim();
        if line.starts_with('@') && !line.starts_with("@Override") {
            i -= 1;
            continue;
        }
        break;
    }

    let line = lines[i].trim();

    // Check for end of JavaDoc on this line
    if line.ends_with("*/") {
        // Could be single-line: /** comment */
        if line.starts_with("/**") {
            return true;
        }
        // Multi-line JavaDoc - look for start
        while i > 0 {
            i -= 1;
            let prev = lines[i].trim();
            if prev.starts_with("/**") {
                return true;
            }
            if prev.starts_with("/*") && !prev.starts_with("/**") {
                return false; // Regular comment, not JavaDoc
            }
        }
    }

    false
}

/// Check for class declaration
fn check_class(line: &str, line_num: usize, has_javadoc: bool, config: &JavaDocConfig) -> Option<DocViolation> {
    let visibility = config.class.as_ref()?;

    // Must contain " class " but not other type keywords before it
    let pos = line.find(" class ")?;
    let before = &line[..pos];

    // Exclude: enum class, sealed class, data class, etc. (Kotlin keywords that might appear)
    if before.contains("enum") || before.contains("sealed") || before.contains("data") || before.contains("value") {
        return None;
    }

    // Check visibility
    if !check_visibility(before, visibility) {
        return None;
    }

    if has_javadoc {
        return None;
    }

    let name = extract_identifier(&line[pos + 7..]);
    Some(DocViolation { line: line_num, kind: DocKind::Class, name })
}

/// Check for interface declaration
fn check_interface(line: &str, line_num: usize, has_javadoc: bool, config: &JavaDocConfig) -> Option<DocViolation> {
    let visibility = config.interface.as_ref()?;

    // Must contain " interface " but not "@interface"
    if line.contains("@interface") {
        return None;
    }

    let pos = line.find(" interface ")?;
    let before = &line[..pos];

    // Exclude sealed interface (Kotlin)
    if before.contains("sealed") {
        return None;
    }

    if !check_visibility(before, visibility) {
        return None;
    }

    if has_javadoc {
        return None;
    }

    let name = extract_identifier(&line[pos + 11..]);
    Some(DocViolation { line: line_num, kind: DocKind::Interface, name })
}

/// Check for enum declaration
fn check_enum(line: &str, line_num: usize, has_javadoc: bool, config: &JavaDocConfig) -> Option<DocViolation> {
    let visibility = config.enum_.as_ref()?;

    let pos = line.find(" enum ")?;
    let before = &line[..pos];

    if !check_visibility(before, visibility) {
        return None;
    }

    if has_javadoc {
        return None;
    }

    let name = extract_identifier(&line[pos + 6..]);
    Some(DocViolation { line: line_num, kind: DocKind::Enum, name })
}

/// Check for record declaration
fn check_record(line: &str, line_num: usize, has_javadoc: bool, config: &JavaDocConfig) -> Option<DocViolation> {
    let visibility = config.record.as_ref()?;

    let pos = line.find(" record ")?;
    let before = &line[..pos];

    if !check_visibility(before, visibility) {
        return None;
    }

    if has_javadoc {
        return None;
    }

    let name = extract_identifier(&line[pos + 8..]);
    Some(DocViolation { line: line_num, kind: DocKind::Record, name })
}

/// Check for annotation type declaration (@interface)
fn check_annotation(line: &str, line_num: usize, has_javadoc: bool, config: &JavaDocConfig) -> Option<DocViolation> {
    let visibility = config.annotation.as_ref()?;

    let pos = line.find("@interface ")?;
    let before = &line[..pos];

    if !check_visibility(before, visibility) {
        return None;
    }

    if has_javadoc {
        return None;
    }

    let name = extract_identifier(&line[pos + 11..]);
    Some(DocViolation { line: line_num, kind: DocKind::Annotation, name })
}

/// Check for method declaration
fn check_method(line: &str, line_num: usize, has_javadoc: bool, config: &JavaDocConfig) -> Option<DocViolation> {
    let visibility = config.method.as_ref()?;

    // Method must have parentheses and not be a type declaration
    if !line.contains('(') {
        return None;
    }

    // Exclude type declarations
    if line.contains(" class ")
        || line.contains(" interface ")
        || line.contains(" enum ")
        || line.contains(" record ")
        || line.contains("@interface ")
    {
        return None;
    }

    // Exclude field declarations (contain =)
    if line.contains('=') {
        return None;
    }

    let paren_pos = line.find('(')?;
    let before_paren = &line[..paren_pos];
    let words: Vec<&str> = before_paren.split_whitespace().collect();

    // Need at least return type and method name
    if words.len() < 2 {
        return None;
    }

    let method_name = words.last()?;

    // Skip constructors (name starts with uppercase)
    if method_name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
        return None;
    }

    if !check_visibility(before_paren, visibility) {
        return None;
    }

    if has_javadoc {
        return None;
    }

    Some(DocViolation { line: line_num, kind: DocKind::Method, name: method_name.to_string() })
}

fn is_comment_line(line: &str) -> bool {
    line.starts_with("//") || line.starts_with("/*") || line.starts_with("*")
}

fn check_visibility(before: &str, visibility: &Visibility) -> bool {
    let is_public = before.contains("public");
    match visibility {
        Visibility::Public => is_public,
        Visibility::All => true,
    }
}

fn extract_identifier(s: &str) -> String {
    s.trim().chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Class tests
    // =========================================================================

    #[test]
    fn test_class_without_javadoc() {
        let content = "public class MyClass {}";
        let config = JavaDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Class);
        assert_eq!(violations[0].name, "MyClass");
    }

    #[test]
    fn test_class_with_javadoc() {
        let content = "/** Doc */\npublic class MyClass {}";
        let config = JavaDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_class_public_only_skips_private() {
        let content = "class MyClass {}";
        let config = JavaDocConfig { class: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_class_disabled() {
        let content = "public class MyClass {}";
        let config = JavaDocConfig { class: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Interface tests
    // =========================================================================

    #[test]
    fn test_interface_without_javadoc() {
        let content = "public interface MyInterface {}";
        let config = JavaDocConfig { interface: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Interface);
        assert_eq!(violations[0].name, "MyInterface");
    }

    #[test]
    fn test_interface_with_javadoc() {
        let content = "/** Doc */\npublic interface MyInterface {}";
        let config = JavaDocConfig { interface: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_interface_public_only_skips_private() {
        let content = "interface MyInterface {}";
        let config = JavaDocConfig { interface: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_interface_disabled() {
        let content = "public interface MyInterface {}";
        let config = JavaDocConfig { interface: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Enum tests
    // =========================================================================

    #[test]
    fn test_enum_without_javadoc() {
        let content = "public enum MyEnum { A, B }";
        let config = JavaDocConfig { enum_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Enum);
        assert_eq!(violations[0].name, "MyEnum");
    }

    #[test]
    fn test_enum_with_javadoc() {
        let content = "/** Doc */\npublic enum MyEnum { A }";
        let config = JavaDocConfig { enum_: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_enum_public_only_skips_private() {
        let content = "enum MyEnum { A }";
        let config = JavaDocConfig { enum_: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_enum_disabled() {
        let content = "public enum MyEnum { A }";
        let config = JavaDocConfig { enum_: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Record tests
    // =========================================================================

    #[test]
    fn test_record_without_javadoc() {
        let content = "public record MyRecord(String name) {}";
        let config = JavaDocConfig { record: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Record);
        assert_eq!(violations[0].name, "MyRecord");
    }

    #[test]
    fn test_record_with_javadoc() {
        let content = "/** Doc */\npublic record MyRecord(String name) {}";
        let config = JavaDocConfig { record: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_record_public_only_skips_private() {
        let content = "record MyRecord(String name) {}";
        let config = JavaDocConfig { record: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_record_disabled() {
        let content = "public record MyRecord(String name) {}";
        let config = JavaDocConfig { record: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Annotation tests
    // =========================================================================

    #[test]
    fn test_annotation_without_javadoc() {
        let content = "public @interface MyAnnotation {}";
        let config = JavaDocConfig { annotation: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Annotation);
        assert_eq!(violations[0].name, "MyAnnotation");
    }

    #[test]
    fn test_annotation_with_javadoc() {
        let content = "/** Doc */\npublic @interface MyAnnotation {}";
        let config = JavaDocConfig { annotation: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_annotation_public_only_skips_private() {
        let content = "@interface MyAnnotation {}";
        let config = JavaDocConfig { annotation: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_annotation_disabled() {
        let content = "public @interface MyAnnotation {}";
        let config = JavaDocConfig { annotation: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Method tests
    // =========================================================================

    #[test]
    fn test_method_without_javadoc() {
        let content = "public void doSomething() {}";
        let config = JavaDocConfig { method: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Method);
        assert_eq!(violations[0].name, "doSomething");
    }

    #[test]
    fn test_method_with_javadoc() {
        let content = "/** Doc */\npublic void doSomething() {}";
        let config = JavaDocConfig { method: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_method_public_only_skips_private() {
        let content = "void doSomething() {}";
        let config = JavaDocConfig { method: Some(Visibility::Public), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_method_disabled() {
        let content = "public void doSomething() {}";
        let config = JavaDocConfig { method: None, ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_method_skips_constructor() {
        let content = "public MyClass() {}";
        let config = JavaDocConfig { method: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_annotation_before_class() {
        let content = "/** Doc */\n@Component\npublic class MyClass {}";
        let config = JavaDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_multiline_javadoc() {
        let content = "/**\n * Multi-line\n */\npublic class MyClass {}";
        let config = JavaDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_regular_comment_not_javadoc() {
        let content = "/* Not JavaDoc */\npublic class MyClass {}";
        let config = JavaDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_multiline_regular_comment_not_javadoc() {
        let content = "/*\n * Not JavaDoc\n */\npublic class MyClass {}";
        let config = JavaDocConfig { class: Some(Visibility::All), ..Default::default() };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_empty_config_no_violations() {
        let content = "public class MyClass {}\npublic void foo() {}";
        let config = JavaDocConfig::default();
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_multiple_elements() {
        let content = "public class A {}\npublic interface B {}\npublic enum C {}";
        let config = JavaDocConfig {
            class: Some(Visibility::All),
            interface: Some(Visibility::All),
            enum_: Some(Visibility::All),
            ..Default::default()
        };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 3);
    }
}
