use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

/// Filter type for exclude_files
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExcludeFilterType {
    /// Check if filename starts with keyword
    FileStartsWith,
    /// Check if filename ends with keyword
    FileEndsWith,
    /// Check if path contains keyword
    PathContains,
}

/// Single exclude filter entry
#[derive(Clone, Debug, Deserialize)]
pub struct RawExcludeFilter {
    pub filter: ExcludeFilterType,
    pub keyword: String,
}

#[derive(Deserialize)]
pub struct RawConfig {
    pub required: Option<Vec<RawRule>>,
    pub deny: Option<Vec<RawRule>>,
    pub review: Option<Vec<RawReviewItem>>,
}

#[derive(Deserialize)]
pub struct RawRule {
    pub label: String,
    pub validator: String,
    pub keywords: Option<Vec<String>>,
    pub exec: Option<String>,
    pub message: String,
    pub include_exts: Option<Vec<String>>,
    pub exclude_exts: Option<Vec<String>>,
    pub exclude_files: Option<Vec<RawExcludeFilter>>,
}

#[derive(Deserialize)]
pub struct RawReviewItem {
    pub message: String,
    pub include_exts: Option<Vec<String>>,
    pub exclude_exts: Option<Vec<String>>,
}

impl RawConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let content =
            fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let config: RawConfig =
            serde_yaml::from_str(&content).with_context(|| format!("Failed to parse YAML: {}", path.display()))?;
        Ok(config)
    }

    pub fn parse(content: &str) -> Result<Self> {
        let config: RawConfig = serde_yaml::from_str(content).with_context(|| "Failed to parse YAML")?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_yaml() {
        let yaml = "";
        let config = RawConfig::parse(yaml).unwrap();
        assert!(config.required.is_none());
        assert!(config.deny.is_none());
        assert!(config.review.is_none());
    }

    #[test]
    fn test_parse_empty_sections() {
        let yaml = r#"
required: []
deny: []
review: []
"#;
        let config = RawConfig::parse(yaml).unwrap();
        assert_eq!(config.required.unwrap().len(), 0);
        assert_eq!(config.deny.unwrap().len(), 0);
        assert_eq!(config.review.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_text_rule() {
        let yaml = r#"
deny:
  - label: test-rule
    validator: text
    keywords:
      - keyword1
      - keyword2
    message: Test message
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rules = config.deny.unwrap();
        assert_eq!(rules.len(), 1);
        let rule = &rules[0];
        assert_eq!(rule.label, "test-rule");
        assert_eq!(rule.validator, "text");
        assert_eq!(rule.keywords.as_ref().unwrap().len(), 2);
        assert!(rule.exec.is_none());
        assert_eq!(rule.message, "Test message");
        assert!(rule.include_exts.is_none());
        assert!(rule.exclude_exts.is_none());
        assert!(rule.exclude_files.is_none());
    }

    #[test]
    fn test_parse_regex_rule() {
        let yaml = r#"
deny:
  - label: regex-rule
    validator: regex
    keywords:
      - "pattern.*"
    message: Regex message
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        assert_eq!(rule.validator, "regex");
        assert!(rule.keywords.is_some());
        assert!(rule.exec.is_none());
    }

    #[test]
    fn test_parse_custom_rule() {
        let yaml = r#"
required:
  - label: custom-rule
    validator: custom
    exec: "command {file}"
    message: Custom message
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.required.unwrap()[0];
        assert_eq!(rule.validator, "custom");
        assert!(rule.keywords.is_none());
        assert_eq!(rule.exec.as_ref().unwrap(), "command {file}");
    }

    #[test]
    fn test_parse_rule_with_exts() {
        let yaml = r#"
deny:
  - label: ext-rule
    validator: text
    keywords: [test]
    message: Message
    include_exts: [.java, .kt]
    exclude_exts: [.test.java]
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        assert_eq!(rule.include_exts.as_ref().unwrap(), &vec![".java", ".kt"]);
        assert_eq!(rule.exclude_exts.as_ref().unwrap(), &vec![".test.java"]);
    }

    #[test]
    fn test_parse_rule_with_exclude_files() {
        let yaml = r#"
deny:
  - label: exclude-test
    validator: text
    keywords: [test]
    message: Message
    exclude_files:
      - filter: file_starts_with
        keyword: Test
      - filter: path_contains
        keyword: /generated/
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        let exclude_files = rule.exclude_files.as_ref().unwrap();
        assert_eq!(exclude_files.len(), 2);
        assert_eq!(exclude_files[0].keyword, "Test");
        assert_eq!(exclude_files[1].keyword, "/generated/");
    }

    #[test]
    fn test_parse_review_item() {
        let yaml = r#"
review:
  - message: Review point 1
  - message: Review point 2
    include_exts: [.java]
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let reviews = config.review.unwrap();
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].message, "Review point 1");
        assert!(reviews[0].include_exts.is_none());
        assert_eq!(reviews[1].message, "Review point 2");
        assert!(reviews[1].include_exts.is_some());
    }

    #[test]
    fn test_parse_mixed_sections() {
        let yaml = r#"
required:
  - label: req1
    validator: custom
    exec: cmd
    message: msg
deny:
  - label: deny1
    validator: text
    keywords: [kw]
    message: msg
review:
  - message: rev1
"#;
        let config = RawConfig::parse(yaml).unwrap();
        assert_eq!(config.required.unwrap().len(), 1);
        assert_eq!(config.deny.unwrap().len(), 1);
        assert_eq!(config.review.unwrap().len(), 1);
    }
}
