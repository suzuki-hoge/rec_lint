use super::{filter_non_japanese, TestViolation};

/// Extract Kotest test names and validate for Japanese
pub fn validate(content: &str) -> Vec<TestViolation> {
    let tests = extract_test_names(content);
    filter_non_japanese(tests)
}

/// Extract test names from Kotest DSL patterns
fn extract_test_names(content: &str) -> Vec<(usize, String)> {
    let mut tests = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1;
        let trimmed = line.trim();

        // Check for various Kotest DSL patterns
        for pattern in ["test(\"", "context(\"", "describe(\"", "it(\"", "should(\"", "given(\"", "when(\"", "then(\""]
        {
            if let Some(name) = extract_string_arg(trimmed, pattern) {
                tests.push((line_num, name));
            }
        }
    }

    tests
}

/// Extract string argument from DSL pattern like: test("name") or test("name") {
fn extract_string_arg(line: &str, pattern: &str) -> Option<String> {
    let start = line.find(pattern)?;
    let after_pattern = &line[start + pattern.len()..];

    // Find closing quote
    let end = after_pattern.find('"')?;
    let name = &after_pattern[..end];

    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}
