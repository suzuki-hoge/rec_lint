use super::{SameFileTestConfig, TestExistenceViolation, TestExistenceViolationKind};
use crate::rule::parser::TestRequireLevel;

/// Validate test existence for a Rust source file
pub fn validate(content: &str, config: &SameFileTestConfig) -> Vec<TestExistenceViolation> {
    let mut violations = Vec::new();

    // Skip validation if there are no public functions to test
    let public_functions = extract_public_functions(content);
    if public_functions.is_empty() {
        return violations;
    }

    let has_test_module = has_test_module_or_function(content);

    if !has_test_module {
        violations.push(TestExistenceViolation { kind: TestExistenceViolationKind::MissingUnitTest });
    } else if config.require == TestRequireLevel::AllPublic {
        // Check that all public functions are tested
        let test_content = extract_test_module_content(content);

        for (line, func_name) in public_functions {
            if !test_content.contains(&func_name) {
                violations.push(TestExistenceViolation {
                    kind: TestExistenceViolationKind::UntestedPublicFunction { line, function_name: func_name },
                });
            }
        }
    }

    violations
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
