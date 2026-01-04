use std::path::Path;

use anyhow::Result;

use crate::rule::{collect_rules, CollectedRules, Rule};

pub fn run(dir: &Path) -> Result<Vec<String>> {
    let rules = collect_rules(dir)?;
    let mut output = Vec::new();

    for (rule, source_dir) in &rules.rule {
        output.push(format_rule(rule, source_dir, &rules));
    }

    for (item, source_dir) in &rules.guideline {
        let message = &item.message;
        output.push(format_item("guideline", message, source_dir, &rules));
    }

    Ok(output)
}

fn format_rule(rule: &Rule, source_dir: &Path, rules: &CollectedRules) -> String {
    let label = rule.label();
    format_item("rule", label, source_dir, rules)
}

fn format_item(category: &str, message: &str, source_dir: &Path, rules: &CollectedRules) -> String {
    if source_dir == rules.root_dir {
        format!("[ {category} ] {message}")
    } else if let Ok(relative) = source_dir.strip_prefix(&rules.root_dir) {
        format!("[ {category} ] {}: {message}", relative.display())
    } else {
        format!("[ {category} ] {message}")
    }
}
