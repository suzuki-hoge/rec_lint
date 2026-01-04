use std::path::Path;

use anyhow::Result;

use crate::rule::collect_rules;

pub fn run(dir: &Path) -> Result<Vec<String>> {
    let rules = collect_rules(dir)?;

    let output: Vec<String> = rules
        .guideline
        .iter()
        .map(|(item, source_dir)| {
            if source_dir == &rules.root_dir {
                format!("[ guideline ] {}", item.message)
            } else if let Ok(relative) = source_dir.strip_prefix(&rules.root_dir) {
                format!("[ guideline ] {}: {}", relative.display(), item.message)
            } else {
                format!("[ guideline ] {}", item.message)
            }
        })
        .collect();

    Ok(output)
}
