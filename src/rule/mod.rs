mod collector;
pub mod parser;

pub use collector::{collect_rules, CollectedRules};

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::matcher::Matcher;
use crate::validate::comment::custom::{BlockSyntax, CustomCommentSyntax};
use crate::validate::doc::{JavaDocConfig, KotlinDocConfig, RustDocConfig};
use parser::{CommentLang, RawConfig, RawGuidelineItem, RawRule, Visibility};

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

    pub fn matcher(&self) -> &Matcher {
        match self {
            Rule::Text(r) => &r.matcher,
            Rule::Regex(r) => &r.matcher,
            Rule::Custom(r) => &r.matcher,
            Rule::JavaDoc(r) => &r.matcher,
            Rule::KotlinDoc(r) => &r.matcher,
            Rule::RustDoc(r) => &r.matcher,
            Rule::JapaneseComment(r) => &r.matcher,
            Rule::EnglishComment(r) => &r.matcher,
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
pub struct JavaDocRule {
    pub label: String,
    pub config: JavaDocConfig,
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
        "require_java_doc" => {
            let raw_config = raw
                .java_doc
                .ok_or_else(|| anyhow!("Rule '{}': type 'require_java_doc' requires 'java_doc' config", raw.label))?;
            if raw_config.class.is_none()
                && raw_config.interface.is_none()
                && raw_config.enum_.is_none()
                && raw_config.record.is_none()
                && raw_config.annotation.is_none()
                && raw_config.method.is_none()
            {
                return Err(anyhow!(
                    "Rule '{}': 'java_doc' config requires at least one element (class, interface, enum, record, annotation, method)",
                    raw.label
                ));
            }
            let config = JavaDocConfig {
                class: raw_config.class.map(convert_visibility),
                interface: raw_config.interface.map(convert_visibility),
                enum_: raw_config.enum_.map(convert_visibility),
                record: raw_config.record.map(convert_visibility),
                annotation: raw_config.annotation.map(convert_visibility),
                method: raw_config.method.map(convert_visibility),
            };
            Ok(Rule::JavaDoc(JavaDocRule { label: raw.label, config, message: raw.message, matcher }))
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    // =========================================================================
    // 設定変換の正常系
    // =========================================================================

    #[test]
    fn 空の設定を変換すると空のConfigが生成される() {
        let raw = RawConfig { rule: None, guideline: None };
        let config = Config::try_from(raw).unwrap();
        assert!(config.rule.is_empty());
        assert!(config.guideline.is_empty());
    }

    #[test]
    fn 空のベクタセクションを変換すると空のConfigが生成される() {
        let raw = RawConfig { rule: Some(vec![]), guideline: Some(vec![]) };
        let config = Config::try_from(raw).unwrap();
        assert!(config.rule.is_empty());
        assert!(config.guideline.is_empty());
    }

    #[test]
    fn テキストルールが正しく変換される() {
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "test".to_string(),
                type_: "forbidden_texts".to_string(),
                keywords: Some(vec!["kw1".to_string(), "kw2".to_string()]),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            guideline: None,
        };
        let config = Config::try_from(raw).unwrap();
        assert_eq!(config.rule.len(), 1);
        match &config.rule[0] {
            Rule::Text(r) => {
                assert_eq!(r.label, "test");
                assert_eq!(r.keywords, vec!["kw1", "kw2"]);
                assert_eq!(r.message, "msg");
                assert!(r.matcher.items.is_empty());
            }
            _ => panic!("Expected Text rule"),
        }
    }

    #[test]
    fn 正規表現ルールがマッチ条件と共に変換される() {
        use crate::rule::parser::{MatchCond, MatchPattern, RawMatchItem};
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "regex-test".to_string(),
                type_: "forbidden_patterns".to_string(),
                keywords: Some(vec!["pattern.*".to_string()]),
                message: "msg".to_string(),
                match_: vec![RawMatchItem {
                    pattern: MatchPattern::FileEndsWith,
                    keywords: vec![".java".to_string()],
                    cond: MatchCond::Or,
                }],
                ..Default::default()
            }]),
            guideline: None,
        };
        let config = Config::try_from(raw).unwrap();
        match &config.rule[0] {
            Rule::Regex(r) => {
                assert_eq!(r.label, "regex-test");
                assert_eq!(r.patterns.len(), 1);
                assert_eq!(r.keywords, vec!["pattern.*"]);
                assert_eq!(r.matcher.items.len(), 1);
            }
            _ => panic!("Expected Regex rule"),
        }
    }

    #[test]
    fn カスタムルールがexecコマンドと共に変換される() {
        use crate::rule::parser::{MatchCond, MatchPattern, RawMatchItem};
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "custom-test".to_string(),
                type_: "custom".to_string(),
                exec: Some("cmd {path}".to_string()),
                message: "msg".to_string(),
                match_: vec![RawMatchItem {
                    pattern: MatchPattern::FileNotEndsWith,
                    keywords: vec![".txt".to_string()],
                    cond: MatchCond::And,
                }],
                ..Default::default()
            }]),
            guideline: None,
        };
        let config = Config::try_from(raw).unwrap();
        match &config.rule[0] {
            Rule::Custom(r) => {
                assert_eq!(r.label, "custom-test");
                assert_eq!(r.exec, "cmd {path}");
                assert_eq!(r.matcher.items.len(), 1);
            }
            _ => panic!("Expected Custom rule"),
        }
    }

    #[test]
    fn ガイドライン項目がマッチ条件と共に変換される() {
        use crate::rule::parser::{MatchCond, MatchPattern, RawMatchItem};
        let raw = RawConfig {
            rule: None,
            guideline: Some(vec![
                RawGuidelineItem { message: "guideline1".to_string(), match_: vec![] },
                RawGuidelineItem {
                    message: "guideline2".to_string(),
                    match_: vec![
                        RawMatchItem {
                            pattern: MatchPattern::FileEndsWith,
                            keywords: vec![".java".to_string()],
                            cond: MatchCond::Or,
                        },
                        RawMatchItem {
                            pattern: MatchPattern::FileNotEndsWith,
                            keywords: vec![".test.java".to_string()],
                            cond: MatchCond::And,
                        },
                    ],
                },
            ]),
        };
        let config = Config::try_from(raw).unwrap();
        assert_eq!(config.guideline.len(), 2);
        assert_eq!(config.guideline[0].message, "guideline1");
        assert!(config.guideline[0].matcher.items.is_empty());
        assert_eq!(config.guideline[1].message, "guideline2");
        assert_eq!(config.guideline[1].matcher.items.len(), 2);
    }

    // =========================================================================
    // 設定変換のエラー系
    // =========================================================================

    #[test]
    fn テキストルールにキーワードがないとエラーになる() {
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "forbidden_texts".to_string(),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            guideline: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'forbidden_texts' requires 'keywords'"));
    }

    #[test]
    fn テキストルールにexecがあるとエラーになる() {
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "forbidden_texts".to_string(),
                keywords: Some(vec!["kw".to_string()]),
                exec: Some("cmd".to_string()),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            guideline: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'forbidden_texts' must not have 'exec'"));
    }

    #[test]
    fn 正規表現ルールにキーワードがないとエラーになる() {
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "forbidden_patterns".to_string(),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            guideline: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'forbidden_patterns' requires 'keywords'"));
    }

    #[test]
    fn 正規表現ルールにexecがあるとエラーになる() {
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "forbidden_patterns".to_string(),
                keywords: Some(vec![".*".to_string()]),
                exec: Some("cmd".to_string()),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            guideline: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'forbidden_patterns' must not have 'exec'"));
    }

    #[test]
    fn 不正な正規表現パターンはエラーになる() {
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "forbidden_patterns".to_string(),
                keywords: Some(vec!["[invalid".to_string()]),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            guideline: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("invalid regex"));
    }

    #[test]
    fn カスタムルールにexecがないとエラーになる() {
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "custom".to_string(),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            guideline: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'custom' requires 'exec'"));
    }

    #[test]
    fn カスタムルールにキーワードがあるとエラーになる() {
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "custom".to_string(),
                keywords: Some(vec!["kw".to_string()]),
                exec: Some("cmd".to_string()),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            guideline: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("'custom' must not have 'keywords'"));
    }

    #[test]
    fn 未知のルールタイプはエラーになる() {
        let raw = RawConfig {
            rule: Some(vec![RawRule {
                label: "bad-rule".to_string(),
                type_: "unknown".to_string(),
                message: "msg".to_string(),
                ..Default::default()
            }]),
            guideline: None,
        };
        let err = Config::try_from(raw).unwrap_err();
        assert!(err.to_string().contains("unknown type 'unknown'"));
    }

    // =========================================================================
    // ルールアクセサ
    // =========================================================================

    #[test]
    fn ルールからラベルを取得できる() {
        let text_rule = Rule::Text(TextRule {
            label: "text-label".to_string(),
            keywords: vec![],
            message: "".to_string(),
            matcher: Matcher::default(),
        });
        assert_eq!(text_rule.label(), "text-label");

        let custom_rule = Rule::Custom(CustomRule {
            label: "custom-label".to_string(),
            exec: "".to_string(),
            message: "".to_string(),
            matcher: Matcher::default(),
        });
        assert_eq!(custom_rule.label(), "custom-label");
    }

    #[test]
    fn ルールからキーワードを取得できる() {
        let text_rule = Rule::Text(TextRule {
            label: "".to_string(),
            keywords: vec!["a".to_string(), "b".to_string()],
            message: "".to_string(),
            matcher: Matcher::default(),
        });
        assert_eq!(text_rule.keywords(), Some(&["a".to_string(), "b".to_string()][..]));

        let custom_rule = Rule::Custom(CustomRule {
            label: "".to_string(),
            exec: "".to_string(),
            message: "".to_string(),
            matcher: Matcher::default(),
        });
        assert!(custom_rule.keywords().is_none());
    }
}
