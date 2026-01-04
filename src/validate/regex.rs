use crate::rule::RegexRule;
use crate::validate::Violation;

pub fn validate(content: &str, rule: &RegexRule) -> Vec<Violation> {
    let mut violations = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        for pattern in &rule.patterns {
            if let Some(m) = pattern.find(line) {
                violations.push(Violation { line: line_num + 1, col: m.start() + 1, found: line.to_string() });
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
    use regex::Regex;

    fn make_rule(patterns: Vec<&str>) -> RegexRule {
        let compiled: Vec<Regex> = patterns.iter().map(|p| Regex::new(p).unwrap()).collect();
        RegexRule {
            label: "test".to_string(),
            patterns: compiled,
            keywords: patterns.into_iter().map(String::from).collect(),
            message: "test message".to_string(),
            matcher: Matcher::default(),
        }
    }

    // =========================================================================
    // Basic matching tests
    // =========================================================================

    #[test]
    fn test_no_match_returns_empty() {
        let rule = make_rule(vec![r"println"]);
        let content = "let x = 42;";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_literal_pattern_match() {
        let rule = make_rule(vec![r"println"]);
        let content = "println!(\"hello\");";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn test_empty_content() {
        let rule = make_rule(vec![r".*"]);
        let content = "";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // Regex-specific tests
    // =========================================================================

    #[test]
    fn test_wildcard_pattern() {
        let rule = make_rule(vec![r"ng-word.*"]);
        let content = "// ng-word: fix this later";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 4); // "// " = 3 chars
    }

    #[test]
    fn test_character_class() {
        let rule = make_rule(vec![r"[0-9]+"]);
        let content = "let x = 42;";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 9); // "let x = " = 8 chars
    }

    #[test]
    fn test_word_boundary() {
        let rule = make_rule(vec![r"\bvar\b"]);
        let content = "var x = 1;";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn test_word_boundary_no_match_in_middle() {
        let rule = make_rule(vec![r"\bvar\b"]);
        let content = "variable x = 1;";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_start_anchor() {
        let rule = make_rule(vec![r"^import"]);
        let content = "import java.util.*;";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn test_start_anchor_no_match_in_middle() {
        let rule = make_rule(vec![r"^import"]);
        let content = "// import java.util.*;";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_optional_pattern() {
        let rule = make_rule(vec![r"colou?r"]);
        let content1 = "color is red";
        let content2 = "colour is blue";
        assert_eq!(validate(content1, &rule).len(), 1);
        assert_eq!(validate(content2, &rule).len(), 1);
    }

    #[test]
    fn test_alternation() {
        let rule = make_rule(vec![r"foo|bar"]);
        let content = "baz bar qux";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 5); // "baz " = 4 chars
    }

    // =========================================================================
    // Multiple patterns tests
    // =========================================================================

    #[test]
    fn test_multiple_patterns_first_match_wins() {
        let rule = make_rule(vec![r"alpha", r"beta"]);
        let content = "beta alpha";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        // "alpha" is first pattern, checked first, finds match at col 6
        assert_eq!(violations[0].col, 6);
    }

    #[test]
    fn test_second_pattern_matches_when_first_absent() {
        let rule = make_rule(vec![r"alpha", r"beta"]);
        let content = "beta gamma";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    // =========================================================================
    // Multiple lines tests
    // =========================================================================

    #[test]
    fn test_multiple_lines_with_violations() {
        let rule = make_rule(vec![r"error"]);
        let content = "ok line\nerror line 1\nok again\nerror line 2";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].line, 2);
        assert_eq!(violations[1].line, 4);
    }

    #[test]
    fn test_line_numbers_are_1_based() {
        let rule = make_rule(vec![r"target"]);
        let content = "line1\nline2\ntarget here";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].line, 3);
    }

    // =========================================================================
    // Column position tests (match.start() based)
    // =========================================================================

    #[test]
    fn test_col_at_regex_match_start() {
        let rule = make_rule(vec![r"[A-Z]+"]);
        let content = "abc XYZ def";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].col, 5); // "abc " = 4 chars, match starts at 5
    }

    #[test]
    fn test_col_with_greedy_match() {
        let rule = make_rule(vec![r"a+"]);
        let content = "bbaaa";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].col, 3); // "bb" = 2 chars
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_empty_patterns() {
        let rule = make_rule(vec![]);
        let content = "any content";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_dot_matches_any_char() {
        let rule = make_rule(vec![r"a.c"]);
        let content = "abc axc a c";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1); // first "abc" matches
    }

    #[test]
    fn test_found_contains_full_line() {
        let rule = make_rule(vec![r"\d+"]);
        let content = "value = 123;";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].found, "value = 123;");
    }

    #[test]
    fn test_case_sensitive_by_default() {
        let rule = make_rule(vec![r"NGWORD"]);
        let content = "ngword item";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_case_insensitive_flag() {
        let rule = make_rule(vec![r"(?i)NGWORD"]);
        let content = "ngword item";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
    }
}
