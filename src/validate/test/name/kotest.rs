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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn testパターンからテスト名を抽出できる() {
        let content = r#"
test("should create user") {
}
"#;
        let tests = extract_test_names(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "should create user");
    }

    #[test]
    fn contextパターンからテスト名を抽出できる() {
        let content = r#"
context("when user is logged in") {
}
"#;
        let tests = extract_test_names(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "when user is logged in");
    }

    #[test]
    fn 日本語のテスト名は違反として検出されない() {
        let content = r#"
test("ユーザーを作成できる") {
}
"#;
        let violations = validate(content);
        assert!(violations.is_empty());
    }

    #[test]
    fn 英語のテスト名は違反として検出される() {
        let content = r#"
test("should create user") {
}
"#;
        let violations = validate(content);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "should create user");
    }

    #[test]
    fn 複数のDSLパターンを同時に検出できる() {
        let content = r#"
describe("UserService") {
    context("when creating") {
        it("should validate") {
        }
    }
}
"#;
        let tests = extract_test_names(content);
        assert_eq!(tests.len(), 3);
    }

    #[test]
    fn 日本語と英語が混在する場合は英語のみ違反となる() {
        let content = r#"
test("ユーザー作成テスト") {
}
test("create user") {
}
"#;
        let violations = validate(content);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "create user");
    }

    #[test]
    fn shouldパターンを検出できる() {
        let content = r#"
should("return empty list") {
}
"#;
        let tests = extract_test_names(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "return empty list");
    }

    #[test]
    fn givenWhenThenパターンを検出できる() {
        let content = r#"
given("a user") {
    when("logged in") {
        then("can access dashboard") {
        }
    }
}
"#;
        let tests = extract_test_names(content);
        assert_eq!(tests.len(), 3);
    }
}
