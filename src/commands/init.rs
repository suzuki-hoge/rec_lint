use std::fs;
use std::path::Path;

use anyhow::{bail, Result};

const VERSION: &str = include_str!("../../.version");

const TEMPLATE: &str = r#"# yaml-language-server: $schema=https://raw.githubusercontent.com/suzuki-hoge/rec_lint/refs/tags/v{version}/schema/rec_lint_config.schema.json
"#;

pub fn run(dir: &Path) -> Result<Vec<String>> {
    let config_path = dir.join(".rec_lint_config.yaml");

    if config_path.exists() {
        bail!("File already exists: {}", config_path.display());
    }

    let content = TEMPLATE.replace("{version}", VERSION.trim());
    fs::write(&config_path, content)?;

    Ok(vec![format!("Created: {}", config_path.display())])
}
