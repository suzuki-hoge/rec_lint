use super::custom::{extract_comments as extract_custom, BlockSyntax, CustomCommentSyntax};
use super::Comment;

/// Extract all comments from Kotlin source code
pub fn extract_comments(content: &str) -> Vec<Comment> {
    let syntax = CustomCommentSyntax {
        lines: vec!["//".to_string()],
        blocks: vec![BlockSyntax { start: "/*".to_string(), end: "*/".to_string() }],
    };
    extract_custom(content, &syntax)
}
