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
