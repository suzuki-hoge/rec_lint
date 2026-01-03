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
        .filter(|c| contains_japanese(&c.text))
        .map(|c| CommentViolation { line: c.line, text: c.text.clone() })
        .collect()
}

/// Validate comments for non-Japanese content (English/ASCII only)
pub fn validate_non_japanese(comments: &[Comment]) -> Vec<CommentViolation> {
    comments
        .iter()
        .filter(|c| {
            // Skip empty or whitespace-only comments
            let trimmed = c.text.trim();
            if trimmed.is_empty() {
                return false;
            }
            // Check if NOT Japanese (but has meaningful text)
            !contains_japanese(&c.text)
        })
        .map(|c| CommentViolation { line: c.line, text: c.text.clone() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // contains_japanese tests
    // =========================================================================

    #[test]
    fn test_contains_japanese_hiragana() {
        assert!(contains_japanese("これはテストです"));
    }

    #[test]
    fn test_contains_japanese_katakana() {
        assert!(contains_japanese("テスト"));
    }

    #[test]
    fn test_contains_japanese_kanji() {
        assert!(contains_japanese("日本語"));
    }

    #[test]
    fn test_contains_japanese_mixed() {
        assert!(contains_japanese("This is テスト"));
    }

    #[test]
    fn test_contains_japanese_ascii_only() {
        assert!(!contains_japanese("This is a test"));
    }

    #[test]
    fn test_contains_japanese_empty() {
        assert!(!contains_japanese(""));
    }

    #[test]
    fn test_contains_japanese_numbers() {
        assert!(!contains_japanese("12345"));
    }

    #[test]
    fn test_contains_japanese_halfwidth_katakana() {
        assert!(contains_japanese("ｱｲｳｴｵ"));
    }

    // =========================================================================
    // validate_japanese tests
    // =========================================================================

    #[test]
    fn test_validate_japanese_finds_japanese() {
        let comments = vec![
            Comment { line: 1, text: "これは日本語".to_string() },
            Comment { line: 2, text: "This is English".to_string() },
            Comment { line: 3, text: "Mixed テスト".to_string() },
        ];
        let violations = validate_japanese(&comments);
        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].line, 1);
        assert_eq!(violations[1].line, 3);
    }

    #[test]
    fn test_validate_japanese_no_japanese() {
        let comments = vec![
            Comment { line: 1, text: "English only".to_string() },
            Comment { line: 2, text: "More English".to_string() },
        ];
        let violations = validate_japanese(&comments);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // validate_non_japanese tests
    // =========================================================================

    #[test]
    fn test_validate_non_japanese_finds_english() {
        let comments = vec![
            Comment { line: 1, text: "これは日本語".to_string() },
            Comment { line: 2, text: "This is English".to_string() },
            Comment { line: 3, text: "Mixed テスト".to_string() },
        ];
        let violations = validate_non_japanese(&comments);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2);
    }

    #[test]
    fn test_validate_non_japanese_all_japanese() {
        let comments =
            vec![Comment { line: 1, text: "日本語のみ".to_string() }, Comment { line: 2, text: "テスト".to_string() }];
        let violations = validate_non_japanese(&comments);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_validate_non_japanese_skips_empty() {
        let comments = vec![Comment { line: 1, text: "   ".to_string() }, Comment { line: 2, text: "".to_string() }];
        let violations = validate_non_japanese(&comments);
        assert!(violations.is_empty());
    }
}
