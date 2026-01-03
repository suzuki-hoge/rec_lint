use std::path::Path;
use std::process::Command;

use anyhow::Result;

use crate::rule::{CustomRule, RegexRule, TextRule};

#[derive(Debug)]
pub struct Violation {
    pub line: usize,
    pub col: usize,
    pub found: String,
}

#[derive(Debug)]
pub struct CustomViolation {
    pub output: String,
}

pub fn validate_text(content: &str, rule: &TextRule) -> Vec<Violation> {
    let mut violations = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        for keyword in &rule.keywords {
            if let Some(col) = line.find(keyword) {
                violations.push(Violation { line: line_num + 1, col: col + 1, found: line.to_string() });
                break;
            }
        }
    }
    violations
}

pub fn validate_regex(content: &str, rule: &RegexRule) -> Vec<Violation> {
    let mut violations = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        for pattern in &rule.patterns {
            if let Some(m) = pattern.find(line) {
                violations.push(Violation { line: line_num + 1, col: m.start() + 1, found: line.to_string() });
                break;
            }
        }
    }
    violations
}

pub fn validate_custom(file_path: &Path, rule: &CustomRule) -> Result<Option<CustomViolation>> {
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
