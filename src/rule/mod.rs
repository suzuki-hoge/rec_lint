mod collector;
pub mod parser;

pub use collector::{collect_rules, CollectedRules};

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::filter::{ExcludeFilter, ExtFilter};
use parser::{RawConfig, RawReviewItem, RawRule};

#[derive(Clone, Debug)]
pub enum Rule {
    Text(TextRule),
    Regex(RegexRule),
    Custom(CustomRule),
}

impl Rule {
    pub fn label(&self) -> &str {
        match self {
            Rule::Text(r) => &r.label,
            Rule::Regex(r) => &r.label,
            Rule::Custom(r) => &r.label,
        }
    }

    pub fn ext_filter(&self) -> &ExtFilter {
        match self {
            Rule::Text(r) => &r.ext_filter,
            Rule::Regex(r) => &r.ext_filter,
            Rule::Custom(r) => &r.ext_filter,
        }
    }

    pub fn exclude_filter(&self) -> &ExcludeFilter {
        match self {
            Rule::Text(r) => &r.exclude_filter,
            Rule::Regex(r) => &r.exclude_filter,
            Rule::Custom(r) => &r.exclude_filter,
        }
    }

    pub fn keywords(&self) -> Option<&[String]> {
        match self {
            Rule::Text(r) => Some(&r.keywords),
            Rule::Regex(r) => Some(&r.keywords),
            Rule::Custom(_) => None,
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
pub struct ReviewItem {
    pub message: String,
    #[allow(dead_code)]
    pub ext_filter: ExtFilter,
}

#[derive(Debug)]
pub struct Config {
    pub required: Vec<Rule>,
    pub deny: Vec<Rule>,
    pub review: Vec<ReviewItem>,
}

impl TryFrom<RawConfig> for Config {
    type Error = anyhow::Error;

    fn try_from(raw: RawConfig) -> Result<Self> {
        let required = raw.required.unwrap_or_default().into_iter().map(convert_rule).collect::<Result<Vec<_>>>()?;

        let deny = raw.deny.unwrap_or_default().into_iter().map(convert_rule).collect::<Result<Vec<_>>>()?;

        let review = raw.review.unwrap_or_default().into_iter().map(convert_review).collect::<Vec<_>>();

        Ok(Config { required, deny, review })
    }
}

fn convert_rule(raw: RawRule) -> Result<Rule> {
    let ext_filter =
        ExtFilter { include: raw.include_exts.unwrap_or_default(), exclude: raw.exclude_exts.unwrap_or_default() };

    let exclude_filter = ExcludeFilter::new(raw.exclude_files.unwrap_or_default());

    match raw.validator.as_str() {
        "text" => {
            let keywords =
                raw.keywords.ok_or_else(|| anyhow!("Rule '{}': validator 'text' requires 'keywords'", raw.label))?;
            if raw.exec.is_some() {
                return Err(anyhow!("Rule '{}': validator 'text' must not have 'exec'", raw.label));
            }
            Ok(Rule::Text(TextRule { label: raw.label, keywords, message: raw.message, ext_filter, exclude_filter }))
        }
        "regex" => {
            let keywords =
                raw.keywords.ok_or_else(|| anyhow!("Rule '{}': validator 'regex' requires 'keywords'", raw.label))?;
            if raw.exec.is_some() {
                return Err(anyhow!("Rule '{}': validator 'regex' must not have 'exec'", raw.label));
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
            let exec = raw.exec.ok_or_else(|| anyhow!("Rule '{}': validator 'custom' requires 'exec'", raw.label))?;
            if raw.keywords.is_some() {
                return Err(anyhow!("Rule '{}': validator 'custom' must not have 'keywords'", raw.label));
            }
            Ok(Rule::Custom(CustomRule { label: raw.label, exec, message: raw.message, ext_filter, exclude_filter }))
        }
        other => Err(anyhow!("Rule '{}': unknown validator '{}'", raw.label, other)),
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
        let raw = RawConfig { required: None, deny: None, review: None };
        let config = Config::try_from(raw).unwrap();
        assert!(config.required.is_empty());
        assert!(config.deny.is_empty());
        assert!(config.review.is_empty());
    }

    #[test]
    fn test_convert_empty_vec_sections() {
        let raw = RawConfig { required: Some(vec![]), deny: Some(vec![]), review: Some(vec![]) };
        let config = Config::try_from(raw).unwrap();
        assert!(config.required.is_empty());
        assert!(config.deny.is_empty());
        assert!(config.review.is_empty());
    }

    #[test]
    fn test_convert_text_rule() {
        let raw = RawConfig {
            required: None,
            deny: Some(vec![RawRule {
                label: "test".to_string(),
                validator: "text".to_string(),
                keywords: Some(vec!["kw1".to_string(), "kw2".to_string()]),
                exec: None,
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: None,
                exclude_files: None,
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
            required: None,
            deny: Some(vec![RawRule {
                label: "regex-test".to_string(),
                validator: "regex".to_string(),
                keywords: Some(vec!["pattern.*".to_string()]),
                exec: None,
                message: "msg".to_string(),
                include_exts: Some(vec![".java".to_string()]),
                exclude_exts: None,
                exclude_files: None,
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
            required: Some(vec![RawRule {
                label: "custom-test".to_string(),
                validator: "custom".to_string(),
                keywords: None,
                exec: Some("cmd {file}".to_string()),
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: Some(vec![".txt".to_string()]),
                exclude_files: None,
            }]),
            deny: None,
            review: None,
        };
        let config = Config::try_from(raw).unwrap();
        match &config.required[0] {
            Rule::Custom(r) => {
                assert_eq!(r.label, "custom-test");
                assert_eq!(r.exec, "cmd {file}");
                assert_eq!(r.ext_filter.exclude, vec![".txt"]);
            }
            _ => panic!("Expected Custom rule"),
        }
    }

    #[test]
    fn test_convert_review_items() {
        let raw = RawConfig {
            required: None,
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
            required: None,
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                validator: "text".to_string(),
                keywords: None,
                exec: None,
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: None,
                exclude_files: None,
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'text' requires 'keywords'"));
    }

    #[test]
    fn test_error_text_with_exec() {
        let raw = RawConfig {
            required: None,
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                validator: "text".to_string(),
                keywords: Some(vec!["kw".to_string()]),
                exec: Some("cmd".to_string()),
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: None,
                exclude_files: None,
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'text' must not have 'exec'"));
    }

    #[test]
    fn test_error_regex_without_keywords() {
        let raw = RawConfig {
            required: None,
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                validator: "regex".to_string(),
                keywords: None,
                exec: None,
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: None,
                exclude_files: None,
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'regex' requires 'keywords'"));
    }

    #[test]
    fn test_error_regex_with_exec() {
        let raw = RawConfig {
            required: None,
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                validator: "regex".to_string(),
                keywords: Some(vec![".*".to_string()]),
                exec: Some("cmd".to_string()),
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: None,
                exclude_files: None,
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'regex' must not have 'exec'"));
    }

    #[test]
    fn test_error_regex_invalid_pattern() {
        let raw = RawConfig {
            required: None,
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                validator: "regex".to_string(),
                keywords: Some(vec!["[invalid".to_string()]),
                exec: None,
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: None,
                exclude_files: None,
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("invalid regex"));
    }

    #[test]
    fn test_error_custom_without_exec() {
        let raw = RawConfig {
            required: None,
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                validator: "custom".to_string(),
                keywords: None,
                exec: None,
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: None,
                exclude_files: None,
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'custom' requires 'exec'"));
    }

    #[test]
    fn test_error_custom_with_keywords() {
        let raw = RawConfig {
            required: None,
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                validator: "custom".to_string(),
                keywords: Some(vec!["kw".to_string()]),
                exec: Some("cmd".to_string()),
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: None,
                exclude_files: None,
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'custom' must not have 'keywords'"));
    }

    #[test]
    fn test_error_unknown_validator() {
        let raw = RawConfig {
            required: None,
            deny: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                validator: "unknown".to_string(),
                keywords: None,
                exec: None,
                message: "msg".to_string(),
                include_exts: None,
                exclude_exts: None,
                exclude_files: None,
            }]),
            review: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("unknown validator 'unknown'"));
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
