/// Filter based on file extension
#[derive(Clone, Default, Debug)]
pub struct ExtFilter {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

impl ExtFilter {
    pub fn matches(&self, filename: &str) -> bool {
        if !self.include.is_empty() && !self.include.iter().any(|e| filename.ends_with(e)) {
            return false;
        }
        if !self.exclude.is_empty() && self.exclude.iter().any(|e| filename.ends_with(e)) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_filter_empty_matches_all() {
        let filter = ExtFilter::default();
        assert!(filter.matches("test.java"));
        assert!(filter.matches("test.txt"));
        assert!(filter.matches("anything"));
    }

    #[test]
    fn test_ext_filter_include_only() {
        let filter = ExtFilter { include: vec![".java".to_string(), ".kt".to_string()], exclude: vec![] };
        assert!(filter.matches("Test.java"));
        assert!(filter.matches("Test.kt"));
        assert!(!filter.matches("Test.txt"));
        assert!(!filter.matches("Test.py"));
    }

    #[test]
    fn test_ext_filter_exclude_only() {
        let filter = ExtFilter { include: vec![], exclude: vec![".test.java".to_string()] };
        assert!(filter.matches("Test.java"));
        assert!(!filter.matches("Test.test.java"));
        assert!(filter.matches("Test.txt"));
    }

    #[test]
    fn test_ext_filter_include_and_exclude() {
        let filter = ExtFilter { include: vec![".java".to_string()], exclude: vec![".test.java".to_string()] };
        assert!(filter.matches("Main.java"));
        assert!(!filter.matches("Main.test.java"));
        assert!(!filter.matches("Main.txt"));
    }
}
