use std::path::Path;

use super::{ExternalFileTestConfig, TestExistenceViolation, TestExistenceViolationKind};
use crate::rule::parser::TestRequireLevel;

/// Validate test existence for a PHP source file
pub fn validate(
    file_path: &Path,
    content: &str,
    root_dir: &Path,
    config: &ExternalFileTestConfig,
) -> Vec<TestExistenceViolation> {
    let mut violations = Vec::new();

    // Extract class name from file
    let class_name = match extract_class_name(content) {
        Some(name) => name,
        None => return violations, // No class found, skip validation
    };

    // Build expected test file path using namespace
    let source_namespace = extract_namespace_path(content);

    // Build test file path: {test_directory}/{namespace_path}/{ClassName}{suffix}.php
    let test_file_name = format!("{}{}.php", class_name, config.test_file_suffix);
    let test_path = if let Some(ns_path) = &source_namespace {
        root_dir.join(&config.test_directory).join(ns_path).join(&test_file_name)
    } else {
        // Fallback to file-path based mapping
        let relative_path = file_path.strip_prefix(root_dir).unwrap_or(file_path);
        build_test_path_from_file(relative_path, root_dir, config)
    };

    // Check if test file exists and has matching namespace
    let test_exists = if test_path.exists() {
        // Check namespace matches if source has a namespace
        if let Some(ref src_ns) = source_namespace {
            let test_content = std::fs::read_to_string(&test_path).unwrap_or_default();
            let test_ns = extract_namespace_path(&test_content);
            test_ns.as_ref() == Some(src_ns)
        } else {
            true
        }
    } else {
        false
    };

    if !test_exists {
        // Also try the file-path based approach if namespace didn't work
        let fallback_path = {
            let relative_path = file_path.strip_prefix(root_dir).unwrap_or(file_path);
            build_test_path_from_file(relative_path, root_dir, config)
        };

        let fallback_exists = if test_path != fallback_path && fallback_path.exists() {
            // Check namespace matches for fallback path too
            if let Some(ref src_ns) = source_namespace {
                let test_content = std::fs::read_to_string(&fallback_path).unwrap_or_default();
                let test_ns = extract_namespace_path(&test_content);
                test_ns.as_ref() == Some(src_ns)
            } else {
                true
            }
        } else {
            false
        };

        if !fallback_exists {
            // Neither path exists with matching namespace
            let expected = test_path.strip_prefix(root_dir).unwrap_or(&test_path);
            violations.push(TestExistenceViolation {
                kind: TestExistenceViolationKind::MissingTestFile { expected_path: expected.display().to_string() },
            });
            return violations;
        }
    }

    // For all_public mode, check that all public methods are tested
    if config.require == TestRequireLevel::AllPublic {
        let test_content = if test_path.exists() {
            std::fs::read_to_string(&test_path).unwrap_or_default()
        } else {
            let relative_path = file_path.strip_prefix(root_dir).unwrap_or(file_path);
            let fallback_path = build_test_path_from_file(relative_path, root_dir, config);
            std::fs::read_to_string(fallback_path).unwrap_or_default()
        };

        let public_methods = extract_public_methods(content);
        for (line, method_name) in public_methods {
            if !test_content.contains(&method_name) {
                violations.push(TestExistenceViolation {
                    kind: TestExistenceViolationKind::UntestedPublicMethod { line, method_name },
                });
            }
        }
    }

    violations
}

/// Build test file path from source file path
fn build_test_path_from_file(
    relative_path: &Path,
    root_dir: &Path,
    config: &ExternalFileTestConfig,
) -> std::path::PathBuf {
    // Remove common source prefixes like src/, src/main/php/, etc.
    let path_str = relative_path.to_string_lossy();
    let stripped = path_str
        .strip_prefix("src/main/php/")
        .or_else(|| path_str.strip_prefix("src/"))
        .or_else(|| path_str.strip_prefix("app/"))
        .unwrap_or(&path_str);

    // Replace .php with {suffix}.php
    let test_file = if let Some(base) = stripped.strip_suffix(".php") {
        format!("{}{}.php", base, config.test_file_suffix)
    } else {
        stripped.to_string()
    };

    root_dir.join(&config.test_directory).join(test_file)
}

/// Extract namespace path from PHP source
fn extract_namespace_path(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("namespace ") {
            // namespace App\Service; -> App/Service
            let ns = trimmed.strip_prefix("namespace ")?.trim_end_matches(';').trim();
            return Some(ns.replace('\\', "/"));
        }
    }
    None
}

/// Extract class name from PHP source
fn extract_class_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        // Match: class ClassName, abstract class ClassName, final class ClassName
        if let Some(class_pos) = trimmed.find("class ") {
            // Make sure "class" is preceded by nothing or a keyword
            let before = &trimmed[..class_pos];
            if before.is_empty()
                || before.ends_with("abstract ")
                || before.ends_with("final ")
                || before.ends_with("readonly ")
            {
                let after_class = &trimmed[class_pos + 6..];
                let name: String = after_class.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
    }
    None
}

/// Extract public method names from PHP source
fn extract_public_methods(content: &str) -> Vec<(usize, String)> {
    let mut methods = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let line_num = i + 1;

        // Look for public function declarations
        if trimmed.contains("public") && trimmed.contains("function ") {
            if let Some(func_pos) = trimmed.find("function ") {
                let after_function = &trimmed[func_pos + 9..];
                if let Some(name) = extract_method_name(after_function) {
                    // Skip constructor and magic methods
                    if !name.starts_with("__") {
                        methods.push((line_num, name));
                    }
                }
            }
        }
    }

    methods
}

/// Extract method name from function declaration
fn extract_method_name(line: &str) -> Option<String> {
    // Look for pattern: methodName(
    let paren_pos = line.find('(')?;
    let name_part = &line[..paren_pos];

    // Clean up: handle visibility/type prefixes that might appear
    let clean_name: String = name_part.trim().chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();

    if clean_name.is_empty() {
        None
    } else {
        Some(clean_name)
    }
}
