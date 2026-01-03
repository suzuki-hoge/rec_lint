use std::path::Path;

use anyhow::Result;

use crate::collector::collect_rules;

pub fn run(dir: &Path) -> Result<Vec<String>> {
    let rules = collect_rules(dir)?;

    let output: Vec<String> = rules
        .review
        .iter()
        .map(|(item, source_dir)| {
            let base = format!("review: {}", item.message);
            if source_dir == &rules.root_dir {
                base
            } else if let Ok(relative) = source_dir.strip_prefix(&rules.root_dir) {
                format!("{base} @ {}", relative.display())
            } else {
                base
            }
        })
        .collect();

    Ok(output)
}
