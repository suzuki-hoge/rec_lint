use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Result};

use crate::rule::CustomRule;
use crate::validate::CustomViolation;

pub fn validate(file_path: &Path, rule: &CustomRule, script_dir: Option<&Path>) -> Result<Option<CustomViolation>> {
    let file_path_str = file_path.to_string_lossy();
    let mut exec_command = rule.exec.replace("{file}", file_path_str.as_ref());

    if exec_command.contains("{script_dir}") {
        let script_dir = script_dir
            .ok_or_else(|| anyhow!("{{script_dir}} placeholder requires script_dir in .rec_lint_config.yaml"))?;
        let script_dir_str = script_dir.to_string_lossy();
        exec_command = exec_command.replace("{script_dir}", script_dir_str.as_ref());
    }

    let parts: Vec<&str> = exec_command.split_whitespace().collect();

    if parts.is_empty() {
        return Ok(None);
    }

    let output = Command::new(parts[0]).args(&parts[1..]).output()?;

    if output.status.success() {
        Ok(None)
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = if stdout.is_empty() {
            stderr.to_string()
        } else if stderr.is_empty() {
            stdout.to_string()
        } else {
            format!("{stdout}{stderr}")
        };
        Ok(Some(CustomViolation { output: combined.trim().to_string() }))
    }
}
