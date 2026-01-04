use std::fs;
use std::path::Path;

use anyhow::{bail, Result};

const VERSION: &str = include_str!("../../.version");

const TEMPLATE: &str = r#"# yaml-language-server: $schema=https://raw.githubusercontent.com/suzuki-hoge/rec_lint/refs/heads/v{version}/schema/rec_lint.schema.json

rule:

guideline:
"#;

pub fn run(dir: &Path) -> Result<Vec<String>> {
    let file_path = dir.join(".rec_lint.yaml");

    if file_path.exists() {
        bail!("File already exists: {}", file_path.display());
    }

    let content = TEMPLATE.replace("{version}", VERSION.trim());
    fs::write(&file_path, content)?;

    Ok(vec![format!("Created: {}", file_path.display())])
}
