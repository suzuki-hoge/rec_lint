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

/// Config for no_java_doc validator
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawJavaDocConfig {
    pub class: Option<Visibility>,
    pub interface: Option<Visibility>,
    #[serde(rename = "enum")]
    pub enum_: Option<Visibility>,
    pub record: Option<Visibility>,
    pub annotation: Option<Visibility>,
    pub method: Option<Visibility>,
}

/// Config for no_kotlin_doc validator
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawKotlinDocConfig {
    pub class: Option<Visibility>,
    pub interface: Option<Visibility>,
    pub object: Option<Visibility>,
    pub enum_class: Option<Visibility>,
    pub sealed_class: Option<Visibility>,
    pub sealed_interface: Option<Visibility>,
    pub data_class: Option<Visibility>,
    pub value_class: Option<Visibility>,
    pub annotation_class: Option<Visibility>,
    pub typealias: Option<Visibility>,
    pub function: Option<Visibility>,
}

/// Config for no_rust_doc validator
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawRustDocConfig {
    #[serde(rename = "struct")]
    pub struct_: Option<Visibility>,
    #[serde(rename = "enum")]
    pub enum_: Option<Visibility>,
    #[serde(rename = "trait")]
    pub trait_: Option<Visibility>,
    pub type_alias: Option<Visibility>,
    pub union: Option<Visibility>,
    #[serde(rename = "fn")]
    pub fn_: Option<Visibility>,
    pub macro_rules: Option<Visibility>,
    #[serde(rename = "mod")]
    pub mod_: Option<Visibility>,
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
    pub rule: Option<Vec<RawRule>>,
    pub guideline: Option<Vec<RawGuidelineItem>>,
}

#[derive(Deserialize, Default)]
pub struct RawRule {
    #[serde(default)]
    pub label: String,
    #[serde(default, rename = "type")]
    pub type_: String,
    pub keywords: Option<Vec<String>>,
    pub exec: Option<String>,
    #[serde(default)]
    pub message: String,
    pub include_exts: Option<Vec<String>>,
    pub exclude_exts: Option<Vec<String>>,
    pub exclude_files: Option<Vec<RawExcludeFilter>>,
    // Doc validator configs
    pub java_doc: Option<RawJavaDocConfig>,
    pub kotlin_doc: Option<RawKotlinDocConfig>,
    pub rust_doc: Option<RawRustDocConfig>,
    // Comment validator configs
    pub comment: Option<RawCommentConfig>,
}

#[derive(Deserialize)]
pub struct RawGuidelineItem {
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
        assert!(config.rule.is_none());
        assert!(config.guideline.is_none());
    }

    #[test]
    fn test_parse_empty_sections() {
        let yaml = r#"
rule: []
guideline: []
"#;
        let config = RawConfig::parse(yaml).unwrap();
        assert_eq!(config.rule.unwrap().len(), 0);
        assert_eq!(config.guideline.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_text_rule() {
        let yaml = r#"
rule:
  - label: test-rule
    type: text
    keywords:
      - keyword1
      - keyword2
    message: Test message
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rules = config.rule.unwrap();
        assert_eq!(rules.len(), 1);
        let rule = &rules[0];
        assert_eq!(rule.label, "test-rule");
        assert_eq!(rule.type_, "text");
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
rule:
  - label: regex-rule
    type: regex
    keywords:
      - "pattern.*"
    message: Regex message
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "regex");
        assert!(rule.keywords.is_some());
        assert!(rule.exec.is_none());
    }

    #[test]
    fn test_parse_custom_rule() {
        let yaml = r#"
rule:
  - label: custom-rule
    type: custom
    exec: "command {path}"
    message: Custom message
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "custom");
        assert!(rule.keywords.is_none());
        assert_eq!(rule.exec.as_ref().unwrap(), "command {path}");
    }

    #[test]
    fn test_parse_rule_with_exts() {
        let yaml = r#"
rule:
  - label: ext-rule
    type: text
    keywords: [test]
    message: Message
    include_exts: [.java, .kt]
    exclude_exts: [.test.java]
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.include_exts.as_ref().unwrap(), &vec![".java", ".kt"]);
        assert_eq!(rule.exclude_exts.as_ref().unwrap(), &vec![".test.java"]);
    }

    #[test]
    fn test_parse_rule_with_exclude_files() {
        let yaml = r#"
rule:
  - label: exclude-test
    type: text
    keywords: [test]
    message: Message
    exclude_files:
      - filter: file_starts_with
        keyword: Test
      - filter: path_contains
        keyword: /generated/
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        let exclude_files = rule.exclude_files.as_ref().unwrap();
        assert_eq!(exclude_files.len(), 2);
        assert_eq!(exclude_files[0].keyword, "Test");
        assert_eq!(exclude_files[1].keyword, "/generated/");
    }

    #[test]
    fn test_parse_guideline_item() {
        let yaml = r#"
guideline:
  - message: Guideline point 1
  - message: Guideline point 2
    include_exts: [.java]
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let guidelines = config.guideline.unwrap();
        assert_eq!(guidelines.len(), 2);
        assert_eq!(guidelines[0].message, "Guideline point 1");
        assert!(guidelines[0].include_exts.is_none());
        assert_eq!(guidelines[1].message, "Guideline point 2");
        assert!(guidelines[1].include_exts.is_some());
    }

    #[test]
    fn test_parse_mixed_sections() {
        let yaml = r#"
rule:
  - label: rule1
    type: text
    keywords: [kw]
    message: msg
guideline:
  - message: guideline1
"#;
        let config = RawConfig::parse(yaml).unwrap();
        assert_eq!(config.rule.unwrap().len(), 1);
        assert_eq!(config.guideline.unwrap().len(), 1);
    }

    // =========================================================================
    // Doc validator tests
    // =========================================================================

    #[test]
    fn test_parse_no_java_doc_all_options() {
        let yaml = r#"
rule:
  - label: java-doc
    type: no_java_doc
    message: Missing JavaDoc
    java_doc:
      class: public
      interface: public
      enum: all
      record: public
      annotation: public
      method: public
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "no_java_doc");
        let doc = rule.java_doc.as_ref().unwrap();
        assert_eq!(doc.class, Some(Visibility::Public));
        assert_eq!(doc.interface, Some(Visibility::Public));
        assert_eq!(doc.enum_, Some(Visibility::All));
        assert_eq!(doc.record, Some(Visibility::Public));
        assert_eq!(doc.annotation, Some(Visibility::Public));
        assert_eq!(doc.method, Some(Visibility::Public));
    }

    #[test]
    fn test_parse_no_java_doc_partial_options() {
        let yaml = r#"
rule:
  - label: java-doc
    type: no_java_doc
    message: Missing JavaDoc
    java_doc:
      class: all
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        let doc = rule.java_doc.as_ref().unwrap();
        assert_eq!(doc.class, Some(Visibility::All));
        assert!(doc.method.is_none());
    }

    #[test]
    fn test_parse_no_kotlin_doc() {
        let yaml = r#"
rule:
  - label: kotlin-doc
    type: no_kotlin_doc
    message: Missing KDoc
    kotlin_doc:
      class: public
      interface: public
      object: all
      enum_class: public
      sealed_class: public
      sealed_interface: public
      data_class: public
      value_class: public
      annotation_class: public
      typealias: all
      function: all
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "no_kotlin_doc");
        let doc = rule.kotlin_doc.as_ref().unwrap();
        assert_eq!(doc.class, Some(Visibility::Public));
        assert_eq!(doc.interface, Some(Visibility::Public));
        assert_eq!(doc.object, Some(Visibility::All));
        assert_eq!(doc.enum_class, Some(Visibility::Public));
        assert_eq!(doc.sealed_class, Some(Visibility::Public));
        assert_eq!(doc.sealed_interface, Some(Visibility::Public));
        assert_eq!(doc.data_class, Some(Visibility::Public));
        assert_eq!(doc.value_class, Some(Visibility::Public));
        assert_eq!(doc.annotation_class, Some(Visibility::Public));
        assert_eq!(doc.typealias, Some(Visibility::All));
        assert_eq!(doc.function, Some(Visibility::All));
    }

    #[test]
    fn test_parse_no_rust_doc() {
        let yaml = r#"
rule:
  - label: rust-doc
    type: no_rust_doc
    message: Missing RustDoc
    rust_doc:
      struct: public
      enum: public
      trait: public
      type_alias: public
      union: public
      fn: all
      macro_rules: public
      mod: all
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "no_rust_doc");
        let doc = rule.rust_doc.as_ref().unwrap();
        assert_eq!(doc.struct_, Some(Visibility::Public));
        assert_eq!(doc.enum_, Some(Visibility::Public));
        assert_eq!(doc.trait_, Some(Visibility::Public));
        assert_eq!(doc.type_alias, Some(Visibility::Public));
        assert_eq!(doc.union, Some(Visibility::Public));
        assert_eq!(doc.fn_, Some(Visibility::All));
        assert_eq!(doc.macro_rules, Some(Visibility::Public));
        assert_eq!(doc.mod_, Some(Visibility::All));
    }

    // =========================================================================
    // Comment validator tests
    // =========================================================================

    #[test]
    fn test_parse_no_japanese_comment_with_lang() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: no_japanese_comment
    message: Japanese comment found
    comment:
      lang: java
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "no_japanese_comment");
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Java));
        assert!(comment.custom.is_none());
    }

    #[test]
    fn test_parse_no_japanese_comment_with_lang_kotlin() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: no_japanese_comment
    message: Japanese comment found
    comment:
      lang: kotlin
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Kotlin));
    }

    #[test]
    fn test_parse_no_japanese_comment_with_lang_rust() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: no_japanese_comment
    message: Japanese comment found
    comment:
      lang: rust
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Rust));
    }

    #[test]
    fn test_parse_no_japanese_comment_with_custom() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: no_japanese_comment
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
        let rule = &config.rule.unwrap()[0];
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
rule:
  - label: no-jp-comment
    type: no_japanese_comment
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
        let rule = &config.rule.unwrap()[0];
        let comment = rule.comment.as_ref().unwrap();
        let custom = comment.custom.as_ref().unwrap();
        assert_eq!(custom.lines, vec!["#"]);
        assert_eq!(custom.blocks[0].start, "\"\"\"");
        assert_eq!(custom.blocks[0].end, "\"\"\"");
    }

    #[test]
    fn test_parse_no_japanese_comment_custom_html() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: no_japanese_comment
    message: Japanese comment found
    comment:
      custom:
        blocks:
          - start: "<!--"
            end: "-->"
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
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
rule:
  - label: no-en-comment
    type: no_english_comment
    message: English comment found
    comment:
      lang: java
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "no_english_comment");
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Java));
    }
}
