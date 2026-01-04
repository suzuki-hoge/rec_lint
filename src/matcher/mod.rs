use std::path::Path;

use crate::rule::parser::{MatchCond, MatchPattern, RawMatchItem};

/// Parsed matcher for file path matching
#[derive(Clone, Debug, Default)]
pub struct Matcher {
    pub items: Vec<RawMatchItem>,
}

impl Matcher {
    pub fn new(items: Vec<RawMatchItem>) -> Self {
        Self { items }
    }

    /// Returns true if the file matches all conditions (AND logic between items)
    pub fn matches(&self, file_path: &Path) -> bool {
        if self.items.is_empty() {
            return true;
        }

        let filename = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let path_str = file_path.to_string_lossy();

        // All items must match (AND logic)
        for item in &self.items {
            if !self.item_matches(item, filename, &path_str) {
                return false;
            }
        }
        true
    }

    fn item_matches(&self, item: &RawMatchItem, filename: &str, path_str: &str) -> bool {
        let check_keyword = |keyword: &str| -> bool {
            match item.pattern {
                MatchPattern::FileStartsWith => filename.starts_with(keyword),
                MatchPattern::FileEndsWith => filename.ends_with(keyword),
                MatchPattern::PathContains => path_str.contains(keyword),
                MatchPattern::FileNotStartsWith => !filename.starts_with(keyword),
                MatchPattern::FileNotEndsWith => !filename.ends_with(keyword),
                MatchPattern::PathNotContains => !path_str.contains(keyword),
            }
        };

        match item.cond {
            MatchCond::And => item.keywords.iter().all(|k| check_keyword(k)),
            MatchCond::Or => item.keywords.iter().any(|k| check_keyword(k)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(pattern: MatchPattern, keywords: Vec<&str>, cond: MatchCond) -> RawMatchItem {
        RawMatchItem { pattern, keywords: keywords.into_iter().map(String::from).collect(), cond }
    }

    // =========================================================================
    // Empty matcher tests
    // =========================================================================

    #[test]
    fn test_empty_matcher_matches_all() {
        let matcher = Matcher::default();
        assert!(matcher.matches(Path::new("src/Main.java")));
        assert!(matcher.matches(Path::new("test/Test.java")));
        assert!(matcher.matches(Path::new("anything.txt")));
    }

    // =========================================================================
    // Positive pattern tests (file_starts_with, file_ends_with, path_contains)
    // =========================================================================

    #[test]
    fn test_file_starts_with() {
        let matcher = Matcher::new(vec![make_item(MatchPattern::FileStartsWith, vec!["Test"], MatchCond::And)]);
        assert!(matcher.matches(Path::new("src/TestMain.java")));
        assert!(matcher.matches(Path::new("TestFile.java")));
        assert!(!matcher.matches(Path::new("src/Main.java")));
        assert!(!matcher.matches(Path::new("src/MyTest.java")));
    }

    #[test]
    fn test_file_ends_with() {
        let matcher = Matcher::new(vec![make_item(MatchPattern::FileEndsWith, vec![".java"], MatchCond::And)]);
        assert!(matcher.matches(Path::new("src/Main.java")));
        assert!(matcher.matches(Path::new("Test.java")));
        assert!(!matcher.matches(Path::new("src/Main.kt")));
        assert!(!matcher.matches(Path::new("src/Main.txt")));
    }

    #[test]
    fn test_file_ends_with_or_condition() {
        let matcher = Matcher::new(vec![make_item(MatchPattern::FileEndsWith, vec![".java", ".kt"], MatchCond::Or)]);
        assert!(matcher.matches(Path::new("src/Main.java")));
        assert!(matcher.matches(Path::new("src/Main.kt")));
        assert!(!matcher.matches(Path::new("src/Main.txt")));
    }

    #[test]
    fn test_path_contains() {
        let matcher = Matcher::new(vec![make_item(MatchPattern::PathContains, vec!["/src/"], MatchCond::And)]);
        assert!(matcher.matches(Path::new("project/src/Main.java")));
        assert!(matcher.matches(Path::new("/src/Test.java")));
        assert!(!matcher.matches(Path::new("test/Main.java")));
    }

    // =========================================================================
    // Negative pattern tests (file_not_*, path_not_*)
    // =========================================================================

    #[test]
    fn test_file_not_starts_with() {
        let matcher = Matcher::new(vec![make_item(MatchPattern::FileNotStartsWith, vec!["Test"], MatchCond::And)]);
        assert!(matcher.matches(Path::new("src/Main.java")));
        assert!(matcher.matches(Path::new("src/MyTest.java")));
        assert!(!matcher.matches(Path::new("src/TestMain.java")));
        assert!(!matcher.matches(Path::new("TestFile.java")));
    }

    #[test]
    fn test_file_not_ends_with() {
        let matcher = Matcher::new(vec![make_item(MatchPattern::FileNotEndsWith, vec![".test.java"], MatchCond::And)]);
        assert!(matcher.matches(Path::new("src/Main.java")));
        assert!(matcher.matches(Path::new("src/Test.java")));
        assert!(!matcher.matches(Path::new("src/Main.test.java")));
    }

    #[test]
    fn test_path_not_contains() {
        let matcher = Matcher::new(vec![make_item(MatchPattern::PathNotContains, vec!["/test/"], MatchCond::And)]);
        assert!(matcher.matches(Path::new("src/Main.java")));
        assert!(matcher.matches(Path::new("test.java"))); // filename, not path
        assert!(!matcher.matches(Path::new("src/test/Main.java")));
    }

    #[test]
    fn test_path_not_contains_or_condition() {
        // NOT contains /test/ OR NOT contains /generated/
        // This means: file is excluded only if it contains BOTH /test/ AND /generated/
        let matcher =
            Matcher::new(vec![make_item(MatchPattern::PathNotContains, vec!["/test/", "/generated/"], MatchCond::Or)]);
        // Has neither -> matches (not test OR not generated = true OR true = true)
        assert!(matcher.matches(Path::new("src/Main.java")));
        // Has only /test/ -> matches (not test OR not generated = false OR true = true)
        assert!(matcher.matches(Path::new("src/test/Main.java")));
        // Has only /generated/ -> matches
        assert!(matcher.matches(Path::new("src/generated/Main.java")));
        // Has both -> still matches because OR means any keyword not matching passes
        // Actually, for PathNotContains with OR: passes if NOT contains ANY keyword
        // So /test/generated/ does NOT contain "/test/" = false, does NOT contain "/generated/" = false
        // false OR false = false
        assert!(!matcher.matches(Path::new("src/test/generated/Main.java")));
    }

    #[test]
    fn test_path_not_contains_and_condition() {
        // NOT contains /test/ AND NOT contains /generated/
        // File passes only if it contains neither
        let matcher =
            Matcher::new(vec![make_item(MatchPattern::PathNotContains, vec!["/test/", "/generated/"], MatchCond::And)]);
        // Has neither -> matches
        assert!(matcher.matches(Path::new("src/Main.java")));
        // Has /test/ -> doesn't match
        assert!(!matcher.matches(Path::new("src/test/Main.java")));
        // Has /generated/ -> doesn't match
        assert!(!matcher.matches(Path::new("src/generated/Main.java")));
    }

    // =========================================================================
    // Multiple items (AND logic between items)
    // =========================================================================

    #[test]
    fn test_multiple_items_and_logic() {
        let matcher = Matcher::new(vec![
            make_item(MatchPattern::FileEndsWith, vec![".java"], MatchCond::And),
            make_item(MatchPattern::PathNotContains, vec!["/test/"], MatchCond::And),
        ]);
        // .java AND not in /test/
        assert!(matcher.matches(Path::new("src/Main.java")));
        assert!(!matcher.matches(Path::new("src/test/Main.java"))); // in /test/
        assert!(!matcher.matches(Path::new("src/Main.kt"))); // not .java
    }

    #[test]
    fn test_complex_matcher() {
        // Match: (.java OR .kt) AND NOT in /test/ AND NOT in /generated/
        let matcher = Matcher::new(vec![
            make_item(MatchPattern::FileEndsWith, vec![".java", ".kt"], MatchCond::Or),
            make_item(MatchPattern::PathNotContains, vec!["/test/", "/generated/"], MatchCond::And),
        ]);
        assert!(matcher.matches(Path::new("src/Main.java")));
        assert!(matcher.matches(Path::new("src/Main.kt")));
        assert!(!matcher.matches(Path::new("src/Main.txt"))); // wrong extension
        assert!(!matcher.matches(Path::new("src/test/Main.java"))); // in /test/
        assert!(!matcher.matches(Path::new("src/generated/Main.kt"))); // in /generated/
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_empty_keywords_and_condition() {
        // Empty keywords with AND -> all() returns true for empty iterator
        let matcher = Matcher::new(vec![make_item(MatchPattern::FileEndsWith, vec![], MatchCond::And)]);
        assert!(matcher.matches(Path::new("anything.txt")));
    }

    #[test]
    fn test_empty_keywords_or_condition() {
        // Empty keywords with OR -> any() returns false for empty iterator
        let matcher = Matcher::new(vec![make_item(MatchPattern::FileEndsWith, vec![], MatchCond::Or)]);
        assert!(!matcher.matches(Path::new("anything.txt")));
    }
}
