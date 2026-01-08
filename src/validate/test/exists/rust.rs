use std::path::Path;

use super::{RustTestConfig, TestExistenceViolation, TestExistenceViolationKind};
use crate::rule::parser::TestRequireLevelRust;

/// Validate test existence for a Rust source file
pub fn validate(
    file_path: &Path,
    content: &str,
    root_dir: &Path,
    config: &RustTestConfig,
) -> Vec<TestExistenceViolation> {
    let mut violations = Vec::new();

    // Check unit tests
    if let Some(unit_config) = &config.unit {
        let has_test_module = has_test_module_or_function(content);

        if !has_test_module {
            violations.push(TestExistenceViolation { kind: TestExistenceViolationKind::MissingUnitTest });
        } else if unit_config.require == TestRequireLevelRust::AllPublic {
            // Check that all public functions are tested
            let test_content = extract_test_module_content(content);
            let public_functions = extract_public_functions(content);

            for (line, func_name) in public_functions {
                if !test_content.contains(&func_name) {
                    violations.push(TestExistenceViolation {
                        kind: TestExistenceViolationKind::UntestedPublicFunction { line, function_name: func_name },
                    });
                }
            }
        }
    }

    // Check integration tests
    if let Some(integration_config) = &config.integration {
        let relative_path = file_path.strip_prefix(root_dir).unwrap_or(file_path);
        let test_path = build_integration_test_path(relative_path, root_dir, integration_config, &config.suffix);

        if !test_path.exists() {
            let expected = test_path.strip_prefix(root_dir).unwrap_or(&test_path);
            violations.push(TestExistenceViolation {
                kind: TestExistenceViolationKind::MissingIntegrationTestFile {
                    expected_path: expected.display().to_string(),
                },
            });
        } else if integration_config.require == TestRequireLevelRust::AllPublic {
            // Check that all public functions are tested in integration tests
            let test_content = std::fs::read_to_string(&test_path).unwrap_or_default();
            let public_functions = extract_public_functions(content);

            for (line, func_name) in public_functions {
                if !test_content.contains(&func_name) {
                    violations.push(TestExistenceViolation {
                        kind: TestExistenceViolationKind::UntestedPublicFunction { line, function_name: func_name },
                    });
                }
            }
        }
    }

    violations
}

/// Build integration test file path
fn build_integration_test_path(
    relative_path: &Path,
    root_dir: &Path,
    config: &super::RustIntegrationTestConfig,
    suffix: &str,
) -> std::path::PathBuf {
    // Remove src/ prefix if present
    let path_str = relative_path.to_string_lossy();
    let stripped = path_str.strip_prefix("src/").unwrap_or(&path_str);

    // Replace .rs with {suffix}.rs (or just .rs if suffix is empty)
    let test_file = if let Some(base) = stripped.strip_suffix(".rs") {
        if suffix.is_empty() {
            format!("{base}.rs")
        } else {
            format!("{base}{suffix}.rs")
        }
    } else {
        stripped.to_string()
    };

    root_dir.join(&config.test_directory).join(test_file)
}

/// Check if content has a test module or test functions
fn has_test_module_or_function(content: &str) -> bool {
    // Check for #[cfg(test)] module
    if content.contains("#[cfg(test)]") {
        return true;
    }

    // Check for #[test] attribute
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("#[test]")
            || trimmed.starts_with("#[tokio::test]")
            || trimmed.starts_with("#[actix_web::test]")
            || trimmed.starts_with("#[actix_rt::test]")
            || trimmed.starts_with("#[async_std::test]")
        {
            return true;
        }
    }

    false
}

/// Extract content from test module (everything after #[cfg(test)])
fn extract_test_module_content(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_test_module = false;
    let mut test_content = String::new();

    for line in lines {
        if line.trim().starts_with("#[cfg(test)]") {
            in_test_module = true;
            continue;
        }

        if in_test_module {
            test_content.push_str(line);
            test_content.push('\n');
        }
    }

    test_content
}

/// Extract public function names from Rust source (outside of test module)
fn extract_public_functions(content: &str) -> Vec<(usize, String)> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_test_module = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let line_num = i + 1;

        // Track if we're in test module
        if trimmed.starts_with("#[cfg(test)]") {
            in_test_module = true;
            continue;
        }

        // Skip test module content
        if in_test_module {
            continue;
        }

        // Look for pub fn declarations
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("pub async fn ") {
            if let Some(name) = extract_fn_name(trimmed) {
                functions.push((line_num, name));
            }
        }
    }

    functions
}

/// Extract function name from a fn declaration line
fn extract_fn_name(line: &str) -> Option<String> {
    // Look for pattern: pub fn name( or pub async fn name(
    let fn_pos = line.find("fn ")?;
    let after_fn = &line[fn_pos + 3..];
    let name_start = after_fn.trim();

    // Extract identifier
    let name: String = name_start.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn テストモジュールを検出できる() {
        let content = r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
"#;
        assert!(has_test_module_or_function(content));
    }

    #[test]
    fn test属性を検出できる() {
        let content = r#"
#[test]
fn test_something() {
}
"#;
        assert!(has_test_module_or_function(content));
    }

    #[test]
    fn テストがない場合は検出されない() {
        let content = r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;
        assert!(!has_test_module_or_function(content));
    }

    #[test]
    fn pub関数を抽出できる() {
        let content = r#"
pub fn public_function() {}
fn private_function() {}
pub fn another_public() {}
"#;
        let functions = extract_public_functions(content);
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].1, "public_function");
        assert_eq!(functions[1].1, "another_public");
    }

    #[test]
    fn テストモジュール内のpub関数は除外される() {
        let content = r#"
pub fn main_function() {}

#[cfg(test)]
mod tests {
    pub fn test_helper() {}
}
"#;
        let functions = extract_public_functions(content);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].1, "main_function");
    }

    #[test]
    fn async関数も検出できる() {
        let content = r#"
pub async fn async_function() {}
"#;
        let functions = extract_public_functions(content);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].1, "async_function");
    }

    #[test]
    fn tokio_testも検出できる() {
        let content = r#"
#[tokio::test]
async fn test_async() {
}
"#;
        assert!(has_test_module_or_function(content));
    }
}
