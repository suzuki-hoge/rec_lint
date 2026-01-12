use std::path::Path;

use super::{ExternalFileTestConfig, TestExistenceViolation, TestExistenceViolationKind};
use crate::rule::parser::TestRequireLevel;

/// Validate test existence for a Kotlin source file
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

    // Skip validation if there are no public methods to test
    let public_methods = extract_public_methods(content);
    if public_methods.is_empty() {
        return violations;
    }

    // Build expected test file path using package
    let source_package = extract_package_path(content);

    // Build test file path: {test_directory}/{package_path}/{ClassName}{suffix}.kt
    let test_file_name = format!("{}{}.kt", class_name, config.test_file_suffix);
    let test_path = if let Some(pkg_path) = &source_package {
        root_dir.join(&config.test_directory).join(pkg_path).join(&test_file_name)
    } else {
        // Fallback to file-path based mapping
        let relative_path = file_path.strip_prefix(root_dir).unwrap_or(file_path);
        build_test_path_from_file(relative_path, root_dir, config)
    };

    // Check if test file exists and has matching package
    let test_exists = if test_path.exists() {
        // Check package matches if source has a package
        if let Some(ref src_pkg) = source_package {
            let test_content = std::fs::read_to_string(&test_path).unwrap_or_default();
            let test_pkg = extract_package_path(&test_content);
            test_pkg.as_ref() == Some(src_pkg)
        } else {
            true
        }
    } else {
        false
    };

    if !test_exists {
        // Also try the file-path based approach if package didn't work
        let fallback_path = {
            let relative_path = file_path.strip_prefix(root_dir).unwrap_or(file_path);
            build_test_path_from_file(relative_path, root_dir, config)
        };

        let fallback_exists = if test_path != fallback_path && fallback_path.exists() {
            // Check package matches for fallback path too
            if let Some(ref src_pkg) = source_package {
                let test_content = std::fs::read_to_string(&fallback_path).unwrap_or_default();
                let test_pkg = extract_package_path(&test_content);
                test_pkg.as_ref() == Some(src_pkg)
            } else {
                true
            }
        } else {
            false
        };

        if !fallback_exists {
            // Neither path exists with matching package
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
    // Remove common source prefixes like src/main/kotlin/, src/, etc.
    let path_str = relative_path.to_string_lossy();
    let stripped = path_str
        .strip_prefix("src/main/kotlin/")
        .or_else(|| path_str.strip_prefix("src/main/java/"))
        .or_else(|| path_str.strip_prefix("src/"))
        .unwrap_or(&path_str);

    // Replace .kt with {suffix}.kt
    let test_file = if let Some(base) = stripped.strip_suffix(".kt") {
        format!("{}{}.kt", base, config.test_file_suffix)
    } else {
        stripped.to_string()
    };

    root_dir.join(&config.test_directory).join(test_file)
}

/// Extract package path from Kotlin source
fn extract_package_path(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("package ") {
            // package com.example.service -> com/example/service
            let pkg = trimmed.strip_prefix("package ")?.trim();
            return Some(pkg.replace('.', "/"));
        }
    }
    None
}

/// Extract class name from Kotlin source
fn extract_class_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        // Match: class ClassName, data class, sealed class, object, etc.
        for prefix in
            ["class ", "data class ", "sealed class ", "object ", "interface ", "abstract class ", "open class "]
        {
            if let Some(pos) = trimmed.find(prefix) {
                let after_prefix = &trimmed[pos + prefix.len()..];
                let name: String = after_prefix.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
    }
    None
}

/// Extract public method names from Kotlin source
fn extract_public_methods(content: &str) -> Vec<(usize, String)> {
    let mut methods = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let line_num = i + 1;

        // Look for public function declarations (public is default in Kotlin)
        // Skip private, protected, internal
        if trimmed.contains("fun ")
            && !trimmed.contains("private ")
            && !trimmed.contains("protected ")
            && !trimmed.contains("internal ")
        {
            if let Some(fun_pos) = trimmed.find("fun ") {
                let after_fun = &trimmed[fun_pos + 4..];
                if let Some(paren_pos) = after_fun.find('(') {
                    let name: String = after_fun[..paren_pos]
                        .trim()
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !name.is_empty() {
                        methods.push((line_num, name));
                    }
                }
            }
        }
    }

    methods
}
