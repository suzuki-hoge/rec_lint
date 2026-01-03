use std::path::Path;

use crate::rule::parser::{ExcludeFilterType, RawExcludeFilter};

/// Parsed exclude filter
#[derive(Clone, Debug, Default)]
pub struct ExcludeFilter {
    pub filters: Vec<RawExcludeFilter>,
}

impl ExcludeFilter {
    pub fn new(filters: Vec<RawExcludeFilter>) -> Self {
        Self { filters }
    }

    /// Returns true if the file should be excluded
    pub fn should_exclude(&self, file_path: &Path) -> bool {
        if self.filters.is_empty() {
            return false;
        }

        let filename = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let path_str = file_path.to_string_lossy();

        for filter in &self.filters {
            let matched = match filter.filter {
                ExcludeFilterType::FileStartsWith => filename.starts_with(&filter.keyword),
                ExcludeFilterType::FileEndsWith => filename.ends_with(&filter.keyword),
                ExcludeFilterType::PathContains => path_str.contains(&filter.keyword),
            };
            if matched {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_empty_filter_excludes_nothing() {
        let filter = ExcludeFilter::default();
        assert!(!filter.should_exclude(Path::new("src/Main.java")));
        assert!(!filter.should_exclude(Path::new("test/Test.java")));
    }

    #[test]
    fn test_file_starts_with() {
        let filter = ExcludeFilter::new(vec![RawExcludeFilter {
            filter: ExcludeFilterType::FileStartsWith,
            keyword: "Test".to_string(),
        }]);
        assert!(filter.should_exclude(Path::new("src/TestMain.java")));
        assert!(filter.should_exclude(Path::new("TestFile.java")));
        assert!(!filter.should_exclude(Path::new("src/Main.java")));
        assert!(!filter.should_exclude(Path::new("src/MyTest.java")));
    }

    #[test]
    fn test_file_ends_with() {
        let filter = ExcludeFilter::new(vec![RawExcludeFilter {
            filter: ExcludeFilterType::FileEndsWith,
            keyword: ".test.java".to_string(),
        }]);
        assert!(filter.should_exclude(Path::new("src/Main.test.java")));
        assert!(filter.should_exclude(Path::new("Sample.test.java")));
        assert!(!filter.should_exclude(Path::new("src/Main.java")));
        assert!(!filter.should_exclude(Path::new("src/Test.java")));
    }

    #[test]
    fn test_path_contains() {
        let filter = ExcludeFilter::new(vec![RawExcludeFilter {
            filter: ExcludeFilterType::PathContains,
            keyword: "/test/".to_string(),
        }]);
        assert!(filter.should_exclude(Path::new("src/test/Main.java")));
        assert!(filter.should_exclude(Path::new("/test/Sample.java")));
        assert!(!filter.should_exclude(Path::new("src/Main.java")));
        assert!(!filter.should_exclude(Path::new("test.java")));
    }

    #[test]
    fn test_multiple_filters_or_logic() {
        let filter = ExcludeFilter::new(vec![
            RawExcludeFilter { filter: ExcludeFilterType::FileStartsWith, keyword: "Test".to_string() },
            RawExcludeFilter { filter: ExcludeFilterType::PathContains, keyword: "/generated/".to_string() },
        ]);
        // Matches first filter
        assert!(filter.should_exclude(Path::new("src/TestMain.java")));
        // Matches second filter
        assert!(filter.should_exclude(Path::new("src/generated/Model.java")));
        // Matches neither
        assert!(!filter.should_exclude(Path::new("src/Main.java")));
    }

    #[test]
    fn test_path_buf_input() {
        let filter = ExcludeFilter::new(vec![RawExcludeFilter {
            filter: ExcludeFilterType::FileEndsWith,
            keyword: ".gen.rs".to_string(),
        }]);
        let path = PathBuf::from("src/model.gen.rs");
        assert!(filter.should_exclude(&path));
    }
}
