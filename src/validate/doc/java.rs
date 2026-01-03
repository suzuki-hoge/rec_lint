use super::{DocKind, DocViolation, JavaDocConfig};
use crate::rule::parser::Visibility;

/// Validate Java file for missing JavaDoc
pub fn validate(content: &str, config: &JavaDocConfig) -> Vec<DocViolation> {
    let mut violations = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines and single-line comments
        if line.is_empty() || line.starts_with("//") {
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

        // Check for type declaration
        if let Some(violation) = check_type_declaration(line, i + 1, has_javadoc, config) {
            violations.push(violation);
        }

        // Check for method declaration
        if let Some(violation) = check_method_declaration(line, i + 1, has_javadoc, config) {
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
            let prev = lines[i].trim();
            if prev.starts_with("/**") {
                return true;
            }
            if prev.starts_with("/*") && !prev.starts_with("/**") {
                return false; // Regular comment, not JavaDoc
            }
            i -= 1;
        }
    }

    false
}

fn check_type_declaration(
    line: &str,
    line_num: usize,
    has_javadoc: bool,
    config: &JavaDocConfig,
) -> Option<DocViolation> {
    let visibility = config.type_visibility.as_ref()?;

    // Skip comment lines (they may contain keywords like "class" in text)
    if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
        return None;
    }

    // Check for class, interface, enum, record, @interface
    let type_keywords = ["class ", "interface ", "enum ", "record ", "@interface "];

    for keyword in type_keywords {
        if let Some(pos) = line.find(keyword) {
            let before = &line[..pos];

            // Check visibility
            let is_public = before.contains("public");
            if *visibility == Visibility::Public && !is_public {
                return None;
            }

            // Skip if has javadoc
            if has_javadoc {
                return None;
            }

            // Extract name
            let after = &line[pos + keyword.len()..];
            let name = extract_identifier(after);

            return Some(DocViolation { line: line_num, kind: DocKind::Type, name });
        }
    }

    None
}

fn check_method_declaration(
    line: &str,
    line_num: usize,
    has_javadoc: bool,
    config: &JavaDocConfig,
) -> Option<DocViolation> {
    let visibility = config.function_visibility.as_ref()?;

    // Skip comment lines
    if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
        return None;
    }

    // Method pattern: visibility? modifiers? ReturnType methodName(
    // Skip constructors (uppercase first letter), fields (no parens or =)

    if line.contains(" class ")
        || line.contains(" interface ")
        || line.contains(" enum ")
        || line.contains(" record ")
        || line.contains("=")
        || !line.contains('(')
    {
        return None;
    }

    let trimmed = line.trim();

    // Skip if looks like constructor
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    // Find method name (word before '(')
    let paren_pos = trimmed.find('(')?;
    let before_paren = &trimmed[..paren_pos];
    let words: Vec<&str> = before_paren.split_whitespace().collect();

    if words.len() < 2 {
        return None; // Need at least return type and name
    }

    let method_name = words.last()?;

    // Constructor check: name starts with uppercase and matches a type keyword pattern
    if method_name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
        // This is likely a constructor
        return None;
    }

    let is_public = before_paren.contains("public");

    if *visibility == Visibility::Public && !is_public {
        return None;
    }

    if has_javadoc {
        return None;
    }

    Some(DocViolation { line: line_num, kind: DocKind::Function, name: method_name.to_string() })
}

fn extract_identifier(s: &str) -> String {
    s.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_all() -> JavaDocConfig {
        JavaDocConfig { type_visibility: Some(Visibility::All), function_visibility: Some(Visibility::All) }
    }

    fn config_public() -> JavaDocConfig {
        JavaDocConfig { type_visibility: Some(Visibility::Public), function_visibility: Some(Visibility::Public) }
    }

    // =========================================================================
    // Type declaration tests
    // =========================================================================

    #[test]
    fn test_class_without_javadoc() {
        let content = "public class MyClass {}";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Type);
        assert_eq!(violations[0].name, "MyClass");
    }

    #[test]
    fn test_class_with_javadoc() {
        let content = r#"
/** This is a class */
public class MyClass {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_interface_without_javadoc() {
        let content = "public interface MyInterface {}";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Type);
        assert_eq!(violations[0].name, "MyInterface");
    }

    #[test]
    fn test_enum_without_javadoc() {
        let content = "public enum MyEnum { A, B }";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Type);
    }

    #[test]
    fn test_record_without_javadoc() {
        let content = "public record MyRecord(String name) {}";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Type);
    }

    #[test]
    fn test_private_class_skipped_with_public_config() {
        let content = "class MyClass {}";
        let violations = validate(content, &config_public());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_multiline_javadoc() {
        let content = r#"
/**
 * This is a multi-line
 * JavaDoc comment.
 */
public class MyClass {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Method declaration tests
    // =========================================================================

    #[test]
    fn test_method_without_javadoc() {
        let content = r#"
public class MyClass {
    public void doSomething() {}
}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.iter().any(|v| v.kind == DocKind::Function && v.name == "doSomething"));
    }

    #[test]
    fn test_method_with_javadoc() {
        let content = r#"
public class MyClass {
    /** Does something */
    public void doSomething() {}
}
"#;
        let config = JavaDocConfig {
            type_visibility: None, // Skip type check
            function_visibility: Some(Visibility::All),
        };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_private_method_skipped_with_public_config() {
        let content = r#"
public class MyClass {
    private void helper() {}
}
"#;
        let config = JavaDocConfig { type_visibility: None, function_visibility: Some(Visibility::Public) };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_annotation_before_class() {
        let content = r#"
/** MyClass doc */
@Component
public class MyClass {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_regular_comment_not_javadoc() {
        let content = r#"
/* This is not JavaDoc */
public class MyClass {}
"#;
        let violations = validate(content, &config_all());
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_empty_config_no_violations() {
        let content = "public class MyClass {}";
        let config = JavaDocConfig::default();
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }
}
