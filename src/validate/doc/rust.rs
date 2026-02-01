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
    let is_public = line.contains("pub ") || line.contains("pub(");
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
