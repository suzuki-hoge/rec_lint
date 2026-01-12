use anyhow::{Context, Result};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const OUTPUT_DIR: &str = "docs/schema";
const RULES_OUTPUT_DIR: &str = "docs/schema/rules";

struct SchemaConfig {
    input: &'static str,
    output: &'static str,
    is_index: bool,
}

const SCHEMAS: &[SchemaConfig] = &[
    SchemaConfig {
        input: "schema/parts/rec_lint.schema.json",
        output: "rec_lint.schema.md",
        is_index: true,
    },
    SchemaConfig {
        input: "schema/parts/rec_lint_config.schema.json",
        output: "rec_lint_config.schema.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/common.schema.json",
        output: "rules/common.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/guideline.schema.json",
        output: "rules/guideline.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/forbidden-texts.schema.json",
        output: "rules/forbidden-texts.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/forbidden-patterns.schema.json",
        output: "rules/forbidden-patterns.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/custom.schema.json",
        output: "rules/custom.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-php-doc.schema.json",
        output: "rules/require-php-doc.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-kotlin-doc.schema.json",
        output: "rules/require-kotlin-doc.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-rust-doc.schema.json",
        output: "rules/require-rust-doc.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-english-comment.schema.json",
        output: "rules/require-english-comment.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-japanese-comment.schema.json",
        output: "rules/require-japanese-comment.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-japanese-phpunit-test-name.schema.json",
        output: "rules/require-japanese-phpunit-test-name.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-japanese-kotest-test-name.schema.json",
        output: "rules/require-japanese-kotest-test-name.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-japanese-rust-test-name.schema.json",
        output: "rules/require-japanese-rust-test-name.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-phpunit-test.schema.json",
        output: "rules/require-phpunit-test.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-kotest-test.schema.json",
        output: "rules/require-kotest-test.md",
        is_index: false,
    },
    SchemaConfig {
        input: "schema/parts/rules/require-rust-unit-test.schema.json",
        output: "rules/require-rust-unit-test.md",
        is_index: false,
    },
];

struct RuleTypeInfo {
    type_name: &'static str,
    description: &'static str,
    doc_path: &'static str,
}

const RULE_TYPES: &[RuleTypeInfo] = &[
    RuleTypeInfo {
        type_name: "forbidden_texts",
        description: "禁止キーワードを完全一致で検出",
        doc_path: "./rules/forbidden-texts.md",
    },
    RuleTypeInfo {
        type_name: "forbidden_patterns",
        description: "禁止パターンを正規表現で検出",
        doc_path: "./rules/forbidden-patterns.md",
    },
    RuleTypeInfo {
        type_name: "custom",
        description: "任意のコマンドを実行して検証",
        doc_path: "./rules/custom.md",
    },
    RuleTypeInfo {
        type_name: "require_php_doc",
        description: "PHPDoc がないファイルを検出",
        doc_path: "./rules/require-php-doc.md",
    },
    RuleTypeInfo {
        type_name: "require_kotlin_doc",
        description: "KDoc がないファイルを検出",
        doc_path: "./rules/require-kotlin-doc.md",
    },
    RuleTypeInfo {
        type_name: "require_rust_doc",
        description: "rustdoc がないファイルを検出",
        doc_path: "./rules/require-rust-doc.md",
    },
    RuleTypeInfo {
        type_name: "require_english_comment",
        description: "コメントが日本語のファイルを検出",
        doc_path: "./rules/require-english-comment.md",
    },
    RuleTypeInfo {
        type_name: "require_japanese_comment",
        description: "コメントが英語のファイルを検出",
        doc_path: "./rules/require-japanese-comment.md",
    },
    RuleTypeInfo {
        type_name: "require_japanese_phpunit_test_name",
        description: "PHPUnit テスト名が日本語でないファイルを検出",
        doc_path: "./rules/require-japanese-phpunit-test-name.md",
    },
    RuleTypeInfo {
        type_name: "require_japanese_kotest_test_name",
        description: "Kotest テスト名が日本語でないファイルを検出",
        doc_path: "./rules/require-japanese-kotest-test-name.md",
    },
    RuleTypeInfo {
        type_name: "require_japanese_rust_test_name",
        description: "Rust テスト名が日本語でないファイルを検出",
        doc_path: "./rules/require-japanese-rust-test-name.md",
    },
    RuleTypeInfo {
        type_name: "require_phpunit_test",
        description: "PHPUnit テストファイルの存在を検証",
        doc_path: "./rules/require-phpunit-test.md",
    },
    RuleTypeInfo {
        type_name: "require_kotest_test",
        description: "Kotest テストファイルの存在を検証",
        doc_path: "./rules/require-kotest-test.md",
    },
    RuleTypeInfo {
        type_name: "require_rust_unit_test",
        description: "Rust ユニットテストの存在を検証",
        doc_path: "./rules/require-rust-unit-test.md",
    },
];

struct SchemaSet {
    schemas: BTreeMap<PathBuf, Value>,
    base_path: PathBuf,
}

impl SchemaSet {
    fn new(repo_root: &Path) -> Self {
        Self {
            schemas: BTreeMap::new(),
            base_path: repo_root.to_path_buf(),
        }
    }

    fn load(&mut self, relative_path: &str) -> Result<()> {
        let path = self.base_path.join(relative_path);
        let json_str = fs::read_to_string(&path)
            .with_context(|| format!("スキーマファイルの読み込みに失敗: {}", path.display()))?;
        let schema: Value = serde_json::from_str(&json_str)
            .with_context(|| format!("JSONパースに失敗: {}", path.display()))?;
        self.schemas.insert(PathBuf::from(relative_path), schema);
        Ok(())
    }

    fn get(&self, relative_path: &str) -> Option<&Value> {
        self.schemas.get(&PathBuf::from(relative_path))
    }

    fn resolve_ref(&self, ref_path: &str, current_schema_path: &str) -> Option<&Value> {
        if ref_path.starts_with('#') {
            let def_path = ref_path.trim_start_matches("#/definitions/");
            self.get(current_schema_path)
                .and_then(|s| s.get("definitions"))
                .and_then(|d| d.get(def_path))
        } else if ref_path.contains('#') {
            let parts: Vec<&str> = ref_path.splitn(2, '#').collect();
            let file_path = parts[0];
            let def_path = parts[1].trim_start_matches("/definitions/");

            let current_dir = Path::new(current_schema_path).parent().unwrap_or(Path::new(""));
            let resolved_path = current_dir.join(file_path);
            let normalized = normalize_path(&resolved_path);

            self.schemas
                .get(&normalized)
                .and_then(|s| s.get("definitions"))
                .and_then(|d| d.get(def_path))
        } else {
            None
        }
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                result.pop();
            }
            std::path::Component::Normal(c) => {
                result.push(c);
            }
            std::path::Component::CurDir => {}
            _ => {
                result.push(component);
            }
        }
    }
    result
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("doc");

    match command {
        "bundle" => run_bundle(),
        "doc" | _ => run_doc(),
    }
}

fn run_doc() -> Result<()> {
    let repo_root = get_repo_root()?;
    let output_dir = repo_root.join(OUTPUT_DIR);
    let rules_output_dir = repo_root.join(RULES_OUTPUT_DIR);
    fs::create_dir_all(&output_dir)?;
    fs::create_dir_all(&rules_output_dir)?;

    let mut schema_set = SchemaSet::new(&repo_root);

    for config in SCHEMAS {
        schema_set.load(config.input)?;
        println!("Loaded: {}", config.input);
    }

    for config in SCHEMAS {
        let schema = schema_set.get(config.input).unwrap();
        let md = if config.is_index {
            render_index_schema(schema, &schema_set, config.input)
        } else if config.input.contains("rec_lint_config") {
            render_schema(schema, &schema_set, config.input, false)
        } else {
            render_rule_schema(schema, &schema_set, config.input)
        };

        let output_path = output_dir.join(config.output);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&output_path, &md)?;
        println!("Generated: {}", output_path.display());
    }

    Ok(())
}

fn run_bundle() -> Result<()> {
    let repo_root = get_repo_root()?;

    // Bundle rec_lint.schema.json
    bundle_schema(
        &repo_root,
        "schema/parts/rec_lint.schema.json",
        "schema/rec_lint.schema.json",
    )?;

    // Bundle rec_lint_config.schema.json
    bundle_schema(
        &repo_root,
        "schema/parts/rec_lint_config.schema.json",
        "schema/rec_lint_config.schema.json",
    )?;

    Ok(())
}

fn bundle_schema(repo_root: &Path, input: &str, output: &str) -> Result<()> {
    println!("Bundling: {} -> {}", input, output);

    let input_path = repo_root.join(input);
    let input_canonical = input_path.canonicalize()?;
    let output_path = repo_root.join(output);

    // Load all schema files
    let mut file_cache: BTreeMap<PathBuf, Value> = BTreeMap::new();
    load_schema_recursive(&input_path, &mut file_cache)?;

    // Get the main schema
    let main_schema = file_cache.get(&input_canonical).unwrap().clone();

    // Collect all definitions from all files, resolving refs as we go
    let mut all_definitions: Map<String, Value> = Map::new();
    for (path, schema) in &file_cache {
        if let Some(defs) = schema.get("definitions").and_then(|d| d.as_object()) {
            let prefix = get_definition_prefix(path, &input_canonical);
            for (name, def) in defs {
                let full_name = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{}_{}", prefix, name)
                };
                // Resolve refs in this definition
                let mut resolved_def = def.clone();
                resolve_refs_in_value(&mut resolved_def, path, &input_canonical);
                all_definitions.insert(full_name, resolved_def);
            }
        }
    }

    // Resolve all $ref in the main schema (excluding definitions which we handle separately)
    let mut bundled = main_schema.clone();
    resolve_refs_in_value(&mut bundled, &input_canonical, &input_canonical);

    // Update definitions with resolved ones
    if let Some(obj) = bundled.as_object_mut() {
        obj.insert("definitions".to_string(), json!(all_definitions));
    }

    // Write output
    let output_str = serde_json::to_string_pretty(&bundled)?;
    fs::write(&output_path, output_str)?;
    println!("Bundled: {}", output_path.display());

    Ok(())
}

fn load_schema_recursive(path: &Path, cache: &mut BTreeMap<PathBuf, Value>) -> Result<()> {
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    if cache.contains_key(&canonical) {
        return Ok(());
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read: {}", path.display()))?;
    let schema: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse: {}", path.display()))?;

    cache.insert(canonical.clone(), schema.clone());

    // Find all $ref and load referenced files
    let refs = collect_refs(&schema);
    let parent = path.parent().unwrap_or(Path::new("."));

    for ref_path in refs {
        if ref_path.starts_with('#') {
            continue; // Same file reference
        }
        if let Some(file_part) = ref_path.split('#').next() {
            if !file_part.is_empty() {
                let referenced_path = parent.join(file_part);
                load_schema_recursive(&referenced_path, cache)?;
            }
        }
    }

    Ok(())
}

fn collect_refs(value: &Value) -> Vec<String> {
    let mut refs = Vec::new();
    collect_refs_recursive(value, &mut refs);
    refs
}

fn collect_refs_recursive(value: &Value, refs: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(ref_str)) = map.get("$ref") {
                refs.push(ref_str.clone());
            }
            for v in map.values() {
                collect_refs_recursive(v, refs);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_refs_recursive(v, refs);
            }
        }
        _ => {}
    }
}

fn get_definition_prefix(file_path: &Path, main_path: &Path) -> String {
    if file_path == main_path {
        return String::new();
    }

    let file_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    // Remove .schema suffix if present
    let name = file_name.trim_end_matches(".schema");

    // Convert to snake_case identifier
    name.replace('-', "_")
}

fn resolve_refs_in_value(value: &mut Value, current_file: &Path, main_file: &Path) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(ref_str)) = map.get("$ref").cloned() {
                let new_ref = resolve_ref_string(&ref_str, current_file, main_file);
                map.insert("$ref".to_string(), json!(new_ref));
            }
            for v in map.values_mut() {
                resolve_refs_in_value(v, current_file, main_file);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                resolve_refs_in_value(v, current_file, main_file);
            }
        }
        _ => {}
    }
}

fn resolve_ref_string(ref_str: &str, current_file: &Path, main_file: &Path) -> String {
    if ref_str.starts_with('#') {
        // Same file reference - check if we need to add prefix
        if current_file == main_file {
            return ref_str.to_string();
        }
        let def_name = ref_str.trim_start_matches("#/definitions/");
        let prefix = get_definition_prefix(current_file, main_file);
        if prefix.is_empty() {
            ref_str.to_string()
        } else {
            format!("#/definitions/{}_{}", prefix, def_name)
        }
    } else if ref_str.contains('#') {
        // External file reference
        let parts: Vec<&str> = ref_str.splitn(2, '#').collect();
        let file_part = parts[0];
        let def_part = parts.get(1).unwrap_or(&"");
        let def_name = def_part.trim_start_matches("/definitions/");

        let parent = current_file.parent().unwrap_or(Path::new("."));
        let referenced_path = parent.join(file_part);
        let canonical = referenced_path.canonicalize().unwrap_or(referenced_path);

        let prefix = get_definition_prefix(&canonical, main_file);
        if prefix.is_empty() {
            format!("#/definitions/{}", def_name)
        } else {
            format!("#/definitions/{}_{}", prefix, def_name)
        }
    } else {
        ref_str.to_string()
    }
}

fn get_repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("git rev-parse の実行に失敗")?;

    if !output.status.success() {
        anyhow::bail!("git リポジトリ内ではありません");
    }

    let root = String::from_utf8(output.stdout)
        .context("git 出力が不正な UTF-8")?
        .trim()
        .to_string();

    Ok(PathBuf::from(root))
}

fn render_index_schema(schema: &Value, schema_set: &SchemaSet, current_path: &str) -> String {
    let mut out = String::new();

    if let Some(title) = schema.get("title").and_then(|v| v.as_str()) {
        writeln!(out, "# {}\n", title).unwrap();
    }

    writeln!(out, "## トップレベル\n").unwrap();
    if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
        writeln!(out, "| フィールド | 型 | 必須 | 説明 |").unwrap();
        writeln!(out, "|-----------|-----|:---:|------|").unwrap();

        let mut sorted_props: Vec<_> = props.iter().collect();
        sorted_props.sort_by(|(_, a), (_, b)| {
            let a_order = a
                .get("x-property-order")
                .and_then(|v| v.as_i64())
                .unwrap_or(i64::MAX);
            let b_order = b
                .get("x-property-order")
                .and_then(|v| v.as_i64())
                .unwrap_or(i64::MAX);
            a_order.cmp(&b_order)
        });

        for (name, prop) in sorted_props {
            let type_str = if name == "rule" {
                "[RuleItem](#rule-types)[]".to_string()
            } else {
                get_type_string_with_link_and_target(prop, schema_set, current_path, current_path)
            };
            let desc = get_doc_description(prop).unwrap_or("");
            writeln!(out, "| {} | {} | - | {} |", name, type_str, desc).unwrap();
        }
        writeln!(out).unwrap();
    }

    writeln!(out, "## Rule Types\n").unwrap();
    writeln!(out, "| type | 説明 | ドキュメント |").unwrap();
    writeln!(out, "|------|------|--------------|").unwrap();
    for rule_type in RULE_TYPES {
        writeln!(
            out,
            "| `{}` | {} | [詳細]({}) |",
            rule_type.type_name, rule_type.description, rule_type.doc_path
        )
        .unwrap();
    }
    writeln!(out).unwrap();

    writeln!(out, "## 共通定義\n").unwrap();
    writeln!(out, "[共通定義ドキュメント](./rules/common.md) を参照\n").unwrap();
    writeln!(
        out,
        "- [RuleBase](./rules/common.md#rulebase) - ルールの共通フィールド"
    )
    .unwrap();
    writeln!(
        out,
        "- [MatchItem](./rules/common.md#matchitem) - ファイルマッチ条件"
    )
    .unwrap();
    writeln!(
        out,
        "- [MatchPattern](./rules/common.md#matchpattern) - マッチパターンの種類"
    )
    .unwrap();
    writeln!(
        out,
        "- [MatchCond](./rules/common.md#matchcond) - keywords の結合条件"
    )
    .unwrap();
    writeln!(
        out,
        "- [Visibility](./rules/common.md#visibility) - Doc コメントを強制する対象の可視性"
    )
    .unwrap();
    writeln!(
        out,
        "- [TestRequireLevelExternalFile](./rules/common.md#testrequirelevelexternalfile) - テスト存在検証レベル (外部ファイル)"
    )
    .unwrap();
    writeln!(
        out,
        "- [TestRequireLevelSameFile](./rules/common.md#testrequirelevelsamefile) - テスト存在検証レベル (同一ファイル)"
    )
    .unwrap();

    out
}

fn render_rule_schema(schema: &Value, schema_set: &SchemaSet, current_path: &str) -> String {
    let mut out = String::new();

    let definitions = schema.get("definitions").and_then(|v| v.as_object());
    let is_common = current_path.contains("common.schema.json");

    if let Some(defs) = definitions {
        let mut sorted_defs: Vec<_> = defs.iter().collect();
        sorted_defs.sort_by(|(_, a), (_, b)| {
            let a_order = a
                .get("x-doc-order")
                .and_then(|v| v.as_i64())
                .unwrap_or(i64::MAX);
            let b_order = b
                .get("x-doc-order")
                .and_then(|v| v.as_i64())
                .unwrap_or(i64::MAX);
            a_order.cmp(&b_order)
        });

        let mut is_first = true;
        for (name, def) in sorted_defs {
            if is_first {
                if is_common {
                    writeln!(out, "# 共通定義\n").unwrap();
                    writeln!(
                        out,
                        "[← トップに戻る](../rec_lint.schema.md)\n"
                    )
                    .unwrap();
                } else {
                    let type_const = extract_type_const(def, schema_set, current_path);
                    if let Some(type_name) = type_const {
                        writeln!(out, "# {}\n", type_name).unwrap();
                    } else {
                        let title = def.get("title").and_then(|v| v.as_str()).unwrap_or(name);
                        writeln!(out, "# {}\n", title).unwrap();
                    }
                    writeln!(
                        out,
                        "[← ルール一覧に戻る](../rec_lint.schema.md#rule-types)\n"
                    )
                    .unwrap();
                    if let Some(desc) = get_doc_description(def) {
                        writeln!(out, "{}\n", desc).unwrap();
                    }
                }
                is_first = false;
            }
            render_definition(&mut out, name, def, schema_set, current_path);
        }
    }

    out
}

fn extract_type_const(def: &Value, schema_set: &SchemaSet, current_path: &str) -> Option<String> {
    if let Some(all_of) = def.get("allOf").and_then(|v| v.as_array()) {
        for item in all_of {
            if let Some(props) = item.get("properties").and_then(|v| v.as_object()) {
                if let Some(type_prop) = props.get("type") {
                    if let Some(const_val) = type_prop.get("const").and_then(|v| v.as_str()) {
                        return Some(const_val.to_string());
                    }
                }
            }
            if let Some(ref_path) = item.get("$ref").and_then(|v| v.as_str()) {
                if let Some(resolved) = schema_set.resolve_ref(ref_path, current_path) {
                    if let Some(result) = extract_type_const(resolved, schema_set, current_path) {
                        return Some(result);
                    }
                }
            }
        }
    }
    None
}

fn render_schema(
    schema: &Value,
    schema_set: &SchemaSet,
    current_path: &str,
    _is_rule: bool,
) -> String {
    let mut out = String::new();
    let definitions = schema.get("definitions").and_then(|v| v.as_object());

    if let Some(title) = schema.get("title").and_then(|v| v.as_str()) {
        writeln!(out, "# {}\n", title).unwrap();
    }
    if let Some(desc) = get_doc_description(schema) {
        writeln!(out, "{}\n", desc).unwrap();
    }

    if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
        writeln!(out, "## トップレベル\n").unwrap();
        render_properties_table(&mut out, props, &[], schema_set, current_path);
    }

    if let Some(defs) = definitions {
        let mut sorted_defs: Vec<_> = defs.iter().collect();
        sorted_defs.sort_by(|(_, a), (_, b)| {
            let a_order = a
                .get("x-doc-order")
                .and_then(|v| v.as_i64())
                .unwrap_or(i64::MAX);
            let b_order = b
                .get("x-doc-order")
                .and_then(|v| v.as_i64())
                .unwrap_or(i64::MAX);
            a_order.cmp(&b_order)
        });

        for (name, def) in sorted_defs {
            if name == "ruleBase" {
                continue;
            }
            render_definition(&mut out, name, def, schema_set, current_path);
        }
    }

    out
}

fn render_definition(
    out: &mut String,
    name: &str,
    def: &Value,
    schema_set: &SchemaSet,
    current_path: &str,
) {
    let title = def.get("title").and_then(|v| v.as_str()).unwrap_or(name);

    writeln!(out, "## {}\n", title).unwrap();

    if let Some(desc) = get_doc_description(def) {
        writeln!(out, "{}\n", desc).unwrap();
    }

    if let Some(one_of) = def.get("oneOf").and_then(|v| v.as_array()) {
        let is_string_enum = def.get("type").and_then(|v| v.as_str()) == Some("string")
            || one_of.iter().all(|v| v.get("const").is_some());

        if is_string_enum {
            render_enum_table(out, one_of);
            return;
        }

        for variant in one_of {
            render_one_of_variant(out, variant, schema_set, current_path);
        }
        return;
    }

    if def.get("allOf").is_some() {
        let (merged, required) = merge_all_of(def, schema_set, current_path);
        writeln!(out, "| フィールド | 型 | 必須 | 説明 |").unwrap();
        writeln!(out, "|-----------|-----|:---:|------|").unwrap();

        let mut sorted_fields: Vec<_> = merged.iter().collect();
        sorted_fields.sort_by(|(a, (a_prop, _)), (b, (b_prop, _))| {
            let a_order = a_prop
                .get("x-property-order")
                .and_then(|v| v.as_i64())
                .unwrap_or(i64::MAX);
            let b_order = b_prop
                .get("x-property-order")
                .and_then(|v| v.as_i64())
                .unwrap_or(i64::MAX);
            a_order.cmp(&b_order).then_with(|| a.cmp(b))
        });

        for (field_name, (prop, source_path)) in sorted_fields {
            let type_str = get_type_string_with_link_and_target(prop, schema_set, &source_path, current_path);
            let is_required = if required.iter().any(|r| r == field_name) {
                "o"
            } else {
                "-"
            };
            let desc = get_description_with_examples(prop);
            writeln!(out, "| {} | {} | {} | {} |", field_name, type_str, is_required, desc).unwrap();
        }
        writeln!(out).unwrap();
        return;
    }

    if let Some(props) = def.get("properties").and_then(|v| v.as_object()) {
        let required = get_required_fields(def);
        render_properties_table(out, props, &required, schema_set, current_path);
    }
}

fn render_one_of_variant(
    out: &mut String,
    variant: &Value,
    schema_set: &SchemaSet,
    current_path: &str,
) {
    let title = variant
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Variant");
    writeln!(out, "### {}\n", title).unwrap();

    if let Some(desc) = get_doc_description(variant) {
        writeln!(out, "{}\n", desc).unwrap();
    }

    let (merged, required) = merge_all_of(variant, schema_set, current_path);

    writeln!(out, "| フィールド | 型 | 必須 | 説明 |").unwrap();
    writeln!(out, "|-----------|-----|:---:|------|").unwrap();

    let mut sorted_fields: Vec<_> = merged.iter().collect();
    sorted_fields.sort_by(|(a, (a_prop, _)), (b, (b_prop, _))| {
        let a_order = a_prop
            .get("x-property-order")
            .and_then(|v| v.as_i64())
            .unwrap_or(i64::MAX);
        let b_order = b_prop
            .get("x-property-order")
            .and_then(|v| v.as_i64())
            .unwrap_or(i64::MAX);
        a_order.cmp(&b_order).then_with(|| a.cmp(b))
    });

    for (name, (prop, source_path)) in sorted_fields {
        let type_str = get_type_string_with_link_and_target(prop, schema_set, &source_path, current_path);
        let is_required = if required.iter().any(|r| r == name) {
            "o"
        } else {
            "-"
        };
        let desc = get_description_with_examples(prop);
        writeln!(out, "| {} | {} | {} | {} |", name, type_str, is_required, desc).unwrap();
    }
    writeln!(out).unwrap();
}

fn merge_all_of(
    variant: &Value,
    schema_set: &SchemaSet,
    current_path: &str,
) -> (BTreeMap<String, (Value, String)>, Vec<String>) {
    let mut merged: BTreeMap<String, (Value, String)> = BTreeMap::new();
    let mut required: Vec<String> = Vec::new();

    if let Some(all_of) = variant.get("allOf").and_then(|v| v.as_array()) {
        for schema in all_of {
            if let Some(ref_path) = schema.get("$ref").and_then(|v| v.as_str()) {
                let ref_source_path = resolve_ref_source_path(ref_path, current_path);
                if let Some(resolved) = schema_set.resolve_ref(ref_path, current_path) {
                    if let Some(props) = resolved.get("properties").and_then(|v| v.as_object()) {
                        for (k, v) in props {
                            merged.insert(k.clone(), (v.clone(), ref_source_path.clone()));
                        }
                    }
                }
            }
            if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
                for (k, v) in props {
                    merged.insert(k.clone(), (v.clone(), current_path.to_string()));
                }
            }
            if let Some(req) = schema.get("required").and_then(|v| v.as_array()) {
                for r in req {
                    if let Some(s) = r.as_str() {
                        if !required.contains(&s.to_string()) {
                            required.push(s.to_string());
                        }
                    }
                }
            }
        }
    }

    (merged, required)
}

fn resolve_ref_source_path(ref_path: &str, current_path: &str) -> String {
    if ref_path.starts_with('#') {
        current_path.to_string()
    } else if ref_path.contains('#') {
        let parts: Vec<&str> = ref_path.splitn(2, '#').collect();
        let file_path = parts[0];
        let current_dir = Path::new(current_path).parent().unwrap_or(Path::new(""));
        let resolved_path = current_dir.join(file_path);
        normalize_path(&resolved_path).to_string_lossy().to_string()
    } else {
        current_path.to_string()
    }
}

fn get_required_fields(def: &Value) -> Vec<&str> {
    def.get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default()
}

fn render_properties_table(
    out: &mut String,
    props: &serde_json::Map<String, Value>,
    required: &[&str],
    schema_set: &SchemaSet,
    current_path: &str,
) {
    writeln!(out, "| フィールド | 型 | 必須 | 説明 |").unwrap();
    writeln!(out, "|-----------|-----|:---:|------|").unwrap();

    let mut sorted_props: Vec<_> = props.iter().collect();
    sorted_props.sort_by(|(a, a_prop), (b, b_prop)| {
        let a_order = a_prop
            .get("x-property-order")
            .and_then(|v| v.as_i64())
            .unwrap_or(i64::MAX);
        let b_order = b_prop
            .get("x-property-order")
            .and_then(|v| v.as_i64())
            .unwrap_or(i64::MAX);
        a_order.cmp(&b_order).then_with(|| a.cmp(b))
    });

    for (name, prop) in sorted_props {
        let type_str = get_type_string_with_link_and_target(prop, schema_set, current_path, current_path);
        let is_required = if required.contains(&name.as_str()) {
            "o"
        } else {
            "-"
        };
        let desc = get_description_with_examples(prop);

        writeln!(out, "| {} | {} | {} | {} |", name, type_str, is_required, desc).unwrap();
    }
    writeln!(out).unwrap();
}

fn render_enum_table(out: &mut String, one_of: &[Value]) {
    writeln!(out, "| 値 | 説明 |").unwrap();
    writeln!(out, "|----|------|").unwrap();

    for item in one_of {
        let const_val = item.get("const").and_then(|v| v.as_str()).unwrap_or("");
        let desc = get_doc_description(item).unwrap_or("");
        writeln!(out, "| `{}` | {} |", const_val, desc).unwrap();
    }
    writeln!(out).unwrap();
}

fn get_doc_description(obj: &Value) -> Option<&str> {
    obj.get("x-doc-description")
        .and_then(|v| v.as_str())
        .or_else(|| obj.get("description").and_then(|v| v.as_str()))
}

fn get_description_with_examples(prop: &Value) -> String {
    let desc = get_doc_description(prop).unwrap_or("");

    let example_str = prop
        .get("examples")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .flat_map(|ex| {
                    if let Some(s) = ex.as_str() {
                        vec![format!("<br>e.g. `{}`", s)]
                    } else if let Some(inner_arr) = ex.as_array() {
                        inner_arr
                            .iter()
                            .filter_map(|item| item.as_str())
                            .map(|s| format!("<br>e.g. `{}`", s))
                            .collect()
                    } else {
                        vec![format!(
                            "<br>e.g. `{}`",
                            serde_json::to_string(ex).unwrap_or_default()
                        )]
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default();

    if example_str.is_empty() {
        desc.to_string()
    } else {
        format!("{}{}", desc, example_str)
    }
}

fn get_type_string_with_link_and_target(
    prop: &Value,
    schema_set: &SchemaSet,
    source_path: &str,
    target_doc_path: &str,
) -> String {
    if let Some(const_val) = prop.get("const").and_then(|v| v.as_str()) {
        return format!("`{}`", const_val);
    }

    if let Some(ref_path) = prop.get("$ref").and_then(|v| v.as_str()) {
        return ref_to_markdown_link_with_target(ref_path, source_path, target_doc_path);
    }

    if let Some(one_of) = prop.get("oneOf").and_then(|v| v.as_array()) {
        let values: Vec<&str> = one_of
            .iter()
            .filter_map(|v| v.get("const").and_then(|c| c.as_str()))
            .collect();
        if !values.is_empty() {
            return format!("`{}`", values.join("` \\|<br>`"));
        }
    }

    if let Some(enum_vals) = prop.get("enum").and_then(|v| v.as_array()) {
        let values: Vec<&str> = enum_vals.iter().filter_map(|v| v.as_str()).collect();
        return format!("`{}`", values.join("` \\|<br>`"));
    }

    if let Some(type_val) = prop.get("type").and_then(|v| v.as_str()) {
        if type_val == "array" {
            if let Some(items) = prop.get("items") {
                let item_type = get_type_string_with_link_and_target(items, schema_set, source_path, target_doc_path);
                return format!("{}[]", item_type);
            }
            return "array".to_string();
        }
        return type_val.to_string();
    }

    "any".to_string()
}

fn ref_to_markdown_link_with_target(ref_path: &str, source_path: &str, target_doc_path: &str) -> String {
    if ref_path.starts_with('#') {
        let def_name = ref_path.trim_start_matches("#/definitions/");

        let source_md = schema_path_to_md_path(source_path);
        let target_md = schema_path_to_md_path(target_doc_path);

        if source_md == target_md {
            return format!("[{}](#{})", def_name, def_name.to_lowercase());
        } else {
            let relative_link = get_relative_md_link(&target_md, &source_md);
            return format!("[{}]({}#{})", def_name, relative_link, def_name.to_lowercase());
        }
    }

    if ref_path.contains('#') {
        let parts: Vec<&str> = ref_path.splitn(2, '#').collect();
        let file_path = parts[0];
        let def_name = parts[1].trim_start_matches("/definitions/");

        let current_dir = Path::new(source_path).parent().unwrap_or(Path::new(""));
        let resolved_path = current_dir.join(file_path);
        let normalized = normalize_path(&resolved_path);

        let resolved_md = schema_path_to_md_path(&normalized.to_string_lossy());
        let target_md = schema_path_to_md_path(target_doc_path);
        let relative_link = get_relative_md_link(&target_md, &resolved_md);

        return format!("[{}]({}#{})", def_name, relative_link, def_name.to_lowercase());
    }

    ref_path.to_string()
}

fn schema_path_to_md_path(schema_path: &str) -> String {
    let path = schema_path
        .replace("schema/parts/rules/", "docs/schema/rules/")
        .replace("schema/parts/", "docs/schema/")
        .replace(".schema.json", ".md");

    if path.ends_with(".json") {
        path.replace(".json", ".md")
    } else {
        path
    }
}

fn get_relative_md_link(from_md: &str, to_md: &str) -> String {
    let to_path = Path::new(to_md);

    let from_in_rules = from_md.contains("rules/");
    let to_in_rules = to_md.contains("rules/");

    let to_filename = to_path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    if from_in_rules && to_in_rules {
        format!("./{}", to_filename)
    } else if from_in_rules && !to_in_rules {
        format!("../{}", to_filename)
    } else if !from_in_rules && to_in_rules {
        format!("./rules/{}", to_filename)
    } else {
        format!("./{}", to_filename)
    }
}
