use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::commands::SortMode;
use crate::rule::{collect_rules, CollectedRules, CommentSource, RootConfig, Rule};
use crate::validate::comment::{self, CommentViolation};
use crate::validate::doc::{self, DocViolation};
use crate::validate::test::exists::{self as test_exists, TestExistenceViolation};
use crate::validate::test::{self, TestViolation};
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
    DocViolations(Vec<DocViolation>),
    CommentViolations(Vec<CommentViolation>),
    TestViolations(Vec<TestViolation>),
    TestExistenceViolations(Vec<TestExistenceViolation>),
}

pub fn run(paths: &[PathBuf], sort_mode: SortMode) -> Result<Vec<String>> {
    // First, get root_config from the first path
    let root_config = get_root_config_for_paths(paths);
    let files = collect_files(paths, &root_config);
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

/// Get root config for the given paths (uses the first path's root config)
fn get_root_config_for_paths(paths: &[PathBuf]) -> RootConfig {
    for path in paths {
        let dir = if path.is_file() { path.parent().unwrap_or(path) } else { path.as_path() };
        if let Ok(rules) = collect_rules(dir) {
            return rules.root_config;
        }
    }
    RootConfig::default()
}

fn collect_files(paths: &[PathBuf], root_config: &RootConfig) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            if !is_config_file(path) && should_include_file(path, root_config) {
                files.push(path.clone());
            }
        } else if path.is_dir() {
            let walker = WalkDir::new(path).into_iter().filter_entry(|e| {
                // Skip excluded directories
                if e.file_type().is_dir() {
                    if let Some(name) = e.file_name().to_str() {
                        // Always exclude .git directory
                        if name == ".git" {
                            return false;
                        }
                        if root_config.should_exclude_dir(std::ffi::OsStr::new(name)) {
                            return false;
                        }
                    }
                }
                true
            });
            for entry in walker.filter_map(|e| e.ok()) {
                if entry.file_type().is_file()
                    && !is_config_file(entry.path())
                    && should_include_file(entry.path(), root_config)
                {
                    files.push(entry.into_path());
                }
            }
        }
    }
    files
}

fn should_include_file(path: &Path, root_config: &RootConfig) -> bool {
    root_config.should_include_extension(path.extension())
}

fn is_config_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|name| name == ".rec_lint.yaml" || name == ".rec_lint_config.yaml")
        .unwrap_or(false)
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
    let content = fs::read_to_string(&file)?;
    let mut violations = Vec::new();
    let root_dir = &rules.root_dir;

    for (rule, _source) in &rules.rule {
        if !rule.matcher().matches(&file) {
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
        Rule::PhpDoc(rule) => {
            let violations = doc::php::validate(content, &rule.config);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::DocViolations(violations),
                }));
            }
        }
        Rule::KotlinDoc(rule) => {
            let violations = doc::kotlin::validate(content, &rule.config);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::DocViolations(violations),
                }));
            }
        }
        Rule::RustDoc(rule) => {
            let violations = doc::rust::validate(content, &rule.config);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::DocViolations(violations),
                }));
            }
        }
        Rule::JapaneseComment(rule) => {
            let comments = extract_comments(content, &rule.source);
            let violations = comment::validate_japanese(&comments);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::CommentViolations(violations),
                }));
            }
        }
        Rule::EnglishComment(rule) => {
            let comments = extract_comments(content, &rule.source);
            let violations = comment::validate_non_japanese(&comments);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::CommentViolations(violations),
                }));
            }
        }
        Rule::PhpUnitTest(rule) => {
            let violations = test::name::phpunit::validate(content);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::TestViolations(violations),
                }));
            }
        }
        Rule::KotestTest(rule) => {
            let violations = test::name::kotest::validate(content);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::TestViolations(violations),
                }));
            }
        }
        Rule::RustTest(rule) => {
            let violations = test::name::rust::validate(content);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::TestViolations(violations),
                }));
            }
        }
        Rule::PhpUnitTestExistence(rule) => {
            let violations = test_exists::phpunit::validate(file, content, root_dir, &rule.config);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::TestExistenceViolations(violations),
                }));
            }
        }
        Rule::KotestTestExistence(rule) => {
            let violations = test_exists::kotest::validate(file, content, root_dir, &rule.config);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::TestExistenceViolations(violations),
                }));
            }
        }
        Rule::RustTestExistence(rule) => {
            let violations = test_exists::rust::validate(content, &rule.config);
            if !violations.is_empty() {
                return Ok(Some(FileViolation {
                    file: file.to_path_buf(),
                    root_dir: root_dir.to_path_buf(),
                    message: rule.message.clone(),
                    detail: ViolationDetail::TestExistenceViolations(violations),
                }));
            }
        }
    }
    Ok(None)
}

fn extract_comments(content: &str, source: &CommentSource) -> Vec<comment::Comment> {
    match source {
        CommentSource::Lang(lang) => match lang {
            crate::rule::parser::CommentLang::Java => comment::java::extract_comments(content),
            crate::rule::parser::CommentLang::Kotlin => comment::kotlin::extract_comments(content),
            crate::rule::parser::CommentLang::Rust => comment::rust::extract_comments(content),
        },
        CommentSource::Custom(syntax) => comment::custom::extract_comments(content, syntax),
    }
}

/// A flattened violation entry for sorting
struct FlatViolation {
    file: String,
    line: usize,
    col: usize,
    message: String,
    found: Option<String>,
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
                        found: None,
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
                    found: None,
                    custom_output: if custom.output.is_empty() { None } else { Some(custom.output.clone()) },
                });
            }
            ViolationDetail::DocViolations(doc_violations) => {
                for dv in doc_violations {
                    flat.push(FlatViolation {
                        file: relative_path.clone(),
                        line: dv.line,
                        col: 1,
                        message: v.message.clone(),
                        found: Some(format!("{} {}", dv.kind, dv.name)),
                        custom_output: None,
                    });
                }
            }
            ViolationDetail::CommentViolations(comment_violations) => {
                for cv in comment_violations {
                    flat.push(FlatViolation {
                        file: relative_path.clone(),
                        line: cv.line,
                        col: 1,
                        message: v.message.clone(),
                        found: Some(truncate_text(&cv.text, 40)),
                        custom_output: None,
                    });
                }
            }
            ViolationDetail::TestViolations(test_violations) => {
                for tv in test_violations {
                    flat.push(FlatViolation {
                        file: relative_path.clone(),
                        line: tv.line,
                        col: 1,
                        message: v.message.clone(),
                        found: Some(tv.name.clone()),
                        custom_output: None,
                    });
                }
            }
            ViolationDetail::TestExistenceViolations(existence_violations) => {
                for ev in existence_violations {
                    // Line number depends on the violation kind
                    let line = match &ev.kind {
                        test_exists::TestExistenceViolationKind::UntestedPublicMethod { line, .. } => *line,
                        test_exists::TestExistenceViolationKind::UntestedPublicFunction { line, .. } => *line,
                        _ => 0, // File-level violations don't have a line number
                    };
                    flat.push(FlatViolation {
                        file: relative_path.clone(),
                        line,
                        col: 1,
                        message: v.message.clone(),
                        found: Some(ev.kind.to_string()),
                        custom_output: None,
                    });
                }
            }
        }
    }
    flat
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
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
        let found_suffix = match &fv.found {
            Some(found) => format!(" [ found: {found} ]"),
            None => String::new(),
        };
        let formatted = match sort_mode {
            SortMode::Rule => {
                // message: file:line:col [ found: xxx ]
                if fv.line == 0 {
                    format!("{}: {}{}", fv.message, fv.file, found_suffix)
                } else {
                    format!("{}: {}:{}:{}{}", fv.message, fv.file, fv.line, fv.col, found_suffix)
                }
            }
            SortMode::File => {
                // file:line:col: message [ found: xxx ]
                if fv.line == 0 {
                    format!("{}: {}{}", fv.file, fv.message, found_suffix)
                } else {
                    format!("{}:{}:{}: {}{}", fv.file, fv.line, fv.col, fv.message, found_suffix)
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
