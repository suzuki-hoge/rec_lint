use crate::rule::TextRule;
use crate::validate::Violation;

pub fn validate(content: &str, rule: &TextRule) -> Vec<Violation> {
    let mut violations = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        for keyword in &rule.keywords {
            if let Some(col) = line.find(keyword) {
                violations.push(Violation { line: line_num + 1, col: col + 1, found: line.to_string() });
                break;
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::Matcher;

    fn make_rule(keywords: Vec<&str>) -> TextRule {
        TextRule {
            label: "test".to_string(),
            keywords: keywords.into_iter().map(String::from).collect(),
            message: "test message".to_string(),
            matcher: Matcher::default(),
        }
    }

    // =========================================================================
    // Basic matching tests
    // =========================================================================

    #[test]
    fn test_no_match_returns_empty() {
        let rule = make_rule(vec!["println"]);
        let content = "let x = 42;";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_single_keyword_match() {
        let rule = make_rule(vec!["println"]);
        let content = "println!(\"hello\");";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
        assert_eq!(violations[0].col, 1);
        assert_eq!(violations[0].found, "println!(\"hello\");");
    }

    #[test]
    fn test_keyword_in_middle_of_line() {
        let rule = make_rule(vec!["ng-word"]);
        let content = "// This is a ng-word item";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 14); // "// This is a " = 13 chars, ng-word starts at 14
    }

    #[test]
    fn test_empty_content() {
        let rule = make_rule(vec!["println"]);
        let content = "";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Multiple keywords tests
    // =========================================================================

    #[test]
    fn test_multiple_keywords_first_match_wins() {
        let rule = make_rule(vec!["alpha", "beta"]);
        let content = "beta alpha gamma";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        // "alpha" is first in keywords list, but "beta" appears first in line
        // However, we iterate keywords in order, so "alpha" is checked first
        assert_eq!(violations[0].col, 6); // "beta " = 5 chars, alpha starts at 6
    }

    #[test]
    fn test_second_keyword_matches_when_first_absent() {
        let rule = make_rule(vec!["alpha", "beta"]);
        let content = "beta gamma";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn test_only_first_keyword_per_line() {
        // Even if multiple keywords match, only first match per line is reported (break)
        let rule = make_rule(vec!["a", "b"]);
        let content = "a b c";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1); // 'a' at position 1
    }

    // =========================================================================
    // Multiple lines tests
    // =========================================================================

    #[test]
    fn test_multiple_lines_with_violations() {
        let rule = make_rule(vec!["bad"]);
        let content = "good line\nbad line 1\ngood again\nbad line 2";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].line, 2);
        assert_eq!(violations[0].col, 1);
        assert_eq!(violations[1].line, 4);
        assert_eq!(violations[1].col, 1);
    }

    #[test]
    fn test_line_numbers_are_1_based() {
        let rule = make_rule(vec!["target"]);
        let content = "line1\nline2\ntarget here";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 3);
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_keyword_at_end_of_line() {
        let rule = make_rule(vec!["end"]);
        let content = "this is the end";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 13); // "this is the " = 12 chars
    }

    #[test]
    fn test_case_sensitive_matching() {
        let rule = make_rule(vec!["NGWORD"]);
        let content = "ngword item";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_substring_match() {
        let rule = make_rule(vec!["print"]);
        let content = "println!()";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn test_empty_keywords() {
        let rule = make_rule(vec![]);
        let content = "any content";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_found_contains_full_line() {
        let rule = make_rule(vec!["x"]);
        let content = "let x = 42;";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].found, "let x = 42;");
    }
}
