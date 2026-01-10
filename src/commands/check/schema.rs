use std::path::Path;

use anyhow::Result;
use jsonschema::Retrieve;
use serde_json::Value;
use walkdir::WalkDir;

use super::{find_root_dir, load_root_config};
use crate::rule::root_config::RootConfig;

// Embed all schema files at compile time
const MAIN_SCHEMA: &str = include_str!("../../../schema/rec_lint.schema.json");
const COMMON_SCHEMA: &str = include_str!("../../../schema/rules/common.schema.json");
const GUIDELINE_SCHEMA: &str = include_str!("../../../schema/rules/guideline.schema.json");
const FORBIDDEN_TEXTS_SCHEMA: &str = include_str!("../../../schema/rules/forbidden-texts.schema.json");
const FORBIDDEN_PATTERNS_SCHEMA: &str = include_str!("../../../schema/rules/forbidden-patterns.schema.json");
const CUSTOM_SCHEMA: &str = include_str!("../../../schema/rules/custom.schema.json");
const REQUIRE_PHP_DOC_SCHEMA: &str = include_str!("../../../schema/rules/require-php-doc.schema.json");
const REQUIRE_KOTLIN_DOC_SCHEMA: &str = include_str!("../../../schema/rules/require-kotlin-doc.schema.json");
const REQUIRE_RUST_DOC_SCHEMA: &str = include_str!("../../../schema/rules/require-rust-doc.schema.json");
const REQUIRE_ENGLISH_COMMENT_SCHEMA: &str = include_str!("../../../schema/rules/require-english-comment.schema.json");
const REQUIRE_JAPANESE_COMMENT_SCHEMA: &str =
    include_str!("../../../schema/rules/require-japanese-comment.schema.json");
const REQUIRE_JAPANESE_PHPUNIT_TEST_NAME_SCHEMA: &str =
    include_str!("../../../schema/rules/require-japanese-phpunit-test-name.schema.json");
const REQUIRE_JAPANESE_KOTEST_TEST_NAME_SCHEMA: &str =
    include_str!("../../../schema/rules/require-japanese-kotest-test-name.schema.json");
const REQUIRE_JAPANESE_RUST_TEST_NAME_SCHEMA: &str =
    include_str!("../../../schema/rules/require-japanese-rust-test-name.schema.json");
const REQUIRE_PHPUNIT_TEST_SCHEMA: &str = include_str!("../../../schema/rules/require-phpunit-test.schema.json");
const REQUIRE_KOTEST_TEST_SCHEMA: &str = include_str!("../../../schema/rules/require-kotest-test.schema.json");
const REQUIRE_RUST_UNIT_TEST_SCHEMA: &str = include_str!("../../../schema/rules/require-rust-unit-test.schema.json");

struct EmbeddedSchemaRetriever;

impl Retrieve for EmbeddedSchemaRetriever {
    fn retrieve(
        &self,
        uri: &jsonschema::Uri<String>,
    ) -> std::result::Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let path = uri.path().to_string();

        let content = match path.as_str() {
            p if p.ends_with("common.schema.json") => COMMON_SCHEMA,
            p if p.ends_with("guideline.schema.json") => GUIDELINE_SCHEMA,
            p if p.ends_with("forbidden-texts.schema.json") => FORBIDDEN_TEXTS_SCHEMA,
            p if p.ends_with("forbidden-patterns.schema.json") => FORBIDDEN_PATTERNS_SCHEMA,
            p if p.ends_with("custom.schema.json") => CUSTOM_SCHEMA,
            p if p.ends_with("require-php-doc.schema.json") => REQUIRE_PHP_DOC_SCHEMA,
            p if p.ends_with("require-kotlin-doc.schema.json") => REQUIRE_KOTLIN_DOC_SCHEMA,
            p if p.ends_with("require-rust-doc.schema.json") => REQUIRE_RUST_DOC_SCHEMA,
            p if p.ends_with("require-english-comment.schema.json") => REQUIRE_ENGLISH_COMMENT_SCHEMA,
            p if p.ends_with("require-japanese-comment.schema.json") => REQUIRE_JAPANESE_COMMENT_SCHEMA,
            p if p.ends_with("require-japanese-phpunit-test-name.schema.json") => {
                REQUIRE_JAPANESE_PHPUNIT_TEST_NAME_SCHEMA
            }
            p if p.ends_with("require-japanese-kotest-test-name.schema.json") => {
                REQUIRE_JAPANESE_KOTEST_TEST_NAME_SCHEMA
            }
            p if p.ends_with("require-japanese-rust-test-name.schema.json") => REQUIRE_JAPANESE_RUST_TEST_NAME_SCHEMA,
            p if p.ends_with("require-phpunit-test.schema.json") => REQUIRE_PHPUNIT_TEST_SCHEMA,
            p if p.ends_with("require-kotest-test.schema.json") => REQUIRE_KOTEST_TEST_SCHEMA,
            p if p.ends_with("require-rust-unit-test.schema.json") => REQUIRE_RUST_UNIT_TEST_SCHEMA,
            _ => return Err(format!("Unknown schema: {path}").into()),
        };

        let value: Value = serde_json::from_str(content)?;
        Ok(value)
    }
}

pub fn run(start: &Path) -> Result<Vec<String>> {
    let root = find_root_dir(start)?;
    let root_config = load_root_config(&root)?;
    let mut output = Vec::new();
    let mut has_errors = false;

    // Compile schema with custom retriever
    let schema_json: Value = serde_json::from_str(MAIN_SCHEMA)?;
    let compiled = jsonschema::options().with_retriever(EmbeddedSchemaRetriever).build(&schema_json)?;

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
