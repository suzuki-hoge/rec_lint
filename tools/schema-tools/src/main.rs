use anyhow::{Context, Result};
use jsonschema::Validator;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const OUTPUT_DIR: &str = "docs";

struct SchemaFile {
    input: &'static str,
    output: &'static str,
}

const SCHEMAS: &[SchemaFile] = &[
    SchemaFile { input: "schema/rec_lint.schema.json", output: "rec_lint.schema.md" },
    SchemaFile { input: "schema/rec_lint_config.schema.json", output: "rec_lint_config.schema.md" },
];

fn main() -> Result<()> {
    let repo_root = get_repo_root()?;
    let output_dir = repo_root.join(OUTPUT_DIR);
    fs::create_dir_all(&output_dir)?;

    for schema_file in SCHEMAS {
        let input_path = repo_root.join(schema_file.input);
        let json_str = fs::read_to_string(&input_path).context("Failed to read schema file")?;
        let schema: Value = serde_json::from_str(&json_str).context("Failed to parse JSON")?;

        Validator::new(&schema).context("Invalid JSON Schema")?;
        println!("Validated: {}", input_path.display());

        let md = render_schema(&schema);

        let output_path = output_dir.join(schema_file.output);
        fs::write(&output_path, &md)?;
        println!("Generated: {}", output_path.display());
    }

    Ok(())
}

fn get_repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("Failed to run git rev-parse")?;

    if !output.status.success() {
        anyhow::bail!("Not in a git repository");
    }

    let root = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    Ok(PathBuf::from(root))
}

fn render_schema(schema: &Value) -> String {
    let mut out = String::new();
    let definitions = schema.get("definitions").and_then(|v| v.as_object());

    // Title and description
    if let Some(title) = schema.get("title").and_then(|v| v.as_str()) {
        writeln!(out, "# {}\n", title).unwrap();
    }
    if let Some(desc) = get_doc_description(schema) {
        writeln!(out, "{}\n", desc).unwrap();
    }

    // Top-level properties
    if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
        writeln!(out, "## トップレベル\n").unwrap();
        render_properties_table(&mut out, props, &[], definitions);
    }

    // Definitions (sorted by x-doc-order)
    if let Some(defs) = definitions {
        let mut sorted_defs: Vec<_> = defs.iter().collect();
        sorted_defs.sort_by(|(_, a), (_, b)| {
            let a_order = a.get("x-doc-order").and_then(|v| v.as_i64()).unwrap_or(i64::MAX);
            let b_order = b.get("x-doc-order").and_then(|v| v.as_i64()).unwrap_or(i64::MAX);
            a_order.cmp(&b_order)
        });

        for (name, def) in sorted_defs {
            // Skip ruleBase (it's merged into each rule variant)
            if name == "ruleBase" {
                continue;
            }
            render_definition(&mut out, name, def, definitions);
        }
    }

    out
}

fn render_definition(
    out: &mut String,
    name: &str,
    def: &Value,
    definitions: Option<&serde_json::Map<String, Value>>,
) {
    let title = def.get("title").and_then(|v| v.as_str()).unwrap_or(name);

    writeln!(out, "## {}\n", title).unwrap();

    if let Some(desc) = get_doc_description(def) {
        writeln!(out, "{}\n", desc).unwrap();
    }

    // Handle oneOf
    if let Some(one_of) = def.get("oneOf").and_then(|v| v.as_array()) {
        // Check if this is a string enum (oneOf with const values)
        let is_string_enum = def.get("type").and_then(|v| v.as_str()) == Some("string")
            || one_of.iter().all(|v| v.get("const").is_some());

        if is_string_enum {
            render_enum_table(out, one_of);
            return;
        }

        // Otherwise it's a discriminated union (each variant has allOf)
        for variant in one_of {
            render_one_of_variant(out, variant, definitions);
        }
        return;
    }

    // For object types
    if let Some(props) = def.get("properties").and_then(|v| v.as_object()) {
        let required = get_required_fields(def);
        render_properties_table(out, props, &required, definitions);
    }
}

fn render_one_of_variant(
    out: &mut String,
    variant: &Value,
    definitions: Option<&serde_json::Map<String, Value>>,
) {
    let title = variant.get("title").and_then(|v| v.as_str()).unwrap_or("Variant");
    writeln!(out, "### {}\n", title).unwrap();

    if let Some(desc) = get_doc_description(variant) {
        writeln!(out, "{}\n", desc).unwrap();
    }

    // Merge allOf schemas
    let (merged, required) = merge_all_of(variant, definitions);

    writeln!(out, "| フィールド | 型 | 必須 | 説明 |").unwrap();
    writeln!(out, "|-----------|-----|:---:|------|").unwrap();

    // Sort fields by x-property-order
    let mut sorted_fields: Vec<_> = merged.iter().collect();
    sorted_fields.sort_by(|(a, a_prop), (b, b_prop)| {
        let a_order = a_prop.get("x-property-order").and_then(|v| v.as_i64()).unwrap_or(i64::MAX);
        let b_order = b_prop.get("x-property-order").and_then(|v| v.as_i64()).unwrap_or(i64::MAX);
        a_order.cmp(&b_order).then_with(|| a.cmp(b))
    });

    for (name, prop) in sorted_fields {
        let type_str = get_type_string(prop, definitions);
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
    definitions: Option<&serde_json::Map<String, Value>>,
) -> (BTreeMap<String, Value>, Vec<String>) {
    let mut merged: BTreeMap<String, Value> = BTreeMap::new();
    let mut required: Vec<String> = Vec::new();

    if let Some(all_of) = variant.get("allOf").and_then(|v| v.as_array()) {
        for schema in all_of {
            // Handle $ref
            if let Some(ref_path) = schema.get("$ref").and_then(|v| v.as_str()) {
                if let Some(resolved) = resolve_ref(ref_path, definitions) {
                    if let Some(props) = resolved.get("properties").and_then(|v| v.as_object()) {
                        for (k, v) in props {
                            merged.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
            // Handle inline properties
            if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
                for (k, v) in props {
                    merged.insert(k.clone(), v.clone());
                }
            }
            // Collect required fields
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

fn resolve_ref<'a>(
    ref_path: &str,
    definitions: Option<&'a serde_json::Map<String, Value>>,
) -> Option<&'a Value> {
    if ref_path.starts_with("#/definitions/") {
        let name = ref_path.trim_start_matches("#/definitions/");
        definitions.and_then(|defs| defs.get(name))
    } else {
        None
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
    definitions: Option<&serde_json::Map<String, Value>>,
) {
    writeln!(out, "| フィールド | 型 | 必須 | 説明 |").unwrap();
    writeln!(out, "|-----------|-----|:---:|------|").unwrap();

    // Sort fields by x-property-order
    let mut sorted_props: Vec<_> = props.iter().collect();
    sorted_props.sort_by(|(a, a_prop), (b, b_prop)| {
        let a_order = a_prop.get("x-property-order").and_then(|v| v.as_i64()).unwrap_or(i64::MAX);
        let b_order = b_prop.get("x-property-order").and_then(|v| v.as_i64()).unwrap_or(i64::MAX);
        a_order.cmp(&b_order).then_with(|| a.cmp(b))
    });

    for (name, prop) in sorted_props {
        let type_str = get_type_string(prop, definitions);
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

/// Get description for documentation (prefers x-doc-description over description)
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
                        // Handle nested array: expand each item
                        inner_arr
                            .iter()
                            .filter_map(|item| item.as_str())
                            .map(|s| format!("<br>e.g. `{}`", s))
                            .collect()
                    } else {
                        vec![format!("<br>e.g. `{}`", serde_json::to_string(ex).unwrap_or_default())]
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

fn get_type_string(prop: &Value, definitions: Option<&serde_json::Map<String, Value>>) -> String {
    // Check for const (discriminator value)
    if let Some(const_val) = prop.get("const").and_then(|v| v.as_str()) {
        return format!("`{}`", const_val);
    }

    // Check for $ref first
    if let Some(ref_path) = prop.get("$ref").and_then(|v| v.as_str()) {
        let type_name = ref_path.split('/').last().unwrap_or(ref_path);
        return format!("[{}](#{})", type_name, type_name.to_lowercase());
    }

    // Check for oneOf (enum with descriptions)
    if let Some(one_of) = prop.get("oneOf").and_then(|v| v.as_array()) {
        let values: Vec<&str> = one_of
            .iter()
            .filter_map(|v| v.get("const").and_then(|c| c.as_str()))
            .collect();
        if !values.is_empty() {
            return format!("`{}`", values.join("` \\|<br>`"));
        }
    }

    // Check for enum
    if let Some(enum_vals) = prop.get("enum").and_then(|v| v.as_array()) {
        let values: Vec<&str> = enum_vals.iter().filter_map(|v| v.as_str()).collect();
        return format!("`{}`", values.join("` \\|<br>`"));
    }

    // Check for type
    if let Some(type_val) = prop.get("type").and_then(|v| v.as_str()) {
        if type_val == "array" {
            if let Some(items) = prop.get("items") {
                let item_type = get_type_string(items, definitions);
                return format!("{}[]", item_type);
            }
            return "array".to_string();
        }
        return type_val.to_string();
    }

    "any".to_string()
}
