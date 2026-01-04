use std::path::Path;

use anyhow::Result;

use crate::rule::{collect_rules, CollectedRules, Rule};

pub fn run(dir: &Path) -> Result<Vec<String>> {
    let rules = collect_rules(dir)?;
    let mut output = Vec::new();

    for (rule, source_dir) in &rules.rule {
        output.push(format_rule("rule", rule, source_dir, &rules));
    }

    for (item, source_dir) in &rules.guideline {
        let message = &item.message;
        output.push(format_with_source("guideline", message, source_dir, &rules));
    }

    Ok(output)
}

fn format_rule(category: &str, rule: &Rule, source_dir: &Path, rules: &CollectedRules) -> String {
    let label = rule.label();
    let base = if let Some(keywords) = rule.keywords() {
        let keywords_str = keywords.join(", ");
        format!("{category}: {label} [ {keywords_str} ]")
    } else {
        format!("{category}: {label}")
    };

    append_source(&base, source_dir, rules)
}

fn format_with_source(category: &str, message: &str, source_dir: &Path, rules: &CollectedRules) -> String {
    let base = format!("{category}: {message}");
    append_source(&base, source_dir, rules)
}

fn append_source(base: &str, source_dir: &Path, rules: &CollectedRules) -> String {
    if source_dir == rules.root_dir {
        base.to_string()
    } else if let Ok(relative) = source_dir.strip_prefix(&rules.root_dir) {
        format!("{base} @ {}", relative.display())
    } else {
        base.to_string()
    }
}
