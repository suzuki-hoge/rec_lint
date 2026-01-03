use std::path::Path;
use std::process::Command;

use anyhow::Result;

use crate::rule::CustomRule;
use crate::validate::CustomViolation;

pub fn validate(file_path: &Path, rule: &CustomRule) -> Result<Option<CustomViolation>> {
    let exec_with_file = rule.exec.replace("{file}", &file_path.to_string_lossy());
    let parts: Vec<&str> = exec_with_file.split_whitespace().collect();

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
