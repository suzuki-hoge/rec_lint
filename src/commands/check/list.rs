use std::path::Path;

use anyhow::Result;

use super::{collect_dirs_with_rules, find_root_dir, load_root_config};

pub fn run(start: &Path) -> Result<Vec<String>> {
    let root = find_root_dir(start)?;
    let root_config = load_root_config(&root)?;
    let dirs = collect_dirs_with_rules(&root, &root_config)?;

    let output: Vec<String> = dirs
        .iter()
        .map(|d| {
            let path = if d.relative_path.as_os_str().is_empty() {
                "./.rec_lint.yaml".to_string()
            } else {
                format!("{}/.rec_lint.yaml", d.relative_path.display())
            };
            let types = d.rule_types.join(", ");
            format!("{path}: [ {types} ]")
        })
        .collect();

    Ok(output)
}
