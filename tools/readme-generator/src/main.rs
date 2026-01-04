use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};

const TEMPLATE_PATH: &str = "docs/README_TEMPLATE.md";
const OUTPUT_PATH: &str = "README.md";

fn main() {
    let repo_root = match get_repo_root() {
        Ok(root) => root,
        Err(e) => {
            eprintln!("Error getting repo root: {}", e);
            exit(1);
        }
    };

    let template_path = repo_root.join(TEMPLATE_PATH);
    let output_path = repo_root.join(OUTPUT_PATH);

    let template = match fs::read_to_string(&template_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading template file '{}': {}", template_path.display(), e);
            exit(1);
        }
    };

    let result = match process_template(&template, &repo_root) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error processing template: {}", e);
            exit(1);
        }
    };

    if let Err(e) = fs::write(&output_path, &result) {
        eprintln!("Error writing output file '{}': {}", output_path.display(), e);
        exit(1);
    }

    println!("Generated: {}", output_path.display());
}

fn get_repo_root() -> Result<PathBuf, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| format!("Failed to run git rev-parse: {}", e))?;

    if !output.status.success() {
        return Err("Not in a git repository".to_string());
    }

    let root = String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in git output: {}", e))?
        .trim()
        .to_string();

    Ok(PathBuf::from(root))
}

fn process_template(template: &str, repo_root: &PathBuf) -> Result<String, String> {
    let pattern = Regex::new(r"\{\{exec:\s*(.+?)\s*\}\}").unwrap();
    let mut result = template.to_string();
    let mut offset: i64 = 0;

    for cap in pattern.captures_iter(template) {
        let full_match = cap.get(0).unwrap();
        let command = cap.get(1).unwrap().as_str();

        let output = execute_command(command, repo_root)?;

        let start = (full_match.start() as i64 + offset) as usize;
        let end = (full_match.end() as i64 + offset) as usize;

        result.replace_range(start..end, &output);
        offset += output.len() as i64 - (full_match.end() - full_match.start()) as i64;
    }

    Ok(result)
}

fn execute_command(command: &str, repo_root: &PathBuf) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(repo_root)
        .output()
        .map_err(|e| format!("Failed to execute '{}': {}", command, e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command '{}' failed: {}", command, stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim_end().to_string())
}
