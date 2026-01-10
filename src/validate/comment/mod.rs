pub mod custom;
pub mod java;
pub mod kotlin;
pub mod rust;

/// Extracted comment from source file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub line: usize,
    pub text: String,
}

/// A comment violation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentViolation {
    pub line: usize,
    pub text: String,
}

/// Check if comment is empty or just decoration (e.g., `*` in block comments)
fn is_empty_or_decoration(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return true;
    }
    // Block comment decoration: just `*` or `* ` pattern
    if trimmed == "*" {
        return true;
    }
    false
}

/// Check if text contains Japanese characters (Hiragana, Katakana, CJK)
pub fn contains_japanese(text: &str) -> bool {
    text.chars().any(|c| {
        let code = c as u32;
        // Hiragana: U+3040-U+309F
        // Katakana: U+30A0-U+30FF
        // CJK Unified Ideographs: U+4E00-U+9FFF
        // Katakana Phonetic Extensions: U+31F0-U+31FF
        // Halfwidth Katakana: U+FF65-U+FF9F
        (0x3040..=0x309F).contains(&code)
            || (0x30A0..=0x30FF).contains(&code)
            || (0x4E00..=0x9FFF).contains(&code)
            || (0x31F0..=0x31FF).contains(&code)
            || (0xFF65..=0xFF9F).contains(&code)
    })
}

/// Validate comments for Japanese content
pub fn validate_japanese(comments: &[Comment]) -> Vec<CommentViolation> {
    comments
        .iter()
        .filter(|c| {
            // Skip empty or decoration-only comments
            if is_empty_or_decoration(&c.text) {
                return false;
            }
            contains_japanese(&c.text)
        })
        .map(|c| CommentViolation { line: c.line, text: c.text.clone() })
        .collect()
}

/// Validate comments for non-Japanese content (English/ASCII only)
pub fn validate_non_japanese(comments: &[Comment]) -> Vec<CommentViolation> {
    comments
        .iter()
        .filter(|c| {
            // Skip empty or decoration-only comments
            if is_empty_or_decoration(&c.text) {
                return false;
            }
            // Check if NOT Japanese (but has meaningful text)
            !contains_japanese(&c.text)
        })
        .map(|c| CommentViolation { line: c.line, text: c.text.clone() })
        .collect()
}
