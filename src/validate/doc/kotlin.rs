use super::{DocKind, DocViolation, KotlinDocConfig};
use crate::rule::parser::Visibility;

/// Validate Kotlin file for missing KDoc
pub fn validate(content: &str, config: &KotlinDocConfig) -> Vec<DocViolation> {
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

        // Check for block comment (skip non-kdoc comments)
        if line.starts_with("/*") && !line.starts_with("/**") {
            i = skip_block_comment(&lines, i);
            continue;
        }

        // Check if there's a KDoc before this line
        let has_kdoc = check_kdoc_before(&lines, i);

        // Check for type declaration
        if let Some(violation) = check_type_declaration(line, i + 1, has_kdoc, config) {
            violations.push(violation);
        }

        // Check for function declaration
        if let Some(violation) = check_function_declaration(line, i + 1, has_kdoc, config) {
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

fn check_kdoc_before(lines: &[&str], current: usize) -> bool {
    if current == 0 {
        return false;
    }

    let mut i = current - 1;

    // Skip annotations
    while i > 0 {
        let line = lines[i].trim();
        if line.starts_with('@') {
            i -= 1;
            continue;
        }
        break;
    }

    let line = lines[i].trim();

    // Check for end of KDoc
    if line.ends_with("*/") {
        if line.starts_with("/**") {
            return true;
        }
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

fn check_type_declaration(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.type_visibility.as_ref()?;

    // Skip comment lines
    if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
        return None;
    }

    // Kotlin type keywords
    let patterns = [
        (" class ", "class"),
        (" interface ", "interface"),
        (" object ", "object"),
        ("enum class ", "enum"),
        ("sealed class ", "sealed class"),
        ("data class ", "data class"),
        ("value class ", "value class"),
        ("annotation class ", "annotation class"),
    ];

    // Also check for line starting with these
    let start_patterns = [("class ", "class"), ("interface ", "interface"), ("object ", "object")];

    for (pattern, _kind) in patterns {
        if line.contains(pattern) {
            let is_public = !line.contains("private ") && !line.contains("internal ") && !line.contains("protected ");

            if *visibility == Visibility::Public && !is_public {
                return None;
            }

            if has_kdoc {
                return None;
            }

            let name = extract_class_name(line, pattern.trim());
            return Some(DocViolation { line: line_num, kind: DocKind::Type, name });
        }
    }

    for (pattern, _kind) in start_patterns {
        if line.starts_with(pattern) || line.starts_with(&format!("public {pattern}")) {
            let is_public = !line.contains("private ") && !line.contains("internal ") && !line.contains("protected ");

            if *visibility == Visibility::Public && !is_public {
                return None;
            }

            if has_kdoc {
                return None;
            }

            let name = extract_class_name(line, pattern.trim());
            return Some(DocViolation { line: line_num, kind: DocKind::Type, name });
        }
    }

    None
}

fn check_function_declaration(
    line: &str,
    line_num: usize,
    has_kdoc: bool,
    config: &KotlinDocConfig,
) -> Option<DocViolation> {
    let visibility = config.function_visibility.as_ref()?;

    // Skip comment lines
    if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
        return None;
    }

    // Kotlin function: fun functionName
    if !line.contains("fun ") {
        return None;
    }

    let is_public = !line.contains("private ") && !line.contains("internal ") && !line.contains("protected ");

    if *visibility == Visibility::Public && !is_public {
        return None;
    }

    if has_kdoc {
        return None;
    }

    // Extract function name
    let fun_pos = line.find("fun ")?;
    let after_fun = &line[fun_pos + 4..];

    // Skip generic type parameter if present: fun <T> name
    let name_start = if after_fun.starts_with('<') { after_fun.find('>')? + 1 } else { 0 };

    let name_part = after_fun[name_start..].trim();
    let name = name_part.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect::<String>();

    if name.is_empty() {
        return None;
    }

    Some(DocViolation { line: line_num, kind: DocKind::Function, name })
}

fn extract_class_name(line: &str, keyword: &str) -> String {
    let parts: Vec<&str> = line.split(keyword).collect();
    if parts.len() < 2 {
        return String::new();
    }

    let after = parts[1].trim();

    // Handle generic: class MyClass<T>
    after.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_all() -> KotlinDocConfig {
        KotlinDocConfig { type_visibility: Some(Visibility::All), function_visibility: Some(Visibility::All) }
    }

    fn config_public() -> KotlinDocConfig {
        KotlinDocConfig { type_visibility: Some(Visibility::Public), function_visibility: Some(Visibility::Public) }
    }

    // =========================================================================
    // Type declaration tests
    // =========================================================================

    #[test]
    fn test_class_without_kdoc() {
        let content = "class MyClass {}";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Type);
        assert_eq!(violations[0].name, "MyClass");
    }

    #[test]
    fn test_class_with_kdoc() {
        let content = r#"
/** This is a class */
class MyClass {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_data_class_without_kdoc() {
        let content = "data class User(val name: String)";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Type);
    }

    #[test]
    fn test_sealed_class_without_kdoc() {
        let content = "sealed class Result";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_object_without_kdoc() {
        let content = "object Singleton {}";
        let violations = validate(content, &config_all());
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_private_class_skipped_with_public_config() {
        let content = "private class MyClass {}";
        let violations = validate(content, &config_public());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_internal_class_skipped_with_public_config() {
        let content = "internal class MyClass {}";
        let violations = validate(content, &config_public());
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Function declaration tests
    // =========================================================================

    #[test]
    fn test_function_without_kdoc() {
        let content = "fun doSomething() {}";
        let config = KotlinDocConfig { type_visibility: None, function_visibility: Some(Visibility::All) };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].kind, DocKind::Function);
        assert_eq!(violations[0].name, "doSomething");
    }

    #[test]
    fn test_function_with_kdoc() {
        let content = r#"
/** Does something */
fun doSomething() {}
"#;
        let config = KotlinDocConfig { type_visibility: None, function_visibility: Some(Visibility::All) };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_private_function_skipped() {
        let content = "private fun helper() {}";
        let config = KotlinDocConfig { type_visibility: None, function_visibility: Some(Visibility::Public) };
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_generic_function() {
        let content = "fun <T> process(item: T) {}";
        let config = KotlinDocConfig { type_visibility: None, function_visibility: Some(Visibility::All) };
        let violations = validate(content, &config);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "process");
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_annotation_before_class() {
        let content = r#"
/** MyClass doc */
@Serializable
class MyClass {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_multiline_kdoc() {
        let content = r#"
/**
 * Multi-line KDoc
 */
class MyClass {}
"#;
        let violations = validate(content, &config_all());
        assert!(violations.is_empty());
    }

    #[test]
    fn test_empty_config_no_violations() {
        let content = "class MyClass {}";
        let config = KotlinDocConfig::default();
        let violations = validate(content, &config);
        assert!(violations.is_empty());
    }
}
