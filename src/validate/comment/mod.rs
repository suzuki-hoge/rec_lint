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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    // =========================================================================
    // 日本語判定
    // =========================================================================

    #[test]
    fn ひらがなを含むテキストは日本語と判定される() {
        assert!(contains_japanese("これはテストです"));
    }

    #[test]
    fn カタカナを含むテキストは日本語と判定される() {
        assert!(contains_japanese("テスト"));
    }

    #[test]
    fn 漢字を含むテキストは日本語と判定される() {
        assert!(contains_japanese("日本語"));
    }

    #[test]
    fn 英語と日本語の混在テキストは日本語と判定される() {
        assert!(contains_japanese("This is テスト"));
    }

    #[test]
    fn ASCII文字のみのテキストは日本語と判定されない() {
        assert!(!contains_japanese("This is a test"));
    }

    #[test]
    fn 空文字列は日本語と判定されない() {
        assert!(!contains_japanese(""));
    }

    #[test]
    fn 数字のみのテキストは日本語と判定されない() {
        assert!(!contains_japanese("12345"));
    }

    #[test]
    fn 半角カタカナを含むテキストは日本語と判定される() {
        assert!(contains_japanese("ｱｲｳｴｵ"));
    }

    // =========================================================================
    // 日本語コメント検出
    // =========================================================================

    #[test]
    fn 日本語を含むコメントが違反として検出される() {
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
    fn 日本語がない場合は違反なし() {
        let comments = vec![
            Comment { line: 1, text: "English only".to_string() },
            Comment { line: 2, text: "More English".to_string() },
        ];
        let violations = validate_japanese(&comments);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // 非日本語コメント検出
    // =========================================================================

    #[test]
    fn 英語のみのコメントが違反として検出される() {
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
    fn 全て日本語の場合は違反なし() {
        let comments =
            vec![Comment { line: 1, text: "日本語のみ".to_string() }, Comment { line: 2, text: "テスト".to_string() }];
        let violations = validate_non_japanese(&comments);
        assert!(violations.is_empty());
    }

    #[test]
    fn 空白のみのコメントは違反として検出されない() {
        let comments = vec![Comment { line: 1, text: "   ".to_string() }, Comment { line: 2, text: "".to_string() }];
        let violations = validate_non_japanese(&comments);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // 空行・装飾のみのコメントはスキップ
    // =========================================================================

    #[test]
    fn ブロックコメントのアスタリスク装飾は日本語検査でスキップされる() {
        // /*
        //  * foo
        //  *
        //  */
        let comments =
            vec![Comment { line: 2, text: "* 日本語コメント".to_string() }, Comment { line: 3, text: "*".to_string() }];
        let violations = validate_japanese(&comments);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2);
    }

    #[test]
    fn ブロックコメントのアスタリスク装飾は英語検査でスキップされる() {
        let comments = vec![
            Comment { line: 2, text: "* English comment".to_string() },
            Comment { line: 3, text: "*".to_string() },
        ];
        let violations = validate_non_japanese(&comments);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2);
    }

    #[test]
    fn 空の行コメントは日本語検査でスキップされる() {
        // //
        // // foo
        // //
        let comments = vec![
            Comment { line: 1, text: "".to_string() },
            Comment { line: 2, text: "日本語".to_string() },
            Comment { line: 3, text: "".to_string() },
        ];
        let violations = validate_japanese(&comments);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2);
    }

    #[test]
    fn 空の行コメントは英語検査でスキップされる() {
        let comments = vec![
            Comment { line: 1, text: "".to_string() },
            Comment { line: 2, text: "English".to_string() },
            Comment { line: 3, text: "".to_string() },
        ];
        let violations = validate_non_japanese(&comments);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2);
    }
}
