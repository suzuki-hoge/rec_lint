use super::{DocKind, DocViolation, KotlinDocConfig};
use crate::rule::parser::Visibility;

/// Validate Kotlin file for missing KDoc
pub fn validate(content: &str, config: &KotlinDocConfig) -> Vec<DocViolation> {
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

        // Check for block comment (skip non-kdoc comments)
        if line.starts_with("/*") && !line.starts_with("/**") {
            i = skip_block_comment(&lines, i);
            continue;
        }

        // Check if there's a KDoc before this line
        let has_kdoc = check_kdoc_before(&lines, i);

        // Check each element type independently (order matters for specificity)
        if let Some(v) = check_enum_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_sealed_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_sealed_interface(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_data_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_value_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_annotation_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_class(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_interface(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_object(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_typealias(line, i + 1, has_kdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_function(line, i + 1, has_kdoc, config) {
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
    line.starts_with("//") || line.starts_with("/*") || line.starts_with("*")
}

fn check_visibility(line: &str, visibility: &Visibility) -> bool {
    let is_public = !line.contains("private ") && !line.contains("internal ") && !line.contains("protected ");
    match visibility {
        Visibility::Public => is_public,
        Visibility::All => true,
    }
}

// ============================================================================
// Individual element checkers
// ============================================================================

fn check_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.class.as_ref()?;

    // Check for " class " but exclude specific class types
    if !line.contains(" class ") && !line.starts_with("class ") {
        return None;
    }

    // Exclude other class types
    if line.contains("enum class")
        || line.contains("sealed class")
        || line.contains("data class")
        || line.contains("value class")
        || line.contains("annotation class")
    {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "class");
    Some(DocViolation { line: line_num, kind: DocKind::Class, name })
}

fn check_interface(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.interface.as_ref()?;

    if !line.contains(" interface ") && !line.starts_with("interface ") {
        return None;
    }

    // Exclude sealed interface
    if line.contains("sealed interface") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "interface");
    Some(DocViolation { line: line_num, kind: DocKind::Interface, name })
}

fn check_object(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.object.as_ref()?;

    if !line.contains(" object ") && !line.starts_with("object ") {
        return None;
    }

    // Exclude companion object
    if line.contains("companion object") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "object");
    Some(DocViolation { line: line_num, kind: DocKind::Object, name })
}

fn check_enum_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.enum_class.as_ref()?;

    if !line.contains("enum class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "enum class");
    Some(DocViolation { line: line_num, kind: DocKind::EnumClass, name })
}

fn check_sealed_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.sealed_class.as_ref()?;

    if !line.contains("sealed class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "sealed class");
    Some(DocViolation { line: line_num, kind: DocKind::SealedClass, name })
}

fn check_sealed_interface(
    line: &str,
    line_num: usize,
    has_kdoc: bool,
    config: &KotlinDocConfig,
) -> Option<DocViolation> {
    let visibility = config.sealed_interface.as_ref()?;

    if !line.contains("sealed interface ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "sealed interface");
    Some(DocViolation { line: line_num, kind: DocKind::SealedInterface, name })
}

fn check_data_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.data_class.as_ref()?;

    if !line.contains("data class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "data class");
    Some(DocViolation { line: line_num, kind: DocKind::DataClass, name })
}

fn check_value_class(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.value_class.as_ref()?;

    if !line.contains("value class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "value class");
    Some(DocViolation { line: line_num, kind: DocKind::ValueClass, name })
}

fn check_annotation_class(
    line: &str,
    line_num: usize,
    has_kdoc: bool,
    config: &KotlinDocConfig,
) -> Option<DocViolation> {
    let visibility = config.annotation_class.as_ref()?;

    if !line.contains("annotation class ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "annotation class");
    Some(DocViolation { line: line_num, kind: DocKind::AnnotationClass, name })
}

fn check_typealias(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.typealias.as_ref()?;

    if !line.contains("typealias ") {
        return None;
    }

    if !check_visibility(line, visibility) {
        return None;
    }

    if has_kdoc {
        return None;
    }

    let name = extract_class_name(line, "typealias");
    Some(DocViolation { line: line_num, kind: DocKind::Typealias, name })
}

fn check_function(line: &str, line_num: usize, has_kdoc: bool, config: &KotlinDocConfig) -> Option<DocViolation> {
    let visibility = config.function.as_ref()?;

    if !line.contains("fun ") {
        return None;
    }

    if !check_visibility(line, visibility) {
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
    after.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
}
