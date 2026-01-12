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
// Test existence validator config (require_phpunit_test, require_kotest_test, require_rust_unit_test)
// =============================================================================

/// Require level for test existence
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestRequireLevel {
    /// Test must exist
    Exists,
    /// All public methods/functions must be tested
    AllPublic,
}

/// Unified option config for doc/test validators
/// Contains all possible fields from PhpDoc, KotlinDoc, RustDoc, and Test configs
#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct RawOptionConfig {
    // PhpDocConfig fields
    pub class: Option<Visibility>,
    pub interface: Option<Visibility>,
    #[serde(rename = "trait")]
    pub trait_: Option<Visibility>,
    #[serde(rename = "enum")]
    pub enum_: Option<Visibility>,
    pub function: Option<Visibility>,
    // KotlinDocConfig additional fields
    pub object: Option<Visibility>,
    pub enum_class: Option<Visibility>,
    pub sealed_class: Option<Visibility>,
    pub sealed_interface: Option<Visibility>,
    pub data_class: Option<Visibility>,
    pub value_class: Option<Visibility>,
    pub annotation_class: Option<Visibility>,
    pub typealias: Option<Visibility>,
    // RustDocConfig additional fields
    #[serde(rename = "struct")]
    pub struct_: Option<Visibility>,
    pub type_alias: Option<Visibility>,
    pub union: Option<Visibility>,
    #[serde(rename = "fn")]
    pub fn_: Option<Visibility>,
    pub macro_rules: Option<Visibility>,
    #[serde(rename = "mod")]
    pub mod_: Option<Visibility>,
    // Test config fields (PHPUnit/Kotest/Rust)
    pub test_directory: Option<String>,
    pub require: Option<TestRequireLevel>,
    pub test_file_suffix: Option<String>,
}

#[derive(Deserialize)]
pub struct RawConfig {
    pub rule: Option<Vec<RawRuleItem>>,
    pub guideline: Option<Vec<RawGuidelineItem>>,
}

/// Rule item with rule name as key
#[derive(Deserialize, Default)]
pub struct RawRuleItem {
    pub forbidden_texts: Option<RawRuleContent>,
    pub forbidden_patterns: Option<RawRuleContent>,
    pub custom: Option<RawRuleContent>,
    pub require_php_doc: Option<RawRuleContent>,
    pub require_kotlin_doc: Option<RawRuleContent>,
    pub require_rust_doc: Option<RawRuleContent>,
    pub require_english_comment: Option<RawRuleContent>,
    pub require_japanese_comment: Option<RawRuleContent>,
    pub require_japanese_phpunit_test_name: Option<RawRuleContent>,
    pub require_japanese_kotest_test_name: Option<RawRuleContent>,
    pub require_japanese_rust_test_name: Option<RawRuleContent>,
    pub require_phpunit_test: Option<RawRuleContent>,
    pub require_kotest_test: Option<RawRuleContent>,
    pub require_rust_unit_test: Option<RawRuleContent>,
}

/// Rule content (common fields for all rule types)
#[derive(Deserialize, Default, Clone)]
pub struct RawRuleContent {
    #[serde(default)]
    pub label: String,
    pub texts: Option<Vec<String>>,
    pub patterns: Option<Vec<String>>,
    pub exec: Option<String>,
    #[serde(default)]
    pub message: String,
    #[serde(default, rename = "match")]
    pub match_: Vec<RawMatchItem>,
    // Doc/Comment/Test validator configs (unified as "option" or "format")
    pub option: Option<RawOptionConfig>,
    pub format: Option<RawCommentConfig>,
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
