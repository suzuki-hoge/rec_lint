use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

use super::parser::RawConfig;
use super::root_config::{RawRootConfig, RootConfig};
use super::{Config, GuidelineItem, Rule};

const CONFIG_FILENAME: &str = ".rec_lint.yaml";
const ROOT_CONFIG_FILENAME: &str = ".rec_lint_config.yaml";

pub struct CollectedRules {
    pub root_dir: PathBuf,
    pub root_config: RootConfig,
    pub rule: Vec<(Rule, PathBuf)>,
    pub guideline: Vec<(GuidelineItem, PathBuf)>,
}

pub fn collect_rules(target_dir: &Path) -> Result<CollectedRules> {
    let target_dir = target_dir.canonicalize()?;
    let mut configs: Vec<(Config, PathBuf)> = Vec::new();
    let mut current = Some(target_dir.as_path());
    let mut root_dir: Option<PathBuf> = None;
    let mut root_config: Option<RootConfig> = None;

    while let Some(dir) = current {
        // Check for root marker file
        let root_config_path = dir.join(ROOT_CONFIG_FILENAME);
        let is_root = root_config_path.exists();

        // Load rec_lint.yaml if it exists
        let config_path = dir.join(CONFIG_FILENAME);
        if config_path.exists() {
            let raw = RawConfig::load(&config_path)?;
            let config = Config::try_from(raw)?;
            configs.push((config, dir.to_path_buf()));
        }

        if is_root {
            // Load root config
            let raw_root = RawRootConfig::load(&root_config_path)?;
            root_config = Some(RootConfig::from_raw(raw_root, dir));
            root_dir = Some(dir.to_path_buf());
            break;
        }
        current = dir.parent();
    }

    let root_dir = root_dir.ok_or_else(|| anyhow!("No .rec_lint_config.yaml found in ancestor directories"))?;
    let root_config = root_config.unwrap_or_default();

    configs.reverse();

    let mut collected = CollectedRules { root_dir, root_config, rule: Vec::new(), guideline: Vec::new() };
    for (config, dir) in configs {
        for r in config.rule {
            collected.rule.push((r, dir.clone()));
        }
        for item in config.guideline {
            collected.guideline.push((item, dir.clone()));
        }
    }

    Ok(collected)
}
