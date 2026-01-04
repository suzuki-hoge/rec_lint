use super::{filter_non_japanese, TestViolation};

/// Extract JUnit test method names and validate for Japanese
pub fn validate(content: &str) -> Vec<TestViolation> {
    let tests = extract_test_methods(content);
    filter_non_japanese(tests)
}

/// Extract test method names from JUnit test files (Java/Kotlin)
fn extract_test_methods(content: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut tests = Vec::new();
    let mut pending_test_line: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let line_num = i + 1;

        // Check for @Test annotation
        if trimmed.starts_with("@Test")
            || trimmed == "@Test"
            || trimmed.contains("@Test ")
            || trimmed.contains("@Test(")
        {
            pending_test_line = Some(line_num);
            continue;
        }

        // If we're looking for a method after @Test
        if pending_test_line.is_some() {
            // Skip other annotations
            if trimmed.starts_with('@') {
                continue;
            }

            // Try to extract method name
            if let Some(name) = extract_method_name(trimmed) {
                tests.push((pending_test_line.unwrap(), name));
            }
            pending_test_line = None;
        }
    }

    tests
}

/// Extract method name from a method declaration line
fn extract_method_name(line: &str) -> Option<String> {
    // Look for pattern: ... methodName(
    let paren_pos = line.find('(')?;
    let before_paren = &line[..paren_pos];

    // Get the last word before parenthesis (method name)
    let words: Vec<&str> = before_paren.split_whitespace().collect();
    let method_name = words.last()?;

    // Handle generic methods: <T> void methodName or fun <T> methodName
    let name = if method_name.ends_with('>') {
        // Generic type, skip to previous word
        words.get(words.len().saturating_sub(2))?
    } else {
        method_name
    };

    // Clean up any remaining characters
    let clean_name: String = name.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();

    if clean_name.is_empty() {
        None
    } else {
        Some(clean_name)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn Testアノテーションがある関数名を抽出できる() {
        let content = r#"
@Test
fun testMethod() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "testMethod");
    }

    #[test]
    fn 日本語のテスト名は違反として検出されない() {
        let content = r#"
@Test
fun ユーザーを作成できる() {
}
"#;
        let violations = validate(content);
        assert!(violations.is_empty());
    }

    #[test]
    fn 英語のテスト名は違反として検出される() {
        let content = r#"
@Test
fun createUser() {
}
"#;
        let violations = validate(content);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "createUser");
    }

    #[test]
    fn Java形式のテストメソッドを抽出できる() {
        let content = r#"
@Test
public void testCreateUser() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "testCreateUser");
    }

    #[test]
    fn 複数のアノテーションがある場合も正しく抽出できる() {
        let content = r#"
@Test
@DisplayName("User creation test")
fun testMethod() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "testMethod");
    }

    #[test]
    fn Testアノテーションがない関数は抽出されない() {
        let content = r#"
fun regularMethod() {
}
"#;
        let tests = extract_test_methods(content);
        assert!(tests.is_empty());
    }

    #[test]
    fn 複数のテストメソッドを抽出できる() {
        let content = r#"
@Test
fun test1() {}

@Test
fun test2() {}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 2);
    }

    #[test]
    fn Java形式の日本語テスト名は違反として検出されない() {
        let content = r#"
@Test
void ユーザーを作成できる() {
}
"#;
        let violations = validate(content);
        assert!(violations.is_empty());
    }

    #[test]
    fn Java形式の英語テスト名は違反として検出される() {
        let content = r#"
@Test
void shouldCreateUser() {
}
"#;
        let violations = validate(content);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "shouldCreateUser");
    }

    #[test]
    fn パッケージプライベートなJavaテストメソッドを抽出できる() {
        let content = r#"
@Test
void testMethod() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "testMethod");
    }
}
