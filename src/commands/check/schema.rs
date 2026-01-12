use std::path::Path;

use anyhow::Result;
use serde_json::Value;
use walkdir::WalkDir;

use super::{find_root_dir, load_root_config};
use crate::rule::root_config::RootConfig;

// Embed the bundled schema at compile time (all definitions are inlined)
const BUNDLED_SCHEMA: &str = include_str!("../../../schema/rec_lint.schema.json");

pub fn run(start: &Path) -> Result<Vec<String>> {
    let root = find_root_dir(start)?;
    let root_config = load_root_config(&root)?;
    let mut output = Vec::new();
    let mut has_errors = false;

    // Compile schema (no retriever needed since all refs are resolved in bundled schema)
    let schema_json: Value = serde_json::from_str(BUNDLED_SCHEMA)?;
    let compiled = jsonschema::options().build(&schema_json)?;

    // Find all .rec_lint.yaml files
    for entry in WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && !is_excluded(e, &root_config))
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.file_name() == Some(std::ffi::OsStr::new(".rec_lint.yaml")) {
            match validate_file(path, &compiled) {
                Ok(()) => {
                    // File is valid
                }
                Err(errors) => {
                    has_errors = true;
                    let relative = path.strip_prefix(&root).unwrap_or(path);
                    output.push(format!("Invalid: {}", relative.display()));
                    for error in errors {
                        output.push(format!("  - {error}"));
                    }
                }
            }
        }
    }

    if !has_errors {
        output.push("All .rec_lint.yaml files are valid.".to_string());
    }

    Ok(output)
}

fn validate_file(path: &Path, schema: &jsonschema::Validator) -> std::result::Result<(), Vec<String>> {
    let content = std::fs::read_to_string(path).map_err(|e| vec![e.to_string()])?;

    // Handle empty or comment-only files
    if content.trim().is_empty() || (content.trim().starts_with('#') && !content.contains(':')) {
        return Ok(());
    }

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|e| vec![e.to_string()])?;
    let json_value: Value = serde_json::to_value(yaml_value).map_err(|e| vec![e.to_string()])?;

    // Collect all validation errors
    let errors: Vec<String> =
        schema.iter_errors(&json_value).map(|e| format!("{} at {}", e, e.instance_path())).collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    // Don't skip .rec_lint.yaml files themselves, only hidden directories
    if entry.file_type().is_file() {
        return false;
    }
    entry.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}

fn is_excluded(entry: &walkdir::DirEntry, root_config: &RootConfig) -> bool {
    if !entry.file_type().is_dir() {
        return false;
    }
    root_config.should_exclude_dir(entry.file_name())
}
