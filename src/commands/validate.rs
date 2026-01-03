use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::commands::SortMode;
use crate::rule::{collect_rules, CollectedRules, Rule};
use crate::validate::{custom, regex, text, CustomViolation, Violation};

struct FileViolation {
    file: PathBuf,
    root_dir: PathBuf,
    message: String,
    detail: ViolationDetail,
}

enum ViolationDetail {
    LineViolations(Vec<Violation>),
    CustomViolation(CustomViolation),
}

pub fn run(paths: &[PathBuf], sort_mode: SortMode) -> Result<Vec<String>> {
    let files = collect_files(paths);
    if files.is_empty() {
        return Ok(Vec::new());
    }

    let cached = cache_rules(&files);
    let dir_rules = Arc::new(cached.rules);

    let violations: Vec<FileViolation> = files
        .par_iter()
        .flat_map(|file| {
            let parent = match file.parent() {
                Some(p) => p,
                None => return Vec::new(),
            };
            let rules = match dir_rules.get(parent) {
                Some(r) => r,
                None => return Vec::new(),
            };
            validate_file(file, rules).unwrap_or_default()
        })
        .collect();

    let mut output: Vec<String> = cached.errors;
    output.extend(format_violations(&violations, sort_mode));

    Ok(output)
}

fn collect_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    files.push(entry.into_path());
                }
            }
        }
    }
    files
}

struct CachedRules {
    rules: HashMap<PathBuf, CollectedRules>,
    errors: Vec<String>,
}

fn cache_rules(files: &[PathBuf]) -> CachedRules {
    let mut dirs: Vec<PathBuf> = files.iter().filter_map(|f| f.parent().map(|p| p.to_path_buf())).collect();
    dirs.sort();
    dirs.dedup();

    let mut cache = HashMap::new();
    let mut errors = Vec::new();
    for dir in dirs {
        if let std::collections::hash_map::Entry::Vacant(e) = cache.entry(dir.clone()) {
            match collect_rules(&dir) {
                Ok(rules) => {
                    e.insert(rules);
                }
                Err(err) => {
                    errors.push(format!("{}: {err}", dir.display()));
                }
            }
        }
    }
    CachedRules { rules: cache, errors }
}

fn validate_file(file: &Path, rules: &CollectedRules) -> Result<Vec<FileViolation>> {
    let file = file.canonicalize()?;
    let filename = file.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let content = fs::read_to_string(&file)?;
    let mut violations = Vec::new();
    let root_dir = &rules.root_dir;

    for (rule, _source) in &rules.required {
        if !rule.ext_filter().matches(filename) {
            continue;
        }
        if rule.exclude_filter().should_exclude(&file) {
            continue;
        }
        if let Some(v) = validate_rule(&file, root_dir, rule, &content)? {
            violations.push(v);
        }
    }

    for (rule, _source) in &rules.deny {
        if !rule.ext_filter().matches(filename) {
            continue;
        }
        if rule.exclude_filter().should_exclude(&file) {
            continue;
        }
        if let Some(v) = validate_rule(&file, root_dir, rule, &content)? {
            violations.push(v);
        }
    }

    Ok(violations)
}

fn validate_rule(file: &Path, root_dir: &Path, rule: &Rule, content: &str) -> Result<Option<FileViolation>> {
    match rule {
        Rule::Text(text_rule) => {
            let line_violations = text::validate(content, text_rule);
            if !line_violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: text_rule.message.clone(),
                    detail: ViolationDetail::LineViolations(line_violations),
                }));
            }
        }
        Rule::Regex(regex_rule) => {
            let line_violations = regex::validate(content, regex_rule);
            if !line_violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: regex_rule.message.clone(),
                    detail: ViolationDetail::LineViolations(line_violations),
                }));
            }
        }
        Rule::Custom(custom_rule) => {
            if let Some(custom_violation) = custom::validate(file, custom_rule)? {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: custom_rule.message.clone(),
                    detail: ViolationDetail::CustomViolation(custom_violation),
                }));
            }
        }
    }
    Ok(None)
}

/// A flattened violation entry for sorting
struct FlatViolation {
    file: String,
    line: usize,
    col: usize,
    message: String,
    custom_output: Option<String>,
}

fn flatten_violations(violations: &[FileViolation]) -> Vec<FlatViolation> {
    let mut flat = Vec::new();
    for v in violations {
        let relative_path = v
            .file
            .strip_prefix(&v.root_dir)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| v.file.display().to_string());

        match &v.detail {
            ViolationDetail::LineViolations(line_violations) => {
                for lv in line_violations {
                    flat.push(FlatViolation {
                        file: relative_path.clone(),
                        line: lv.line,
                        col: lv.col,
                        message: v.message.clone(),
                        custom_output: None,
                    });
                }
            }
            ViolationDetail::CustomViolation(custom) => {
                flat.push(FlatViolation {
                    file: relative_path.clone(),
                    line: 0,
                    col: 0,
                    message: v.message.clone(),
                    custom_output: if custom.output.is_empty() { None } else { Some(custom.output.clone()) },
                });
            }
        }
    }
    flat
}

fn format_violations(violations: &[FileViolation], sort_mode: SortMode) -> Vec<String> {
    let mut flat = flatten_violations(violations);

    match sort_mode {
        SortMode::Rule => {
            // Already in rule order from parallel collection, just stable sort by message
            flat.sort_by(|a, b| {
                a.message
                    .cmp(&b.message)
                    .then_with(|| a.file.cmp(&b.file))
                    .then_with(|| a.line.cmp(&b.line))
                    .then_with(|| a.col.cmp(&b.col))
            });
        }
        SortMode::File => {
            flat.sort_by(|a, b| {
                a.file
                    .cmp(&b.file)
                    .then_with(|| a.line.cmp(&b.line))
                    .then_with(|| a.col.cmp(&b.col))
                    .then_with(|| a.message.cmp(&b.message))
            });
        }
    }

    let mut output = Vec::new();
    for fv in flat {
        let formatted = match sort_mode {
            SortMode::Rule => {
                // message: file:line:col
                if fv.line == 0 {
                    format!("{}: {}", fv.message, fv.file)
                } else {
                    format!("{}: {}:{}:{}", fv.message, fv.file, fv.line, fv.col)
                }
            }
            SortMode::File => {
                // file:line:col: message
                if fv.line == 0 {
                    format!("{}: {}", fv.file, fv.message)
                } else {
                    format!("{}:{}:{}: {}", fv.file, fv.line, fv.col, fv.message)
                }
            }
        };
        output.push(formatted);
        if let Some(custom_out) = &fv.custom_output {
            output.push(custom_out.clone());
        }
    }
    output
}
