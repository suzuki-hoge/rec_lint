mod collector;
pub mod parser;

pub use collector::{collect_rules, CollectedRules};

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::filter::{ExcludeFilter, ExtFilter};
use crate::validate::comment::custom::{BlockSyntax, CustomCommentSyntax};
use crate::validate::doc::{JavaDocConfig, KotlinDocConfig, RustDocConfig};
use parser::{CommentLang, RawConfig, RawReviewItem, RawRule, Visibility};

#[derive(Clone, Debug)]
pub enum Rule {
    Text(TextRule),
    Regex(RegexRule),
    Custom(CustomRule),
    JavaDoc(JavaDocRule),
    KotlinDoc(KotlinDocRule),
    RustDoc(RustDocRule),
    JapaneseComment(CommentRule),
    EnglishComment(CommentRule),
}

impl Rule {
    pub fn label(&self) -> &str {
        match self {
            Rule::Text(r) => &r.label,
            Rule::Regex(r) => &r.label,
            Rule::Custom(r) => &r.label,
            Rule::JavaDoc(r) => &r.label,
            Rule::KotlinDoc(r) => &r.label,
            Rule::RustDoc(r) => &r.label,
            Rule::JapaneseComment(r) => &r.label,
            Rule::EnglishComment(r) => &r.label,
        }
    }

    pub fn ext_filter(&self) -> &ExtFilter {
        match self {
            Rule::Text(r) => &r.ext_filter,
            Rule::Regex(r) => &r.ext_filter,
            Rule::Custom(r) => &r.ext_filter,
            Rule::JavaDoc(r) => &r.ext_filter,
            Rule::KotlinDoc(r) => &r.ext_filter,
            Rule::RustDoc(r) => &r.ext_filter,
            Rule::JapaneseComment(r) => &r.ext_filter,
            Rule::EnglishComment(r) => &r.ext_filter,
        }
    }

    pub fn exclude_filter(&self) -> &ExcludeFilter {
        match self {
            Rule::Text(r) => &r.exclude_filter,
            Rule::Regex(r) => &r.exclude_filter,
            Rule::Custom(r) => &r.exclude_filter,
            Rule::JavaDoc(r) => &r.exclude_filter,
            Rule::KotlinDoc(r) => &r.exclude_filter,
            Rule::RustDoc(r) => &r.exclude_filter,
            Rule::JapaneseComment(r) => &r.exclude_filter,
            Rule::EnglishComment(r) => &r.exclude_filter,
        }
    }

    pub fn keywords(&self) -> Option<&[String]> {
        match self {
            Rule::Text(r) => Some(&r.keywords),
            Rule::Regex(r) => Some(&r.keywords),
            Rule::Custom(_) => None,
            Rule::JavaDoc(_) => None,
            Rule::KotlinDoc(_) => None,
            Rule::RustDoc(_) => None,
            Rule::JapaneseComment(_) => None,
            Rule::EnglishComment(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextRule {
    pub label: String,
    pub keywords: Vec<String>,
    pub message: String,
    pub ext_filter: ExtFilter,
    pub exclude_filter: ExcludeFilter,
}

#[derive(Clone, Debug)]
pub struct RegexRule {
    pub label: String,
    pub patterns: Vec<Regex>,
    pub keywords: Vec<String>,
    pub message: String,
    pub ext_filter: ExtFilter,
    pub exclude_filter: ExcludeFilter,
}

#[derive(Clone, Debug)]
pub struct CustomRule {
    pub label: String,
    pub exec: String,
    pub message: String,
    pub ext_filter: ExtFilter,
    pub exclude_filter: ExcludeFilter,
}

#[derive(Clone, Debug)]
pub struct JavaDocRule {
    pub label: String,
    pub config: JavaDocConfig,
    pub message: String,
    pub ext_filter: ExtFilter,
    pub exclude_filter: ExcludeFilter,
}

#[derive(Clone, Debug)]
pub struct KotlinDocRule {
    pub label: String,
    pub config: KotlinDocConfig,
    pub message: String,
    pub ext_filter: ExtFilter,
    pub exclude_filter: ExcludeFilter,
}

#[derive(Clone, Debug)]
pub struct RustDocRule {
    pub label: String,
    pub config: RustDocConfig,
    pub message: String,
    pub ext_filter: ExtFilter,
    pub exclude_filter: ExcludeFilter,
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
    pub ext_filter: ExtFilter,
    pub exclude_filter: ExcludeFilter,
}

#[derive(Clone, Debug)]
pub struct ReviewItem {
    pub message: String,
    #[allow(dead_code)]
    pub ext_filter: ExtFilter,
}

#[derive(Debug)]
pub struct Config {
    pub deny: Vec<Rule>,
    pub review: Vec<ReviewItem>,
}

impl TryFrom<RawConfig> for Config {
    type Error = anyhow::Error;

    fn try_from(raw: RawConfig) -> Result<Self> {
        let deny = raw.deny.unwrap_or_default().into_iter().map(convert_rule).collect::<Result<Vec<_>>>()?;

        let review = raw.review.unwrap_or_default().into_iter().map(convert_review).collect::<Vec<_>>();

        Ok(Config { deny, review })
    }
}

fn convert_rule(raw: RawRule) -> Result<Rule> {
    let ext_filter = ExtFilter {
        include: raw.include_exts.clone().unwrap_or_default(),
        exclude: raw.exclude_exts.clone().unwrap_or_default(),
    };

    let exclude_filter = ExcludeFilter::new(raw.exclude_files.clone().unwrap_or_default());

    match raw.type_.as_str() {
        "text" => {
            let keywords =
                raw.keywords.ok_or_else(|| anyhow!("Rule '{}': type 'text' requires 'keywords'", raw.label))?;
            if raw.exec.is_some() {
                return Err(anyhow!("Rule '{}': type 'text' must not have 'exec'", raw.label));
            }
            Ok(Rule::Text(TextRule { label: raw.label, keywords, message: raw.message, ext_filter, exclude_filter }))
        }
        "regex" => {
            let keywords =
                raw.keywords.ok_or_else(|| anyhow!("Rule '{}': type 'regex' requires 'keywords'", raw.label))?;
            if raw.exec.is_some() {
                return Err(anyhow!("Rule '{}': type 'regex' must not have 'exec'", raw.label));
            }
            let patterns = keywords
                .iter()
                .map(|k| Regex::new(k).map_err(|e| anyhow!("Rule '{}': invalid regex '{}': {}", raw.label, k, e)))
                .collect::<Result<Vec<_>>>()?;
            Ok(Rule::Regex(RegexRule {
                label: raw.label,
                patterns,
                keywords,
                message: raw.message,
                ext_filter,
                exclude_filter,
            }))
        }
        "custom" => {
            let exec = raw.exec.ok_or_else(|| anyhow!("Rule '{}': type 'custom' requires 'exec'", raw.label))?;
            if raw.keywords.is_some() {
                return Err(anyhow!("Rule '{}': type 'custom' must not have 'keywords'", raw.label));
            }
            Ok(Rule::Custom(CustomRule { label: raw.label, exec, message: raw.message, ext_filter, exclude_filter }))
        }
        "no_java_doc" => {
            let raw_config = raw.java_doc.unwrap_or_default();
            let config = JavaDocConfig {
                type_visibility: raw_config.type_.map(convert_visibility),
                function_visibility: raw_config.function.map(convert_visibility),
            };
            Ok(Rule::JavaDoc(JavaDocRule {
                label: raw.label,
                config,
                message: raw.message,
                ext_filter,
                exclude_filter,
            }))
        }
        "no_kotlin_doc" => {
            let raw_config = raw.kotlin_doc.unwrap_or_default();
            let config = KotlinDocConfig {
                type_visibility: raw_config.type_.map(convert_visibility),
                function_visibility: raw_config.function.map(convert_visibility),
            };
            Ok(Rule::KotlinDoc(KotlinDocRule {
                label: raw.label,
                config,
                message: raw.message,
                ext_filter,
                exclude_filter,
            }))
        }
        "no_rust_doc" => {
            let raw_config = raw.rust_doc.unwrap_or_default();
            let config = RustDocConfig {
                type_visibility: raw_config.type_.map(convert_visibility),
                function_visibility: raw_config.function.map(convert_visibility),
            };
            Ok(Rule::RustDoc(RustDocRule {
                label: raw.label,
                config,
                message: raw.message,
                ext_filter,
                exclude_filter,
            }))
        }
        "no_japanese_comment" => {
            let source = convert_comment_source(&raw)?;
            Ok(Rule::JapaneseComment(CommentRule {
                label: raw.label,
                source,
                message: raw.message,
                ext_filter,
                exclude_filter,
            }))
        }
        "no_english_comment" => {
            let source = convert_comment_source(&raw)?;
            Ok(Rule::EnglishComment(CommentRule {
                label: raw.label,
                source,
                message: raw.message,
                ext_filter,
                exclude_filter,
            }))
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

fn convert_review(raw: RawReviewItem) -> ReviewItem {
    ReviewItem {
        message: raw.message,
        ext_filter: ExtFilter {
            include: raw.include_exts.unwrap_or_default(),
            exclude: raw.exclude_exts.unwrap_or_default(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Config TryFrom tests - success cases
    // =========================================================================

    #[test]
    fn test_convert_empty_config() {
        let raw = RawConfig { deny: None, review: None };
        let config = Config::try_from(raw).unwrap();
        assert!(config.deny.is_empty());
        assert!(config.review.is_empty());
    }

    #[test]
    fn test_convert_empty_vec_sections() {
        let raw = RawConfig { deny: Some(vec![]), review: Some(vec![]) };
        let config = Config::try_from(raw).unwrap();
        assert!(config.deny.is_empty());
        assert!(config.review.is_empty());
    }

    #[test]
    fn test_convert_text_rule() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "test".to_string(),
                type_: "text".to_string(),
                keywords: Some(vec!["kw1".to_string(), "kw2".to_string()]),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            review: None,
        };
        let config = Config::try_from(raw).unwrap();
        assert_eq!(config.deny.len(), 1);
        match &config.deny[0] {
            Rule::Text(r) => {
                assert_eq!(r.label, "test");
                assert_eq!(r.keywords, vec!["kw1", "kw2"]);
                assert_eq!(r.message, "msg");
                assert!(r.ext_filter.include.is_empty());
                assert!(r.ext_filter.exclude.is_empty());
                assert!(r.exclude_filter.filters.is_empty());
            }
            _ => panic!("Expected Text rule"),
        }
    }

    #[test]
    fn test_convert_regex_rule() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "regex-test".to_string(),
                type_: "regex".to_string(),
                keywords: Some(vec!["pattern.*".to_string()]),
                message: "msg".to_string(),
                include_exts: Some(vec![".java".to_string()]),
                ..Default::default()
            }]),
            review: None,
        };
        let config = Config::try_from(raw).unwrap();
        match &config.deny[0] {
            Rule::Regex(r) => {
                assert_eq!(r.label, "regex-test");
                assert_eq!(r.patterns.len(), 1);
                assert_eq!(r.keywords, vec!["pattern.*"]);
                assert_eq!(r.ext_filter.include, vec![".java"]);
            }
            _ => panic!("Expected Regex rule"),
        }
    }

    #[test]
    fn test_convert_custom_rule() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "custom-test".to_string(),
                type_: "custom".to_string(),
                exec: Some("cmd {path}".to_string()),
                message: "msg".to_string(),
                exclude_exts: Some(vec![".txt".to_string()]),
                ..Default::default()
            }]),
            review: None,
        };
        let config = Config::try_from(raw).unwrap();
        match &config.deny[0] {
            Rule::Custom(r) => {
                assert_eq!(r.label, "custom-test");
                assert_eq!(r.exec, "cmd {path}");
                assert_eq!(r.ext_filter.exclude, vec![".txt"]);
            }
            _ => panic!("Expected Custom rule"),
        }
    }

    #[test]
    fn test_convert_review_items() {
        let raw = RawConfig {
            deny: None,
            review: Some(vec![
                RawReviewItem { message: "review1".to_string(), include_exts: None, exclude_exts: None },
                RawReviewItem {
                    message: "review2".to_string(),
                    include_exts: Some(vec![".java".to_string()]),
                    exclude_exts: Some(vec![".test.java".to_string()]),
                },
            ]),
        };
        let config = Config::try_from(raw).unwrap();
        assert_eq!(config.review.len(), 2);
        assert_eq!(config.review[0].message, "review1");
        assert!(config.review[0].ext_filter.include.is_empty());
        assert_eq!(config.review[1].message, "review2");
        assert_eq!(config.review[1].ext_filter.include, vec![".java"]);
    }

    // =========================================================================
    // Config TryFrom tests - error cases
    // =========================================================================

    #[test]
    fn test_error_text_without_keywords() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "text".to_string(),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'text' requires 'keywords'"));
    }

    #[test]
    fn test_error_text_with_exec() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "text".to_string(),
                keywords: Some(vec!["kw".to_string()]),
                exec: Some("cmd".to_string()),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'text' must not have 'exec'"));
    }

    #[test]
    fn test_error_regex_without_keywords() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "regex".to_string(),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'regex' requires 'keywords'"));
    }

    #[test]
    fn test_error_regex_with_exec() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "regex".to_string(),
                keywords: Some(vec![".*".to_string()]),
                exec: Some("cmd".to_string()),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'regex' must not have 'exec'"));
    }

    #[test]
    fn test_error_regex_invalid_pattern() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "regex".to_string(),
                keywords: Some(vec!["[invalid".to_string()]),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("invalid regex"));
    }

    #[test]
    fn test_error_custom_without_exec() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "custom".to_string(),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'custom' requires 'exec'"));
    }

    #[test]
    fn test_error_custom_with_keywords() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "custom".to_string(),
                keywords: Some(vec!["kw".to_string()]),
                exec: Some("cmd".to_string()),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'custom' must not have 'keywords'"));
    }

    #[test]
    fn test_error_unknown_type() {
        let raw = RawConfig {
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "unknown".to_string(),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("unknown type 'unknown'"));
    }

    // =========================================================================
    // Rule accessor tests
    // =========================================================================

    #[test]
    fn test_rule_label() {
        let text_rule = Rule::Text(TextRule {
            label: "text-label".to_string(),
            keywords: vec![],
            message: "".to_string(),
            ext_filter: ExtFilter::default(),
            exclude_filter: ExcludeFilter::default(),
        });
        assert_eq!(text_rule.label(), "text-label");

        let custom_rule = Rule::Custom(CustomRule {
            label: "custom-label".to_string(),
            exec: "".to_string(),
            message: "".to_string(),
            ext_filter: ExtFilter::default(),
            exclude_filter: ExcludeFilter::default(),
        });
        assert_eq!(custom_rule.label(), "custom-label");
    }

    #[test]
    fn test_rule_keywords() {
        let text_rule = Rule::Text(TextRule {
            label: "".to_string(),
            keywords: vec!["a".to_string(), "b".to_string()],
            message: "".to_string(),
            ext_filter: ExtFilter::default(),
            exclude_filter: ExcludeFilter::default(),
        });
        assert_eq!(text_rule.keywords(), Some(&["a".to_string(), "b".to_string()][..]));

        let custom_rule = Rule::Custom(CustomRule {
            label: "".to_string(),
            exec: "".to_string(),
            message: "".to_string(),
            ext_filter: ExtFilter::default(),
            exclude_filter: ExcludeFilter::default(),
        });
        assert!(custom_rule.keywords().is_none());
    }
}
