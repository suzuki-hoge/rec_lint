use super::{DocKind, DocViolation, PhpDocConfig};
use crate::rule::parser::Visibility;

/// Validate PHP file for missing PHPDoc
pub fn validate(content: &str, config: &PhpDocConfig) -> Vec<DocViolation> {
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

        // Check for block comment (skip non-PHPDoc comments)
        if line.starts_with("/*") && !line.starts_with("/**") {
            i = skip_block_comment(&lines, i);
            continue;
        }

        // Check if there's a PHPDoc before this line
        let has_phpdoc = check_phpdoc_before(&lines, i);

        // Check each element type independently
        if let Some(v) = check_class(line, i + 1, has_phpdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_interface(line, i + 1, has_phpdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_trait(line, i + 1, has_phpdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_enum(line, i + 1, has_phpdoc, config) {
            violations.push(v);
        } else if let Some(v) = check_function(line, i + 1, has_phpdoc, config) {
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

fn check_phpdoc_before(lines: &[&str], current: usize) -> bool {
    if current == 0 {
        return false;
    }

    // Look backwards for PHPDoc (/** ... */)
    let mut i = current - 1;

    // Skip PHP 8 attributes like #[Test] or #[Route(...)]
    while i > 0 {
        let line = lines[i].trim();
        if line.starts_with("#[") {
            i -= 1;
            continue;
        }
        break;
    }

    let line = lines[i].trim();

    // Check for end of PHPDoc on this line
    if line.ends_with("*/") {
        // Could be single-line: /** comment */
        if line.starts_with("/**") {
            return true;
        }
        // Multi-line PHPDoc - look for start
        while i > 0 {
            i -= 1;
            let prev = lines[i].trim();
            if prev.starts_with("/**") {
                return true;
            }
            if prev.starts_with("/*") && !prev.starts_with("/**") {
                return false; // Regular comment, not PHPDoc
            }
        }
    }

    false
}

/// Check for class declaration
fn check_class(line: &str, line_num: usize, has_phpdoc: bool, config: &PhpDocConfig) -> Option<DocViolation> {
    let visibility = config.class.as_ref()?;

    // PHP class declaration patterns:
    // class Foo {}
    // abstract class Foo {}
    // final class Foo {}
    // readonly class Foo {}
    // public class Foo {} (rare but valid)

    // Must contain "class " but not "enum class" style
    let pos = line.find("class ")?;
    let before = &line[..pos];

    // Exclude if it's not a class declaration (e.g., "new class" for anonymous class)
    if before.contains("new") {
        return None;
    }

    // Check visibility
    if !check_visibility(before, visibility) {
        return None;
    }

    if has_phpdoc {
        return None;
    }

    let name = extract_identifier(&line[pos + 6..]);
    Some(DocViolation { line: line_num, kind: DocKind::Class, name })
}

/// Check for interface declaration
fn check_interface(line: &str, line_num: usize, has_phpdoc: bool, config: &PhpDocConfig) -> Option<DocViolation> {
    let visibility = config.interface.as_ref()?;

    let pos = line.find("interface ")?;
    let before = &line[..pos];

    if !check_visibility(before, visibility) {
        return None;
    }

    if has_phpdoc {
        return None;
    }

    let name = extract_identifier(&line[pos + 10..]);
    Some(DocViolation { line: line_num, kind: DocKind::Interface, name })
}

/// Check for trait declaration
fn check_trait(line: &str, line_num: usize, has_phpdoc: bool, config: &PhpDocConfig) -> Option<DocViolation> {
    let visibility = config.trait_.as_ref()?;

    let pos = line.find("trait ")?;
    let before = &line[..pos];

    if !check_visibility(before, visibility) {
        return None;
    }

    if has_phpdoc {
        return None;
    }

    let name = extract_identifier(&line[pos + 6..]);
    Some(DocViolation { line: line_num, kind: DocKind::Trait, name })
}

/// Check for enum declaration (PHP 8.1+)
fn check_enum(line: &str, line_num: usize, has_phpdoc: bool, config: &PhpDocConfig) -> Option<DocViolation> {
    let visibility = config.enum_.as_ref()?;

    let pos = line.find("enum ")?;
    let before = &line[..pos];

    if !check_visibility(before, visibility) {
        return None;
    }

    if has_phpdoc {
        return None;
    }

    let name = extract_identifier(&line[pos + 5..]);
    Some(DocViolation { line: line_num, kind: DocKind::Enum, name })
}

/// Check for function/method declaration
fn check_function(line: &str, line_num: usize, has_phpdoc: bool, config: &PhpDocConfig) -> Option<DocViolation> {
    let visibility = config.function.as_ref()?;

    // Must contain "function "
    let pos = line.find("function ")?;
    let before = &line[..pos];

    // Exclude closures/anonymous functions (contain "= function" or "use")
    if before.contains('=') || line.contains(" use ") || line.contains(" use(") {
        return None;
    }

    // Skip if it's inside class/interface/trait/enum declaration (already handled)
    if before.contains("class ")
        || before.contains("interface ")
        || before.contains("trait ")
        || before.contains("enum ")
    {
        return None;
    }

    if !check_visibility(before, visibility) {
        return None;
    }

    if has_phpdoc {
        return None;
    }

    // Extract function name (after "function ")
    let after = &line[pos + 9..];
    let name = extract_identifier(after);

    // Skip constructors and magic methods for now (they often have documentation in class)
    if name.starts_with("__") {
        return None;
    }

    Some(DocViolation { line: line_num, kind: DocKind::Function, name })
}

fn is_comment_line(line: &str) -> bool {
    line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") || line.starts_with("#")
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
