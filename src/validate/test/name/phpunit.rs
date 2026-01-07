use super::{filter_non_japanese, TestViolation};

/// Extract PHPUnit test method names and validate for Japanese
pub fn validate(content: &str) -> Vec<TestViolation> {
    let tests = extract_test_methods(content);
    filter_non_japanese(tests)
}

/// Extract test method names from PHPUnit test files
/// Detects tests by:
/// 1. Method name starts with "test"
/// 2. PHPDoc @test annotation
/// 3. PHP 8 #[Test] attribute
fn extract_test_methods(content: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut tests = Vec::new();
    let mut pending_test_line: Option<usize> = None;
    let mut in_phpdoc = false;
    let mut phpdoc_has_test = false;
    let mut phpdoc_start_line: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let line_num = i + 1;

        // Track PHPDoc blocks for @test annotation
        if trimmed.starts_with("/**") {
            in_phpdoc = true;
            phpdoc_has_test = false;
            phpdoc_start_line = Some(line_num);
            // Check single-line PHPDoc
            if trimmed.contains("@test") && trimmed.ends_with("*/") {
                phpdoc_has_test = true;
                in_phpdoc = false;
            }
            continue;
        }

        if in_phpdoc {
            if trimmed.contains("@test") {
                phpdoc_has_test = true;
            }
            if trimmed.contains("*/") {
                in_phpdoc = false;
            }
            continue;
        }

        // Check for #[Test] attribute (PHP 8+)
        if trimmed.starts_with("#[Test]") || trimmed == "#[Test]" || trimmed.contains("#[Test]") {
            pending_test_line = Some(line_num);
            continue;
        }

        // Skip other attributes
        if trimmed.starts_with("#[") {
            continue;
        }

        // Check for function/method declaration
        if let Some(func_pos) = trimmed.find("function ") {
            let after_function = &trimmed[func_pos + 9..];
            if let Some(name) = extract_method_name(after_function) {
                // Check if it's a test by:
                // 1. Name starts with "test"
                // 2. Has #[Test] attribute
                // 3. Has @test in PHPDoc
                let is_test_by_name = name.starts_with("test");
                let is_test_by_attribute = pending_test_line.is_some();
                let is_test_by_phpdoc = phpdoc_has_test;

                if is_test_by_name || is_test_by_attribute || is_test_by_phpdoc {
                    let test_line = pending_test_line.or(phpdoc_start_line).unwrap_or(line_num);
                    tests.push((test_line, name));
                }
            }
        }

        // Reset pending states after processing a line that's not an attribute
        if !trimmed.starts_with("#[") {
            pending_test_line = None;
            phpdoc_has_test = false;
            phpdoc_start_line = None;
        }
    }

    tests
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn testプレフィックスのメソッドを抽出できる() {
        let content = r#"
public function testSomething() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "testSomething");
    }

    #[test]
    fn 日本語のテスト名は違反として検出されない() {
        let content = r#"
public function testユーザーを作成できる() {
}
"#;
        let violations = validate(content);
        assert!(violations.is_empty());
    }

    #[test]
    fn 英語のテスト名は違反として検出される() {
        let content = r#"
public function testCreateUser() {
}
"#;
        let violations = validate(content);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "testCreateUser");
    }

    #[test]
    fn PHPDocのtestアノテーションでテストを検出できる() {
        let content = r#"
/**
 * @test
 */
public function shouldCreateUser() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "shouldCreateUser");
    }

    #[test]
    fn PHP8のTestアトリビュートでテストを検出できる() {
        let content = r#"
#[Test]
public function shouldCreateUser() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "shouldCreateUser");
    }

    #[test]
    fn testプレフィックスがないメソッドは抽出されない() {
        let content = r#"
public function createUser() {
}
"#;
        let tests = extract_test_methods(content);
        assert!(tests.is_empty());
    }

    #[test]
    fn 複数のテストメソッドを抽出できる() {
        let content = r#"
public function testOne() {}

public function testTwo() {}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 2);
    }

    #[test]
    fn 日本語の名前をtestアノテーションで定義した場合は違反にならない() {
        let content = r#"
/**
 * @test
 */
public function ユーザーを作成できる() {
}
"#;
        let violations = validate(content);
        assert!(violations.is_empty());
    }

    #[test]
    fn 英語の名前をtestアノテーションで定義した場合は違反になる() {
        let content = r#"
/**
 * @test
 */
public function shouldCreateUser() {
}
"#;
        let violations = validate(content);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].name, "shouldCreateUser");
    }

    #[test]
    fn プライベートメソッドでもtestプレフィックスがあれば検出する() {
        let content = r#"
private function testHelper() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
    }

    #[test]
    fn 単一行PHPDocのtestアノテーションを検出できる() {
        let content = r#"
/** @test */
public function shouldWork() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "shouldWork");
    }

    #[test]
    fn 複数のアトリビュートがある場合も検出できる() {
        let content = r#"
#[Test]
#[DataProvider('userProvider')]
public function shouldCreateUser() {
}
"#;
        let tests = extract_test_methods(content);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].1, "shouldCreateUser");
    }
}
