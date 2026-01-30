use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;

use super::{extract_rule_types, find_root_dir, load_root_config};
use crate::rule::parser::{MatchPattern, RawConfig, RawRuleContent, RawRuleItem};
use crate::rule::root_config::RootConfig;

/// Rule details for HTML display
#[derive(Clone)]
struct RuleDetail {
    rule_type: String,
    label: String,
    message: String,
    match_info: Vec<String>,
}

/// Guideline details for HTML display
#[derive(Clone)]
struct GuidelineDetail {
    message: String,
    match_info: Vec<String>,
}

/// Combined details for rules and guidelines
type DetailsEntry = (Vec<RuleDetail>, Vec<GuidelineDetail>);

struct TreeNode {
    name: String,
    full_path: PathBuf,
    config_file: Option<PathBuf>, // Full path to .rec_lint.yaml or .rec_lint.yml
    rule_types: Option<Vec<String>>,
    rule_details: Vec<RuleDetail>,
    guideline_details: Vec<GuidelineDetail>,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn has_rules_in_subtree(&self) -> bool {
        if self.rule_types.is_some() {
            return true;
        }
        self.children.iter().any(|c| c.has_rules_in_subtree())
    }

    fn filter_empty_subtrees(self) -> Self {
        let children =
            self.children.into_iter().filter(|c| c.has_rules_in_subtree()).map(|c| c.filter_empty_subtrees()).collect();
        TreeNode {
            name: self.name,
            full_path: self.full_path,
            config_file: self.config_file,
            rule_types: self.rule_types,
            rule_details: self.rule_details,
            guideline_details: self.guideline_details,
            children,
        }
    }
}

pub fn run(start: &Path) -> Result<Vec<String>> {
    let root = find_root_dir(start)?;
    let root_config = load_root_config(&root)?;

    // Build maps
    let rules_map = build_rules_map(&root, &root_config)?;
    let details_map = build_details_map(&root, &root_config)?;

    // Build tree structure
    let tree = build_tree_node(&root, &root, &rules_map, &details_map, &root_config)?;
    let tree = tree.filter_empty_subtrees();

    // Generate HTML
    let html = generate_html(&tree, &root);

    // Write to temp file and open in browser
    let temp_dir = std::env::temp_dir();
    let html_path = temp_dir.join("rec_lint_check.html");
    std::fs::write(&html_path, &html)?;

    open::that(&html_path)?;

    Ok(vec![format!("Opened: {}", html_path.display())])
}

/// Returns (rule_types, config_file_path)
fn build_rules_map(root: &Path, root_config: &RootConfig) -> Result<HashMap<PathBuf, (Vec<String>, PathBuf)>> {
    let mut map = HashMap::new();

    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && !is_excluded(e, root_config))
    {
        let entry = entry?;
        if entry.file_type().is_dir() {
            // Try .rec_lint.yaml first, then .rec_lint.yml
            let yaml_path = entry.path().join(".rec_lint.yaml");
            let yml_path = entry.path().join(".rec_lint.yml");
            let config_path = if yaml_path.exists() {
                Some(yaml_path)
            } else if yml_path.exists() {
                Some(yml_path)
            } else {
                None
            };

            if let Some(config_path) = config_path {
                let raw = RawConfig::load(&config_path)?;
                let types = extract_rule_types(&raw);
                let relative = entry.path().strip_prefix(root)?.to_path_buf();
                map.insert(relative, (types, config_path));
            }
        }
    }

    Ok(map)
}

/// Returns (rule_details, guideline_details)
fn build_details_map(root: &Path, root_config: &RootConfig) -> Result<HashMap<PathBuf, DetailsEntry>> {
    let mut map = HashMap::new();

    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && !is_excluded(e, root_config))
    {
        let entry = entry?;
        if entry.file_type().is_dir() {
            // Try .rec_lint.yaml first, then .rec_lint.yml
            let yaml_path = entry.path().join(".rec_lint.yaml");
            let yml_path = entry.path().join(".rec_lint.yml");
            let config_path = if yaml_path.exists() {
                Some(yaml_path)
            } else if yml_path.exists() {
                Some(yml_path)
            } else {
                None
            };

            if let Some(config_path) = config_path {
                let raw = RawConfig::load(&config_path)?;
                let rule_details = extract_rule_details(&raw);
                let guideline_details = extract_guideline_details(&raw);
                let relative = entry.path().strip_prefix(root)?.to_path_buf();
                map.insert(relative, (rule_details, guideline_details));
            }
        }
    }

    Ok(map)
}

fn extract_rule_details(config: &RawConfig) -> Vec<RuleDetail> {
    let mut details = Vec::new();

    if let Some(rules) = &config.rule {
        for rule in rules {
            if let Some((rule_type, content)) = get_rule_type_and_content(rule) {
                let match_info = content
                    .match_
                    .iter()
                    .map(|m| {
                        let pattern_name = match m.pattern {
                            MatchPattern::FileStartsWith => "file_starts_with",
                            MatchPattern::FileEndsWith => "file_ends_with",
                            MatchPattern::PathContains => "path_contains",
                            MatchPattern::FileNotStartsWith => "file_not_starts_with",
                            MatchPattern::FileNotEndsWith => "file_not_ends_with",
                            MatchPattern::PathNotContains => "path_not_contains",
                        };
                        format!("{}: {}", pattern_name, m.keywords.join(", "))
                    })
                    .collect();

                details.push(RuleDetail {
                    rule_type: rule_type.to_string(),
                    label: content.label.clone(),
                    message: content.message.clone(),
                    match_info,
                });
            }
        }
    }

    details
}

fn extract_guideline_details(config: &RawConfig) -> Vec<GuidelineDetail> {
    let mut details = Vec::new();

    if let Some(guidelines) = &config.guideline {
        for guideline in guidelines {
            let match_info = guideline
                .match_
                .iter()
                .map(|m| {
                    let pattern_name = match m.pattern {
                        MatchPattern::FileStartsWith => "file_starts_with",
                        MatchPattern::FileEndsWith => "file_ends_with",
                        MatchPattern::PathContains => "path_contains",
                        MatchPattern::FileNotStartsWith => "file_not_starts_with",
                        MatchPattern::FileNotEndsWith => "file_not_ends_with",
                        MatchPattern::PathNotContains => "path_not_contains",
                    };
                    format!("{}: {}", pattern_name, m.keywords.join(", "))
                })
                .collect();

            details.push(GuidelineDetail { message: guideline.message.clone(), match_info });
        }
    }

    details
}

fn get_rule_type_and_content(rule: &RawRuleItem) -> Option<(&str, &RawRuleContent)> {
    if let Some(c) = &rule.forbidden_texts {
        return Some(("forbidden_texts", c));
    }
    if let Some(c) = &rule.forbidden_patterns {
        return Some(("forbidden_patterns", c));
    }
    if let Some(c) = &rule.custom {
        return Some(("custom", c));
    }
    if let Some(c) = &rule.require_php_doc {
        return Some(("require_php_doc", c));
    }
    if let Some(c) = &rule.require_kotlin_doc {
        return Some(("require_kotlin_doc", c));
    }
    if let Some(c) = &rule.require_rust_doc {
        return Some(("require_rust_doc", c));
    }
    if let Some(c) = &rule.require_english_comment {
        return Some(("require_english_comment", c));
    }
    if let Some(c) = &rule.require_japanese_comment {
        return Some(("require_japanese_comment", c));
    }
    if let Some(c) = &rule.require_japanese_phpunit_test_name {
        return Some(("require_japanese_phpunit_test_name", c));
    }
    if let Some(c) = &rule.require_japanese_kotest_test_name {
        return Some(("require_japanese_kotest_test_name", c));
    }
    if let Some(c) = &rule.require_japanese_rust_test_name {
        return Some(("require_japanese_rust_test_name", c));
    }
    if let Some(c) = &rule.require_phpunit_test {
        return Some(("require_phpunit_test", c));
    }
    if let Some(c) = &rule.require_kotest_test {
        return Some(("require_kotest_test", c));
    }
    if let Some(c) = &rule.require_rust_unit_test {
        return Some(("require_rust_unit_test", c));
    }
    None
}

fn build_tree_node(
    dir: &Path,
    root: &Path,
    rules_map: &HashMap<PathBuf, (Vec<String>, PathBuf)>,
    details_map: &HashMap<PathBuf, DetailsEntry>,
    root_config: &RootConfig,
) -> Result<TreeNode> {
    let relative = dir.strip_prefix(root).unwrap_or(Path::new("."));
    let name = if relative.as_os_str().is_empty() {
        ".".to_string()
    } else {
        dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| ".".to_string())
    };

    let (rule_types, config_file) = match rules_map.get(&relative.to_path_buf()) {
        Some((types, config_path)) => (Some(types.clone()), Some(config_path.clone())),
        None => (None, None),
    };
    let (rule_details, guideline_details) = details_map.get(&relative.to_path_buf()).cloned().unwrap_or_default();

    let mut children = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().unwrap();
                if !dir_name.to_string_lossy().starts_with('.') && !root_config.should_exclude_dir(dir_name) {
                    children.push(build_tree_node(&path, root, rules_map, details_map, root_config)?);
                }
            }
        }
    }

    children.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(TreeNode {
        name,
        full_path: dir.to_path_buf(),
        config_file,
        rule_types,
        rule_details,
        guideline_details,
        children,
    })
}

fn generate_html(tree: &TreeNode, root: &Path) -> String {
    let tree_html = render_tree_html(tree);
    let root_path = root.display().to_string();

    format!(
        r#"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>rec_lint check - {root_path}</title>
<style>
* {{
    box-sizing: border-box;
}}
body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    margin: 0;
    padding: 20px;
    background: #1a1a2e;
    color: #eee;
    line-height: 1.6;
}}
h1 {{
    font-size: 1.2rem;
    color: #888;
    margin-bottom: 20px;
    font-weight: normal;
}}
.tree {{
    font-family: "SF Mono", Monaco, "Cascadia Code", Consolas, monospace;
    font-size: 14px;
}}
.node {{
    padding: 4px 0;
}}
.node-header {{
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    transition: background 0.15s;
}}
.node-header:hover {{
    background: #2a2a4e;
}}
.toggle {{
    width: 16px;
    color: #666;
    user-select: none;
    transition: transform 0.15s;
}}
.toggle.expanded {{
    transform: rotate(90deg);
}}
.toggle.empty {{
    visibility: hidden;
}}
.name {{
    color: #7dd3fc;
}}
.name.has-config {{
    cursor: pointer;
}}
.name.has-config:hover {{
    text-decoration: underline;
}}
.name.no-config {{
    color: #666;
    cursor: default;
}}
.rules {{
    display: inline-flex;
    gap: 6px;
    margin-left: 8px;
}}
.rule-badge {{
    background: #3b3b5c;
    color: #a5b4fc;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 12px;
    cursor: help;
    position: relative;
}}
.rule-badge:hover {{
    background: #4b4b7c;
}}
.guideline-badge {{
    background: #3b4a5c;
    color: #9cb8c8;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 12px;
    cursor: help;
    position: relative;
}}
.guideline-badge:hover {{
    background: #4a5a6c;
}}
.tooltip {{
    display: none;
    position: fixed;
    background: #2a2a4e;
    border: 1px solid #4b4b7c;
    border-radius: 6px;
    padding: 12px;
    min-width: 600px;
    max-width: 900px;
    z-index: 1000;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    white-space: normal;
}}
.tooltip-label {{
    font-weight: 600;
    color: #7dd3fc;
    margin-bottom: 6px;
}}
.tooltip-message {{
    color: #ccc;
    margin-bottom: 8px;
}}
.tooltip-match {{
    color: #888;
    font-size: 12px;
    font-family: "SF Mono", Monaco, monospace;
}}
.tooltip-match-item {{
    margin: 2px 0;
}}
.children {{
    margin-left: 24px;
    border-left: 1px solid #333;
    padding-left: 8px;
}}
.children.collapsed {{
    display: none;
}}
.copied-toast {{
    position: fixed;
    bottom: 20px;
    right: 20px;
    background: #22c55e;
    color: #fff;
    padding: 12px 20px;
    border-radius: 6px;
    opacity: 0;
    transition: opacity 0.3s;
    pointer-events: none;
}}
.copied-toast.show {{
    opacity: 1;
}}
</style>
</head>
<body>
<h1>rec_lint check: {root_path}</h1>
<div class="tree">
{tree_html}
</div>
<div class="copied-toast" id="toast">Copied to clipboard!</div>
<script>
document.querySelectorAll('.toggle').forEach(toggle => {{
    toggle.addEventListener('click', (e) => {{
        e.stopPropagation();
        const node = toggle.closest('.node');
        const children = node.querySelector(':scope > .children');
        if (children) {{
            children.classList.toggle('collapsed');
            toggle.classList.toggle('expanded');
        }}
    }});
}});

// Only clickable names with config files
document.querySelectorAll('.name.has-config').forEach(name => {{
    name.addEventListener('click', (e) => {{
        e.stopPropagation();
        const path = name.dataset.path;
        navigator.clipboard.writeText(path).then(() => {{
            const toast = document.getElementById('toast');
            toast.textContent = 'Copied: ' + path;
            toast.classList.add('show');
            setTimeout(() => toast.classList.remove('show'), 2000);
        }});
    }});
}});

// Tooltip positioning
document.querySelectorAll('.rule-badge, .guideline-badge').forEach(badge => {{
    const tooltip = badge.querySelector('.tooltip');
    if (!tooltip) return;

    badge.addEventListener('mouseenter', (e) => {{
        const rect = badge.getBoundingClientRect();
        const viewportHeight = window.innerHeight;
        const tooltipHeight = 200; // Approximate height

        tooltip.style.display = 'block';
        tooltip.style.left = rect.left + 'px';

        // Show above if in lower half of screen
        if (rect.top > viewportHeight / 2) {{
            tooltip.style.bottom = (viewportHeight - rect.top + 8) + 'px';
            tooltip.style.top = 'auto';
        }} else {{
            tooltip.style.top = (rect.bottom + 8) + 'px';
            tooltip.style.bottom = 'auto';
        }}
    }});

    badge.addEventListener('mouseleave', () => {{
        tooltip.style.display = 'none';
    }});
}});

// Expand all by default
document.querySelectorAll('.toggle').forEach(toggle => {{
    toggle.classList.add('expanded');
}});
</script>
</body>
</html>"#
    )
}

fn render_tree_html(node: &TreeNode) -> String {
    let has_children = !node.children.is_empty();
    let toggle_class = if has_children { "" } else { "empty" };

    let rules_html = if !node.rule_details.is_empty() {
        let badges: Vec<String> = node
            .rule_details
            .iter()
            .map(|detail| {
                let match_html = if detail.match_info.is_empty() {
                    String::new()
                } else {
                    let items: Vec<String> = detail
                        .match_info
                        .iter()
                        .map(|m| format!(r#"<div class="tooltip-match-item">{}</div>"#, html_escape(m)))
                        .collect();
                    format!(r#"<div class="tooltip-match">{}</div>"#, items.join(""))
                };

                let label_html = if detail.label.is_empty() {
                    String::new()
                } else {
                    format!(r#"<div class="tooltip-label">{}</div>"#, html_escape(&detail.label))
                };

                let message_html = if detail.message.is_empty() {
                    String::new()
                } else {
                    format!(r#"<div class="tooltip-message">{}</div>"#, html_escape(&detail.message))
                };

                format!(
                    r#"<span class="rule-badge">{}<div class="tooltip">{}{}{}</div></span>"#,
                    html_escape(&detail.rule_type),
                    label_html,
                    message_html,
                    match_html
                )
            })
            .collect();
        format!(r#"<span class="rules">{}</span>"#, badges.join(""))
    } else {
        String::new()
    };

    let guidelines_html = if !node.guideline_details.is_empty() {
        let badges: Vec<String> = node
            .guideline_details
            .iter()
            .map(|detail| {
                let match_html = if detail.match_info.is_empty() {
                    String::new()
                } else {
                    let items: Vec<String> = detail
                        .match_info
                        .iter()
                        .map(|m| format!(r#"<div class="tooltip-match-item">{}</div>"#, html_escape(m)))
                        .collect();
                    format!(r#"<div class="tooltip-match">{}</div>"#, items.join(""))
                };

                let message_html = if detail.message.is_empty() {
                    String::new()
                } else {
                    format!(r#"<div class="tooltip-message">{}</div>"#, html_escape(&detail.message))
                };

                format!(
                    r#"<span class="guideline-badge">guideline<div class="tooltip">{message_html}{match_html}</div></span>"#
                )
            })
            .collect();
        format!(r#"<span class="rules">{}</span>"#, badges.join(""))
    } else {
        String::new()
    };

    let children_html = if has_children {
        let child_nodes: Vec<String> = node.children.iter().map(render_tree_html).collect();
        format!(r#"<div class="children">{}</div>"#, child_nodes.join(""))
    } else {
        String::new()
    };

    // Use config file path for clickable items, empty for non-config dirs
    let (name_class, data_path) = match &node.config_file {
        Some(config_path) => ("has-config", config_path.display().to_string()),
        None => ("no-config", String::new()),
    };

    format!(
        r#"<div class="node">
<div class="node-header">
<span class="toggle {toggle_class}">▶</span>
<span class="name {name_class}" data-path="{}">{}</span>
{rules_html}
{guidelines_html}
</div>
{children_html}
</div>"#,
        html_escape(&data_path),
        html_escape(&node.name)
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}

fn is_excluded(entry: &walkdir::DirEntry, root_config: &RootConfig) -> bool {
    if !entry.file_type().is_dir() {
        return false;
    }
    root_config.should_exclude_dir(entry.file_name())
}
