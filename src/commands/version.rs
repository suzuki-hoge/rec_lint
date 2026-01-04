use anyhow::Result;

const VERSION: &str = include_str!("../../.version");

pub fn run() -> Result<Vec<String>> {
    Ok(vec![VERSION.trim().to_string()])
}
