use super::{filter_non_japanese, TestViolation};

/// Extract Rust test function names and validate for Japanese
pub fn validate(content: &str) -> Vec<TestViolation> {
    let tests = extract_test_functions(content);
    filter_non_japanese(tests)
}

/// Extract test function names from Rust test files
fn extract_test_functions(content: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut tests = Vec::new();
    let mut pending_test_line: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let line_num = i + 1;

        // Check for #[test] or #[tokio::test] or similar test attributes
        if trimmed.starts_with("#[test]")
            || trimmed.starts_with("#[tokio::test]")
            || trimmed.starts_with("#[actix_web::test]")
            || trimmed.starts_with("#[actix_rt::test]")
            || trimmed.starts_with("#[async_std::test]")
        {
            pending_test_line = Some(line_num);
            continue;
        }

        // If we're looking for a function after #[test]
        if pending_test_line.is_some() {
            // Skip other attributes
            if trimmed.starts_with("#[") {
                continue;
            }

            // Try to extract function name
            if let Some(name) = extract_fn_name(trimmed) {
                tests.push((pending_test_line.unwrap(), name));
            }
            pending_test_line = None;
        }
    }

    tests
}

/// Extract function name from a fn declaration line
fn extract_fn_name(line: &str) -> Option<String> {
    // Look for pattern: fn name(
    if !line.contains("fn ") {
        return None;
    }

    let fn_pos = line.find("fn ")?;
    let after_fn = &line[fn_pos + 3..];

    // Handle async fn
    let name_start = after_fn.trim();

    // Extract identifier
    let name: String = name_start.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}
