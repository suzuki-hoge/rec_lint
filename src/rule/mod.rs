mod collector;
pub mod parser;
pub mod root_config;

pub use collector::{collect_rules, CollectedRules};
pub use root_config::RootConfig;

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::matcher::Matcher;
use crate::validate::comment::custom::{BlockSyntax, CustomCommentSyntax};
use crate::validate::doc::{KotlinDocConfig, PhpDocConfig, RustDocConfig};
use crate::validate::test::exists::{KotestTestConfig, PhpUnitTestConfig, RustTestConfig, RustUnitTestConfig};
use parser::{CommentLang, RawConfig, RawGuidelineItem, RawRule, TestRequireLevel, TestRequireLevelRust, Visibility};

#[derive(Clone, Debug)]
pub enum Rule {
    Text(TextRule),
    Regex(RegexRule),
    Custom(CustomRule),
    PhpDoc(PhpDocRule),
    KotlinDoc(KotlinDocRule),
    RustDoc(RustDocRule),
    JapaneseComment(CommentRule),
    EnglishComment(CommentRule),
    PhpUnitTest(TestRule),
    KotestTest(TestRule),
    RustTest(TestRule),
    // Test existence rules
    PhpUnitTestExistence(TestExistenceRule<PhpUnitTestConfig>),
    KotestTestExistence(TestExistenceRule<KotestTestConfig>),
    RustTestExistence(TestExistenceRule<RustTestConfig>),
}

impl Rule {
    pub fn label(&self) -> &str {
        match self {
            Rule::Text(r) => &r.label,
            Rule::Regex(r) => &r.label,
            Rule::Custom(r) => &r.label,
            Rule::PhpDoc(r) => &r.label,
            Rule::KotlinDoc(r) => &r.label,
            Rule::RustDoc(r) => &r.label,
            Rule::JapaneseComment(r) => &r.label,
            Rule::EnglishComment(r) => &r.label,
            Rule::PhpUnitTest(r) => &r.label,
            Rule::KotestTest(r) => &r.label,
            Rule::RustTest(r) => &r.label,
            Rule::PhpUnitTestExistence(r) => &r.label,
            Rule::KotestTestExistence(r) => &r.label,
            Rule::RustTestExistence(r) => &r.label,
        }
    }

    pub fn matcher(&self) -> &Matcher {
        match self {
            Rule::Text(r) => &r.matcher,
            Rule::Regex(r) => &r.matcher,
            Rule::Custom(r) => &r.matcher,
            Rule::PhpDoc(r) => &r.matcher,
            Rule::KotlinDoc(r) => &r.matcher,
            Rule::RustDoc(r) => &r.matcher,
            Rule::JapaneseComment(r) => &r.matcher,
            Rule::EnglishComment(r) => &r.matcher,
            Rule::PhpUnitTest(r) => &r.matcher,
            Rule::KotestTest(r) => &r.matcher,
            Rule::RustTest(r) => &r.matcher,
            Rule::PhpUnitTestExistence(r) => &r.matcher,
            Rule::KotestTestExistence(r) => &r.matcher,
            Rule::RustTestExistence(r) => &r.matcher,
        }
    }

    pub fn keywords(&self) -> Option<&[String]> {
        match self {
            Rule::Text(r) => Some(&r.keywords),
            Rule::Regex(r) => Some(&r.keywords),
            Rule::Custom(_) => None,
            Rule::PhpDoc(_) => None,
            Rule::KotlinDoc(_) => None,
            Rule::RustDoc(_) => None,
            Rule::JapaneseComment(_) => None,
            Rule::EnglishComment(_) => None,
            Rule::PhpUnitTest(_) => None,
            Rule::KotestTest(_) => None,
            Rule::RustTest(_) => None,
            Rule::PhpUnitTestExistence(_) => None,
            Rule::KotestTestExistence(_) => None,
            Rule::RustTestExistence(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextRule {
    pub label: String,
    pub keywords: Vec<String>,
    pub message: String,
    pub matcher: Matcher,
}

#[derive(Clone, Debug)]
pub struct RegexRule {
    pub label: String,
    pub patterns: Vec<Regex>,
    pub keywords: Vec<String>,
    pub message: String,
    pub matcher: Matcher,
}

#[derive(Clone, Debug)]
pub struct CustomRule {
    pub label: String,
    pub exec: String,
    pub message: String,
    pub matcher: Matcher,
}

#[derive(Clone, Debug)]
pub struct PhpDocRule {
    pub label: String,
    pub config: PhpDocConfig,
    pub message: String,
    pub matcher: Matcher,
}

#[derive(Clone, Debug)]
pub struct KotlinDocRule {
    pub label: String,
    pub config: KotlinDocConfig,
    pub message: String,
    pub matcher: Matcher,
}

#[derive(Clone, Debug)]
pub struct RustDocRule {
    pub label: String,
    pub config: RustDocConfig,
    pub message: String,
    pub matcher: Matcher,
}

/// Comment source for comment validation
#[derive(Clone, Debug)]
pub enum CommentSource {
    Lang(CommentLang),
    Custom(CustomCommentSyntax),
}

#[derive(Clone, Debug)]
pub struct CommentRule {
    pub label: String,
    pub source: CommentSource,
    pub message: String,
    pub matcher: Matcher,
}

#[derive(Clone, Debug)]
pub struct TestRule {
    pub label: String,
    pub message: String,
    pub matcher: Matcher,
}

#[derive(Clone, Debug)]
pub struct TestExistenceRule<C> {
    pub label: String,
    pub config: C,
    pub message: String,
    pub matcher: Matcher,
}

#[derive(Clone, Debug)]
pub struct GuidelineItem {
    pub message: String,
    #[allow(dead_code)]
    pub matcher: Matcher,
}

#[derive(Debug)]
pub struct Config {
    pub rule: Vec<Rule>,
    pub guideline: Vec<GuidelineItem>,
}

impl TryFrom<RawConfig> for Config {
    type Error = anyhow::Error;

    fn try_from(raw: RawConfig) -> Result<Self> {
        let rule = raw.rule.unwrap_or_default().into_iter().map(convert_rule).collect::<Result<Vec<_>>>()?;

        let guideline = raw.guideline.unwrap_or_default().into_iter().map(convert_guideline).collect::<Vec<_>>();

        Ok(Config { rule, guideline })
    }
}

fn convert_rule(raw: RawRule) -> Result<Rule> {
    let matcher = Matcher::new(raw.match_.clone());

    match raw.type_.as_str() {
        "forbidden_texts" => {
            let keywords = raw
                .keywords
                .ok_or_else(|| anyhow!("Rule '{}': type 'forbidden_texts' requires 'keywords'", raw.label))?;
            if raw.exec.is_some() {
                return Err(anyhow!("Rule '{}': type 'forbidden_texts' must not have 'exec'", raw.label));
            }
            Ok(Rule::Text(TextRule { label: raw.label, keywords, message: raw.message, matcher }))
        }
        "forbidden_patterns" => {
            let keywords = raw
                .keywords
                .ok_or_else(|| anyhow!("Rule '{}': type 'forbidden_patterns' requires 'keywords'", raw.label))?;
            if raw.exec.is_some() {
                return Err(anyhow!("Rule '{}': type 'forbidden_patterns' must not have 'exec'", raw.label));
            }
            let patterns = keywords
                .iter()
                .map(|k| Regex::new(k).map_err(|e| anyhow!("Rule '{}': invalid regex '{}': {}", raw.label, k, e)))
                .collect::<Result<Vec<_>>>()?;
            Ok(Rule::Regex(RegexRule { label: raw.label, patterns, keywords, message: raw.message, matcher }))
        }
        "custom" => {
            let exec = raw.exec.ok_or_else(|| anyhow!("Rule '{}': type 'custom' requires 'exec'", raw.label))?;
            if raw.keywords.is_some() {
                return Err(anyhow!("Rule '{}': type 'custom' must not have 'keywords'", raw.label));
            }
            Ok(Rule::Custom(CustomRule { label: raw.label, exec, message: raw.message, matcher }))
        }
        "require_php_doc" => {
            let raw_config = raw
                .php_doc
                .ok_or_else(|| anyhow!("Rule '{}': type 'require_php_doc' requires 'php_doc' config", raw.label))?;
            if raw_config.class.is_none()
                && raw_config.interface.is_none()
                && raw_config.trait_.is_none()
                && raw_config.enum_.is_none()
                && raw_config.function.is_none()
            {
                return Err(anyhow!(
                    "Rule '{}': 'php_doc' config requires at least one element (class, interface, trait, enum, function)",
                    raw.label
                ));
            }
            let config = PhpDocConfig {
                class: raw_config.class.map(convert_visibility),
                interface: raw_config.interface.map(convert_visibility),
                trait_: raw_config.trait_.map(convert_visibility),
                enum_: raw_config.enum_.map(convert_visibility),
                function: raw_config.function.map(convert_visibility),
            };
            Ok(Rule::PhpDoc(PhpDocRule { label: raw.label, config, message: raw.message, matcher }))
        }
        "require_kotlin_doc" => {
            let raw_config = raw.kotlin_doc.ok_or_else(|| {
                anyhow!("Rule '{}': type 'require_kotlin_doc' requires 'kotlin_doc' config", raw.label)
            })?;
            if raw_config.class.is_none()
                && raw_config.interface.is_none()
                && raw_config.object.is_none()
                && raw_config.enum_class.is_none()
                && raw_config.sealed_class.is_none()
                && raw_config.sealed_interface.is_none()
                && raw_config.data_class.is_none()
                && raw_config.value_class.is_none()
                && raw_config.annotation_class.is_none()
                && raw_config.typealias.is_none()
                && raw_config.function.is_none()
            {
                return Err(anyhow!("Rule '{}': 'kotlin_doc' config requires at least one element", raw.label));
            }
            let config = KotlinDocConfig {
                class: raw_config.class.map(convert_visibility),
                interface: raw_config.interface.map(convert_visibility),
                object: raw_config.object.map(convert_visibility),
                enum_class: raw_config.enum_class.map(convert_visibility),
                sealed_class: raw_config.sealed_class.map(convert_visibility),
                sealed_interface: raw_config.sealed_interface.map(convert_visibility),
                data_class: raw_config.data_class.map(convert_visibility),
                value_class: raw_config.value_class.map(convert_visibility),
                annotation_class: raw_config.annotation_class.map(convert_visibility),
                typealias: raw_config.typealias.map(convert_visibility),
                function: raw_config.function.map(convert_visibility),
            };
            Ok(Rule::KotlinDoc(KotlinDocRule { label: raw.label, config, message: raw.message, matcher }))
        }
        "require_rust_doc" => {
            let raw_config = raw
                .rust_doc
                .ok_or_else(|| anyhow!("Rule '{}': type 'require_rust_doc' requires 'rust_doc' config", raw.label))?;
            if raw_config.struct_.is_none()
                && raw_config.enum_.is_none()
                && raw_config.trait_.is_none()
                && raw_config.type_alias.is_none()
                && raw_config.union.is_none()
                && raw_config.fn_.is_none()
                && raw_config.macro_rules.is_none()
                && raw_config.mod_.is_none()
            {
                return Err(anyhow!("Rule '{}': 'rust_doc' config requires at least one element", raw.label));
            }
            let config = RustDocConfig {
                struct_: raw_config.struct_.map(convert_visibility),
                enum_: raw_config.enum_.map(convert_visibility),
                trait_: raw_config.trait_.map(convert_visibility),
                type_alias: raw_config.type_alias.map(convert_visibility),
                union: raw_config.union.map(convert_visibility),
                fn_: raw_config.fn_.map(convert_visibility),
                macro_rules: raw_config.macro_rules.map(convert_visibility),
                mod_: raw_config.mod_.map(convert_visibility),
            };
            Ok(Rule::RustDoc(RustDocRule { label: raw.label, config, message: raw.message, matcher }))
        }
        "require_english_comment" => {
            let source = convert_comment_source(&raw)?;
            Ok(Rule::JapaneseComment(CommentRule { label: raw.label, source, message: raw.message, matcher }))
        }
        "require_japanese_comment" => {
            let source = convert_comment_source(&raw)?;
            Ok(Rule::EnglishComment(CommentRule { label: raw.label, source, message: raw.message, matcher }))
        }
        "require_japanese_phpunit_test_name" => {
            Ok(Rule::PhpUnitTest(TestRule { label: raw.label, message: raw.message, matcher }))
        }
        "require_japanese_kotest_test_name" => {
            Ok(Rule::KotestTest(TestRule { label: raw.label, message: raw.message, matcher }))
        }
        "require_japanese_rust_test_name" => {
            Ok(Rule::RustTest(TestRule { label: raw.label, message: raw.message, matcher }))
        }
        "require_phpunit_test" => {
            let raw_config = raw.phpunit_test.unwrap_or_default();
            let config = PhpUnitTestConfig {
                test_directory: raw_config.test_directory.unwrap_or_else(|| "tests".to_string()),
                require: raw_config.require.unwrap_or(TestRequireLevel::FileExists),
                suffix: raw_config.suffix.unwrap_or_else(|| "Test".to_string()),
            };
            Ok(Rule::PhpUnitTestExistence(TestExistenceRule {
                label: raw.label,
                config,
                message: raw.message,
                matcher,
            }))
        }
        "require_kotest_test" => {
            let raw_config = raw.kotest_test.unwrap_or_default();
            let config = KotestTestConfig {
                test_directory: raw_config.test_directory.unwrap_or_else(|| "src/test/kotlin".to_string()),
                require: raw_config.require.unwrap_or(TestRequireLevel::FileExists),
                suffix: raw_config.suffix.unwrap_or_else(|| "Test".to_string()),
            };
            Ok(Rule::KotestTestExistence(TestExistenceRule { label: raw.label, config, message: raw.message, matcher }))
        }
        "require_rust_test" => {
            let raw_config = raw.rust_test.unwrap_or_default();
            let unit = raw_config
                .unit
                .map(|u| RustUnitTestConfig { require: u.require.unwrap_or(TestRequireLevelRust::Exists) });
            let config = RustTestConfig { unit };
            Ok(Rule::RustTestExistence(TestExistenceRule { label: raw.label, config, message: raw.message, matcher }))
        }
        other => Err(anyhow!("Rule '{}': unknown type '{}'", raw.label, other)),
    }
}

fn convert_visibility(vis: parser::Visibility) -> Visibility {
    match vis {
        parser::Visibility::Public => Visibility::Public,
        parser::Visibility::All => Visibility::All,
    }
}

fn convert_comment_source(raw: &RawRule) -> Result<CommentSource> {
    let config = raw.comment.as_ref().ok_or_else(|| anyhow!("Rule '{}': comment config is required", raw.label))?;

    // lang and custom are mutually exclusive
    match (&config.lang, &config.custom) {
        (Some(lang), None) => Ok(CommentSource::Lang(lang.clone())),
        (None, Some(custom)) => {
            let blocks =
                custom.blocks.iter().map(|b| BlockSyntax { start: b.start.clone(), end: b.end.clone() }).collect();
            let syntax = CustomCommentSyntax { lines: custom.lines.clone(), blocks };
            Ok(CommentSource::Custom(syntax))
        }
        (Some(_), Some(_)) => Err(anyhow!("Rule '{}': cannot specify both 'lang' and 'custom'", raw.label)),
        (None, None) => Err(anyhow!("Rule '{}': either 'lang' or 'custom' is required", raw.label)),
    }
}

fn convert_guideline(raw: RawGuidelineItem) -> GuidelineItem {
    GuidelineItem { message: raw.message, matcher: Matcher::new(raw.match_.clone()) }
}
