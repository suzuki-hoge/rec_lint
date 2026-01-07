use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

/// Match pattern type
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MatchPattern {
    /// Match if filename starts with keyword
    FileStartsWith,
    /// Match if filename ends with keyword
    FileEndsWith,
    /// Match if path contains keyword
    PathContains,
    /// Match if filename does NOT start with keyword
    FileNotStartsWith,
    /// Match if filename does NOT end with keyword
    FileNotEndsWith,
    /// Match if path does NOT contain keyword
    PathNotContains,
}

/// Match condition for keywords
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum MatchCond {
    #[default]
    And,
    Or,
}

/// Single match item entry
#[derive(Clone, Debug, Deserialize)]
pub struct RawMatchItem {
    pub pattern: MatchPattern,
    pub keywords: Vec<String>,
    #[serde(default)]
    pub cond: MatchCond,
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

/// Config for require_php_doc validator
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawPhpDocConfig {
    pub class: Option<Visibility>,
    pub interface: Option<Visibility>,
    #[serde(rename = "trait")]
    pub trait_: Option<Visibility>,
    #[serde(rename = "enum")]
    pub enum_: Option<Visibility>,
    pub function: Option<Visibility>,
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

// =============================================================================
// Test existence validator config (require_phpunit_test, require_kotest_test, require_rust_test)
// =============================================================================

/// Require level for test existence (PHP/Kotlin)
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestRequireLevel {
    /// Test file must exist
    FileExists,
    /// All public methods must be tested
    AllPublic,
}

/// Require level for test existence (Rust)
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestRequireLevelRust {
    /// Test must exist
    Exists,
    /// All pub functions must be tested
    AllPublic,
}

/// Config for require_phpunit_test validator
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawPhpUnitTestConfig {
    pub test_directory: Option<String>,
    pub require: Option<TestRequireLevel>,
    pub suffix: Option<String>,
}

/// Config for require_kotest_test validator
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawKotestTestConfig {
    pub test_directory: Option<String>,
    pub require: Option<TestRequireLevel>,
    pub suffix: Option<String>,
}

/// Config for require_rust_test validator
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawRustTestConfig {
    pub unit: Option<RawRustUnitTestConfig>,
    pub integration: Option<RawRustIntegrationTestConfig>,
    pub suffix: Option<String>,
}

/// Config for Rust unit test
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawRustUnitTestConfig {
    pub require: Option<TestRequireLevelRust>,
}

/// Config for Rust integration test
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RawRustIntegrationTestConfig {
    pub test_directory: Option<String>,
    pub require: Option<TestRequireLevelRust>,
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
    #[serde(default, rename = "match")]
    pub match_: Vec<RawMatchItem>,
    // Doc validator configs
    pub php_doc: Option<RawPhpDocConfig>,
    pub kotlin_doc: Option<RawKotlinDocConfig>,
    pub rust_doc: Option<RawRustDocConfig>,
    // Comment validator configs
    pub comment: Option<RawCommentConfig>,
    // Test existence validator configs
    pub phpunit_test: Option<RawPhpUnitTestConfig>,
    pub kotest_test: Option<RawKotestTestConfig>,
    pub rust_test: Option<RawRustTestConfig>,
}

#[derive(Deserialize)]
pub struct RawGuidelineItem {
    pub message: String,
    #[serde(default, rename = "match")]
    pub match_: Vec<RawMatchItem>,
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
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 空のYAMLをパースすると空の設定が生成される() {
        let yaml = "";
        let config = RawConfig::parse(yaml).unwrap();
        assert!(config.rule.is_none());
        assert!(config.guideline.is_none());
    }

    #[test]
    fn 空のセクションをパースすると空のベクタが生成される() {
        let yaml = r#"
rule: []
guideline: []
"#;
        let config = RawConfig::parse(yaml).unwrap();
        assert_eq!(config.rule.unwrap().len(), 0);
        assert_eq!(config.guideline.unwrap().len(), 0);
    }

    #[test]
    fn テキストルールのYAMLをパースできる() {
        let yaml = r#"
rule:
  - label: test-rule
    type: forbidden_texts
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
        assert_eq!(rule.type_, "forbidden_texts");
        assert_eq!(rule.keywords.as_ref().unwrap().len(), 2);
        assert!(rule.exec.is_none());
        assert_eq!(rule.message, "Test message");
        assert!(rule.match_.is_empty());
    }

    #[test]
    fn 正規表現ルールのYAMLをパースできる() {
        let yaml = r#"
rule:
  - label: regex-rule
    type: forbidden_patterns
    keywords:
      - "pattern.*"
    message: Regex message
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "forbidden_patterns");
        assert!(rule.keywords.is_some());
        assert!(rule.exec.is_none());
    }

    #[test]
    fn カスタムルールのYAMLをパースできる() {
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
    fn マッチ条件付きルールをパースできる() {
        let yaml = r#"
rule:
  - label: match-rule
    type: forbidden_texts
    keywords: [test]
    message: Message
    match:
      - pattern: file_ends_with
        keywords: [.java, .kt]
        cond: or
      - pattern: file_not_ends_with
        keywords: [.test.java]
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.match_.len(), 2);
        assert_eq!(rule.match_[0].pattern, MatchPattern::FileEndsWith);
        assert_eq!(rule.match_[0].keywords, vec![".java", ".kt"]);
        assert_eq!(rule.match_[0].cond, MatchCond::Or);
        assert_eq!(rule.match_[1].pattern, MatchPattern::FileNotEndsWith);
        assert_eq!(rule.match_[1].keywords, vec![".test.java"]);
        assert_eq!(rule.match_[1].cond, MatchCond::And); // default
    }

    #[test]
    fn 全種類のマッチパターンをパースできる() {
        let yaml = r#"
rule:
  - label: all-patterns
    type: forbidden_texts
    keywords: [test]
    message: Message
    match:
      - pattern: file_starts_with
        keywords: [Test]
      - pattern: file_ends_with
        keywords: [.java]
      - pattern: path_contains
        keywords: [/src/]
      - pattern: file_not_starts_with
        keywords: [_]
      - pattern: file_not_ends_with
        keywords: [.bak]
      - pattern: path_not_contains
        keywords: [/generated/]
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.match_.len(), 6);
        assert_eq!(rule.match_[0].pattern, MatchPattern::FileStartsWith);
        assert_eq!(rule.match_[1].pattern, MatchPattern::FileEndsWith);
        assert_eq!(rule.match_[2].pattern, MatchPattern::PathContains);
        assert_eq!(rule.match_[3].pattern, MatchPattern::FileNotStartsWith);
        assert_eq!(rule.match_[4].pattern, MatchPattern::FileNotEndsWith);
        assert_eq!(rule.match_[5].pattern, MatchPattern::PathNotContains);
    }

    #[test]
    fn ガイドライン項目をパースできる() {
        let yaml = r#"
guideline:
  - message: Guideline point 1
  - message: Guideline point 2
    match:
      - pattern: file_ends_with
        keywords: [.java]
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let guidelines = config.guideline.unwrap();
        assert_eq!(guidelines.len(), 2);
        assert_eq!(guidelines[0].message, "Guideline point 1");
        assert!(guidelines[0].match_.is_empty());
        assert_eq!(guidelines[1].message, "Guideline point 2");
        assert_eq!(guidelines[1].match_.len(), 1);
    }

    #[test]
    fn ルールとガイドラインの混合セクションをパースできる() {
        let yaml = r#"
rule:
  - label: rule1
    type: forbidden_texts
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
    // ドキュメント検証設定
    // =========================================================================

    #[test]
    fn PhpDoc設定の全オプションをパースできる() {
        let yaml = r#"
rule:
  - label: php-doc
    type: require_php_doc
    message: Missing PHPDoc
    php_doc:
      class: public
      interface: public
      trait: all
      enum: public
      function: public
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "require_php_doc");
        let doc = rule.php_doc.as_ref().unwrap();
        assert_eq!(doc.class, Some(Visibility::Public));
        assert_eq!(doc.interface, Some(Visibility::Public));
        assert_eq!(doc.trait_, Some(Visibility::All));
        assert_eq!(doc.enum_, Some(Visibility::Public));
        assert_eq!(doc.function, Some(Visibility::Public));
    }

    #[test]
    fn PhpDoc設定の一部オプションのみパースできる() {
        let yaml = r#"
rule:
  - label: php-doc
    type: require_php_doc
    message: Missing PHPDoc
    php_doc:
      class: all
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        let doc = rule.php_doc.as_ref().unwrap();
        assert_eq!(doc.class, Some(Visibility::All));
        assert!(doc.function.is_none());
    }

    #[test]
    fn KotlinDoc設定をパースできる() {
        let yaml = r#"
rule:
  - label: kotlin-doc
    type: require_kotlin_doc
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
        assert_eq!(rule.type_, "require_kotlin_doc");
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
    fn RustDoc設定をパースできる() {
        let yaml = r#"
rule:
  - label: rust-doc
    type: require_rust_doc
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
        assert_eq!(rule.type_, "require_rust_doc");
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
    // コメント検証設定
    // =========================================================================

    #[test]
    fn Java言語指定のコメント設定をパースできる() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: require_english_comment
    message: Japanese comment found
    comment:
      lang: java
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "require_english_comment");
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Java));
        assert!(comment.custom.is_none());
    }

    #[test]
    fn Kotlin言語指定のコメント設定をパースできる() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: require_english_comment
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
    fn Rust言語指定のコメント設定をパースできる() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: require_english_comment
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
    fn カスタムコメント構文をパースできる() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: require_english_comment
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
    fn Python形式のカスタムコメント構文をパースできる() {
        let yaml = r##"
rule:
  - label: no-jp-comment
    type: require_english_comment
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
    fn HTML形式のカスタムコメント構文をパースできる() {
        let yaml = r#"
rule:
  - label: no-jp-comment
    type: require_english_comment
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
    fn 英語コメント禁止ルールをパースできる() {
        let yaml = r#"
rule:
  - label: no-en-comment
    type: require_japanese_comment
    message: English comment found
    comment:
      lang: java
"#;
        let config = RawConfig::parse(yaml).unwrap();
        let rule = &config.rule.unwrap()[0];
        assert_eq!(rule.type_, "require_japanese_comment");
        let comment = rule.comment.as_ref().unwrap();
        assert_eq!(comment.lang, Some(CommentLang::Java));
    }
}
