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

// =============================================================================
// Doc validator config (no_java_doc, no_kotlin_doc, no_rust_doc)
// =============================================================================

/// Visibility level for doc checks
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Public,
    All,
}

/// Config for no_java_doc and no_kotlin_doc validators
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawJavaDocConfig {
    #[serde(rename = "type")]
    pub type_: Option<Visibility>,
    pub constructor: Option<Visibility>,
    pub function: Option<Visibility>,
}

/// Config for no_rust_doc validator
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawRustDocConfig {
    #[serde(rename = "type")]
    pub type_: Option<Visibility>,
    pub function: Option<Visibility>,
    #[serde(rename = "macro")]
    pub macro_: Option<bool>,
}

// =============================================================================
// Comment validator config (no_japanese_comment, no_english_comment)
// =============================================================================

/// Language preset for comment syntax
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CommentLang {
    Java,
    Kotlin,
    Rust,
}

/// Block comment syntax
#[derive(Clone, Debug, Deserialize)]
pub struct RawBlockComment {
    pub start: String,
    pub end: String,
}

/// Custom comment syntax
#[derive(Clone, Debug, Deserialize)]
pub struct RawCustomComment {
    #[serde(default)]
    pub lines: Vec<String>,
    #[serde(default)]
    pub blocks: Vec<RawBlockComment>,
}

/// Config for no_japanese_comment and no_english_comment validators
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawCommentConfig {
    pub lang: Option<CommentLang>,
    pub custom: Option<RawCustomComment>,
}

#[derive(Deserialize)]
pub struct RawConfig {
    pub required: Option<Vec<RawRule>>,
    pub deny: Option<Vec<RawRule>>,
    pub review: Option<Vec<RawReviewItem>>,
}

#[derive(Deserialize, Default)]
pub struct RawRule {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub validator: String,
    pub keywords: Option<Vec<String>>,
    pub exec: Option<String>,
    #[serde(default)]
    pub message: String,
    pub include_exts: Option<Vec<String>>,
    pub exclude_exts: Option<Vec<String>>,
    pub exclude_files: Option<Vec<RawExcludeFilter>>,
    // Doc validator configs
    pub java_doc: Option<RawJavaDocConfig>,
    pub kotlin_doc: Option<RawJavaDocConfig>,
    pub rust_doc: Option<RawRustDocConfig>,
    // Comment validator configs
    pub comment: Option<RawCommentConfig>,
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

    // =========================================================================
    // Doc validator tests
    // =========================================================================

    #[test]
    fn test_parse_no_java_doc_all_options() {
        let yaml = r#"
deny:
  - label: java-doc
    validator: no_java_doc
    message: Missing JavaDoc
    java_doc:
      type: public
      constructor: all
      function: public
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        assert_eq!(rule.validator, "no_java_doc");
        let doc = rule.java_doc.as_ref().unwrap();
        assert_eq!(doc.type_, Some(Visibility::Public));
        assert_eq!(doc.constructor, Some(Visibility::All));
        assert_eq!(doc.function, Some(Visibility::Public));
    }

    #[test]
    fn test_parse_no_java_doc_partial_options() {
        let yaml = r#"
deny:
  - label: java-doc
    validator: no_java_doc
    message: Missing JavaDoc
    java_doc:
      type: all
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        let doc = rule.java_doc.as_ref().unwrap();
        assert_eq!(doc.type_, Some(Visibility::All));
        assert!(doc.constructor.is_none());
        assert!(doc.function.is_none());
    }

    #[test]
    fn test_parse_no_kotlin_doc() {
        let yaml = r#"
deny:
  - label: kotlin-doc
    validator: no_kotlin_doc
    message: Missing KDoc
    kotlin_doc:
      type: public
      function: all
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        assert_eq!(rule.validator, "no_kotlin_doc");
        let doc = rule.kotlin_doc.as_ref().unwrap();
        assert_eq!(doc.type_, Some(Visibility::Public));
        assert_eq!(doc.function, Some(Visibility::All));
    }

    #[test]
    fn test_parse_no_rust_doc() {
        let yaml = r#"
deny:
  - label: rust-doc
    validator: no_rust_doc
    message: Missing RustDoc
    rust_doc:
      type: public
      function: all
      macro: true
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        assert_eq!(rule.validator, "no_rust_doc");
        let doc = rule.rust_doc.as_ref().unwrap();
        assert_eq!(doc.type_, Some(Visibility::Public));
        assert_eq!(doc.function, Some(Visibility::All));
        assert_eq!(doc.macro_, Some(true));
    }

    #[test]
    fn test_parse_no_rust_doc_without_macro() {
        let yaml = r#"
deny:
  - label: rust-doc
    validator: no_rust_doc
    message: Missing RustDoc
    rust_doc:
      type: all
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        let doc = rule.rust_doc.as_ref().unwrap();
        assert_eq!(doc.type_, Some(Visibility::All));
        assert!(doc.function.is_none());
        assert!(doc.macro_.is_none());
    }

    // =========================================================================
    // Comment validator tests
    // =========================================================================

    #[test]
    fn test_parse_no_japanese_comment_with_lang() {
        let yaml = r#"
deny:
  - label: no-jp-comment
    validator: no_japanese_comment
    message: Japanese comment found
    comment:
      lang: java
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        assert_eq!(rule.validator, "no_japanese_comment");
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Java));
        assert!(comment.custom.is_none());
    }

    #[test]
    fn test_parse_no_japanese_comment_with_lang_kotlin() {
        let yaml = r#"
deny:
  - label: no-jp-comment
    validator: no_japanese_comment
    message: Japanese comment found
    comment:
      lang: kotlin
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Kotlin));
    }

    #[test]
    fn test_parse_no_japanese_comment_with_lang_rust() {
        let yaml = r#"
deny:
  - label: no-jp-comment
    validator: no_japanese_comment
    message: Japanese comment found
    comment:
      lang: rust
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Rust));
    }

    #[test]
    fn test_parse_no_japanese_comment_with_custom() {
        let yaml = r#"
deny:
  - label: no-jp-comment
    validator: no_japanese_comment
    message: Japanese comment found
    comment:
      custom:
        lines:
          - "//"
        blocks:
          - start: "/*"
            end: "*/"
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        let comment = rule.comment.as_ref().unwrap();
        assert!(comment.lang.is_none());
        let custom = comment.custom.as_ref().unwrap();
        assert_eq!(custom.lines, vec!["//"]);
        assert_eq!(custom.blocks.len(), 1);
        assert_eq!(custom.blocks[0].start, "/*");
        assert_eq!(custom.blocks[0].end, "*/");
    }

    #[test]
    fn test_parse_no_japanese_comment_custom_python() {
        let yaml = r##"
deny:
  - label: no-jp-comment
    validator: no_japanese_comment
    message: Japanese comment found
    comment:
      custom:
        lines:
          - "#"
        blocks:
          - start: "\"\"\""
            end: "\"\"\""
"##;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        let comment = rule.comment.as_ref().unwrap();
        let custom = comment.custom.as_ref().unwrap();
        assert_eq!(custom.lines, vec!["#"]);
        assert_eq!(custom.blocks[0].start, "\"\"\"");
        assert_eq!(custom.blocks[0].end, "\"\"\"");
    }

    #[test]
    fn test_parse_no_japanese_comment_custom_html() {
        let yaml = r#"
deny:
  - label: no-jp-comment
    validator: no_japanese_comment
    message: Japanese comment found
    comment:
      custom:
        blocks:
          - start: "<!--"
            end: "-->"
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        let comment = rule.comment.as_ref().unwrap();
        let custom = comment.custom.as_ref().unwrap();
        assert!(custom.lines.is_empty());
        assert_eq!(custom.blocks.len(), 1);
        assert_eq!(custom.blocks[0].start, "<!--");
        assert_eq!(custom.blocks[0].end, "-->");
    }

    #[test]
    fn test_parse_no_english_comment() {
        let yaml = r#"
deny:
  - label: no-en-comment
    validator: no_english_comment
    message: English comment found
    comment:
      lang: java
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.deny.unwrap()[0];
        assert_eq!(rule.validator, "no_english_comment");
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Java));
    }
}
