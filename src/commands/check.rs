mod html;
mod list;
mod schema;
mod tree;

use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use walkdir::WalkDir;

use super::CheckMode;
use crate::rule::parser::RawConfig;
use crate::rule::root_config::{RawRootConfig, RootConfig};

pub fn run(mode: CheckMode) -> Result<Vec<String>> {
    let current_dir = std::env::current_dir()?;

    match mode {
        CheckMode::List => list::run(&current_dir),
        CheckMode::Tree => tree::run(&current_dir),
        CheckMode::Schema => schema::run(&current_dir),
        CheckMode::Html => html::run(&current_dir),
    }
}

/// Directory with its rule types
pub struct DirWithRules {
    pub relative_path: PathBuf,
    pub rule_types: Vec<String>,
}

/// Find root directory by walking up from start
pub fn find_root_dir(start: &Path) -> Result<PathBuf> {
    let start = start.canonicalize()?;
    let mut current = Some(start.as_path());

    while let Some(dir) = current {
        let root_config_path = dir.join(".rec_lint_config.yaml");
        if root_config_path.exists() {
            return Ok(dir.to_path_buf());
        }
        current = dir.parent();
    }

    bail!("No .rec_lint_config.yaml found in ancestor directories")
}

/// Load root config from root directory
pub fn load_root_config(root: &Path) -> Result<RootConfig> {
    let path = root.join(".rec_lint_config.yaml");
    let raw = RawRootConfig::load(&path)?;
    Ok(RootConfig::from_raw(raw, root))
}

/// Collect directories with .rec_lint.yaml files
pub fn collect_dirs_with_rules(root: &Path, root_config: &RootConfig) -> Result<Vec<DirWithRules>> {
    let mut results = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && !is_excluded(e, root_config))
    {
        let entry = entry?;
        if entry.file_type().is_dir() {
            let config_path = entry.path().join(".rec_lint.yaml");
            if config_path.exists() {
                let raw = RawConfig::load(&config_path)?;
                let rule_types = extract_rule_types(&raw);
                let relative = entry.path().strip_prefix(root)?.to_path_buf();
                results.push(DirWithRules { relative_path: relative, rule_types });
            }
        }
    }

    Ok(results)
}

/// Extract rule types from raw config
pub fn extract_rule_types(config: &RawConfig) -> Vec<String> {
    config
        .rule
        .as_ref()
        .map(|rules| {
            rules
                .iter()
                .filter_map(|r| {
                    if r.forbidden_texts.is_some() {
                        Some("forbidden_texts".to_string())
                    } else if r.forbidden_patterns.is_some() {
                        Some("forbidden_patterns".to_string())
                    } else if r.custom.is_some() {
                        Some("custom".to_string())
                    } else if r.require_php_doc.is_some() {
                        Some("require_php_doc".to_string())
                    } else if r.require_kotlin_doc.is_some() {
                        Some("require_kotlin_doc".to_string())
                    } else if r.require_rust_doc.is_some() {
                        Some("require_rust_doc".to_string())
                    } else if r.require_english_comment.is_some() {
                        Some("require_english_comment".to_string())
                    } else if r.require_japanese_comment.is_some() {
                        Some("require_japanese_comment".to_string())
                    } else if r.require_japanese_phpunit_test_name.is_some() {
                        Some("require_japanese_phpunit_test_name".to_string())
                    } else if r.require_japanese_kotest_test_name.is_some() {
                        Some("require_japanese_kotest_test_name".to_string())
                    } else if r.require_japanese_rust_test_name.is_some() {
                        Some("require_japanese_rust_test_name".to_string())
                    } else if r.require_phpunit_test.is_some() {
                        Some("require_phpunit_test".to_string())
                    } else if r.require_kotest_test.is_some() {
                        Some("require_kotest_test".to_string())
                    } else if r.require_rust_unit_test.is_some() {
                        Some("require_rust_unit_test".to_string())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Check if entry is hidden (starts with .)
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}

/// Check if entry should be excluded based on root config
fn is_excluded(entry: &walkdir::DirEntry, root_config: &RootConfig) -> bool {
    if !entry.file_type().is_dir() {
        return false;
    }
    root_config.should_exclude_dir(entry.file_name())
}
