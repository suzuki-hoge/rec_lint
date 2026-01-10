use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;

use super::{extract_rule_types, find_root_dir, load_root_config};
use crate::rule::parser::RawConfig;
use crate::rule::root_config::RootConfig;

// Tree drawing characters (ASCII)
const BRANCH: &str = "|-- ";
const LAST_BRANCH: &str = "`-- ";
const VERTICAL: &str = "|   ";
const EMPTY: &str = "    ";

struct TreeNode {
    name: String,
    rule_types: Option<Vec<String>>,
    children: Vec<TreeNode>,
}

impl TreeNode {
    /// Returns true if this node or any descendant has rules
    fn has_rules_in_subtree(&self) -> bool {
        if self.rule_types.is_some() {
            return true;
        }
        self.children.iter().any(|c| c.has_rules_in_subtree())
    }

    /// Filter out children that have no rules in their subtree
    fn filter_empty_subtrees(self) -> Self {
        let children =
            self.children.into_iter().filter(|c| c.has_rules_in_subtree()).map(|c| c.filter_empty_subtrees()).collect();
        TreeNode { name: self.name, rule_types: self.rule_types, children }
    }
}

pub fn run(start: &Path) -> Result<Vec<String>> {
    let root = find_root_dir(start)?;
    let root_config = load_root_config(&root)?;

    // Build a map of relative_path -> rule types
    let rules_map = build_rules_map(&root, &root_config)?;

    // Build tree structure
    let tree = build_tree_node(&root, &root, &rules_map, &root_config)?;

    // Filter out directories with no rules in subtree
    let tree = tree.filter_empty_subtrees();

    // Calculate max width of name column (prefix + name)
    let max_name_width = calculate_max_name_width(&tree, &[]);

    // Render tree to output lines
    let output = render_tree(&tree, &[], max_name_width + 4);

    Ok(output)
}

fn build_rules_map(root: &Path, root_config: &RootConfig) -> Result<HashMap<PathBuf, Vec<String>>> {
    let mut map = HashMap::new();

    // Walk all directories to find .rec_lint.yaml files
    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && !is_excluded(e, root_config))
    {
        let entry = entry?;
        if entry.file_type().is_dir() {
            let config_path = entry.path().join(".rec_lint.yaml");
            if config_path.exists() {
                let raw = RawConfig::load(&config_path)?;
                let types = extract_rule_types(&raw);
                let relative = entry.path().strip_prefix(root)?.to_path_buf();
                map.insert(relative, types);
            }
        }
    }

    Ok(map)
}

fn build_tree_node(
    dir: &Path,
    root: &Path,
    rules_map: &HashMap<PathBuf, Vec<String>>,
    root_config: &RootConfig,
) -> Result<TreeNode> {
    let relative = dir.strip_prefix(root).unwrap_or(Path::new("."));
    let name = if relative.as_os_str().is_empty() {
        ".".to_string()
    } else {
        dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| ".".to_string())
    };

    let rule_types = rules_map.get(&relative.to_path_buf()).cloned();

    let mut children = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().unwrap();
                // Skip hidden and excluded directories
                if !dir_name.to_string_lossy().starts_with('.') && !root_config.should_exclude_dir(dir_name) {
                    children.push(build_tree_node(&path, root, rules_map, root_config)?);
                }
            }
        }
    }

    // Sort children alphabetically
    children.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(TreeNode { name, rule_types, children })
}

/// Calculate the maximum width of the name column
/// ancestors: stack of booleans indicating if each ancestor is the last child
fn calculate_max_name_width(node: &TreeNode, ancestors: &[bool]) -> usize {
    // Root has no prefix, children have prefix based on depth
    let prefix_width = if ancestors.is_empty() { 0 } else { ancestors.len() * 4 };
    let current_width = prefix_width + node.name.len();

    let mut max_width = current_width;
    let child_count = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        let is_last = i == child_count - 1;
        let mut child_ancestors = ancestors.to_vec();
        child_ancestors.push(is_last);
        let child_width = calculate_max_name_width(child, &child_ancestors);
        max_width = max_width.max(child_width);
    }

    max_width
}

/// Render the tree to output lines
/// ancestors: stack of booleans indicating if each ancestor is the last child
fn render_tree(node: &TreeNode, ancestors: &[bool], rule_column: usize) -> Vec<String> {
    let mut output = Vec::new();

    // Build the prefix string
    let prefix = if ancestors.is_empty() {
        String::new()
    } else {
        let mut p = String::new();
        // Add vertical lines or spaces for all ancestors except the last
        for &is_last in &ancestors[..ancestors.len() - 1] {
            p.push_str(if is_last { EMPTY } else { VERTICAL });
        }
        // Add branch for current node
        let is_last = *ancestors.last().unwrap();
        p.push_str(if is_last { LAST_BRANCH } else { BRANCH });
        p
    };

    let name_part = format!("{}{}", prefix, node.name);

    let line = if let Some(types) = &node.rule_types {
        let types_str = format!("[ {} ]", types.join(", "));
        format!("{name_part:<rule_column$}{types_str}")
    } else {
        name_part
    };
    output.push(line);

    let child_count = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        let is_last = i == child_count - 1;
        let mut child_ancestors = ancestors.to_vec();
        child_ancestors.push(is_last);
        output.extend(render_tree(child, &child_ancestors, rule_column));
    }

    output
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
