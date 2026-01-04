use super::custom::{extract_comments as extract_custom, BlockSyntax, CustomCommentSyntax};
use super::Comment;

/// Extract all comments from Rust source code
pub fn extract_comments(content: &str) -> Vec<Comment> {
    let syntax = CustomCommentSyntax {
        lines: vec!["//".to_string()],
        blocks: vec![BlockSyntax { start: "/*".to_string(), end: "*/".to_string() }],
    };
    extract_custom(content, &syntax)
        .into_iter()
        // Skip doc comments (/// and //!)
        .filter(|c| !c.text.starts_with('/') && !c.text.starts_with('!'))
        .collect()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 通常のコメントを抽出できる() {
        let content = "// This is a comment";
        let comments = extract_comments(content);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text, "This is a comment");
    }

    #[test]
    fn ドキュメントコメントはスキップされる() {
        let content = "/// This is a doc comment";
        let comments = extract_comments(content);
        assert!(comments.is_empty());
    }

    #[test]
    fn 複数行のドキュメントコメントはスキップされる() {
        let content = "/// foo\n///\n/// bar";
        let comments = extract_comments(content);
        assert!(comments.is_empty());
    }

    #[test]
    fn 内部ドキュメントコメントはスキップされる() {
        let content = "//! This is an inner doc comment";
        let comments = extract_comments(content);
        assert!(comments.is_empty());
    }

    #[test]
    fn 通常コメントとドキュメントコメントが混在する場合() {
        let content = "/// doc comment\n// normal comment\n//! inner doc";
        let comments = extract_comments(content);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text, "normal comment");
        assert_eq!(comments[0].line, 2);
    }
}
