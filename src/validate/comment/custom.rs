use super::Comment;

/// Block comment syntax (start and end markers)
#[derive(Debug, Clone)]
pub struct BlockSyntax {
    pub start: String,
    pub end: String,
}

/// Custom comment syntax configuration
#[derive(Debug, Clone)]
pub struct CustomCommentSyntax {
    pub lines: Vec<String>,
    pub blocks: Vec<BlockSyntax>,
}

/// Extract all comments from source code using custom syntax
pub fn extract_comments(content: &str, syntax: &CustomCommentSyntax) -> Vec<Comment> {
    let mut comments = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    // State for block comment tracking
    let mut in_block_comment = false;
    let mut block_start_line = 0;
    let mut block_text = String::new();
    let mut current_block_end = String::new();

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        let line_num = i + 1;

        if in_block_comment {
            // Continue block comment
            if let Some(end_pos) = line.find(current_block_end.as_str()) {
                // End of block comment
                let before_end = &line[..end_pos];
                if !block_text.is_empty() {
                    block_text.push('\n');
                }
                block_text.push_str(before_end.trim());

                comments.push(Comment { line: block_start_line, text: block_text.clone() });

                in_block_comment = false;
                block_text.clear();

                // Check for line comment after block end
                let after_end = &line[end_pos + current_block_end.len()..];
                if let Some(comment) = find_line_comment(after_end, line_num, &syntax.lines) {
                    comments.push(comment);
                }
            } else {
                // Middle of block comment
                if !block_text.is_empty() {
                    block_text.push('\n');
                }
                block_text.push_str(trimmed);
            }
        } else {
            // Check for line comment
            if let Some(comment) = find_line_comment(line, line_num, &syntax.lines) {
                comments.push(comment);
            }

            // Check for block comments (may be multiple on one line)
            let mut remaining = line;
            while let Some((block, pos)) = find_block_start(remaining, &syntax.blocks) {
                let after = &remaining[pos + block.start.len()..];

                // Check for single-line block comment
                if let Some(end_pos) = after.find(block.end.as_str()) {
                    // Single-line block comment
                    let comment_text = after[..end_pos].trim();
                    comments.push(Comment { line: line_num, text: comment_text.to_string() });
                    // Continue checking the rest of the line
                    remaining = &after[end_pos + block.end.len()..];
                } else {
                    // Start of multi-line block comment
                    in_block_comment = true;
                    block_start_line = line_num;
                    block_text = after.trim().to_string();
                    current_block_end = block.end.clone();
                    break;
                }
            }
        }

        i += 1;
    }

    comments
}

/// Find a line comment using any of the line markers
fn find_line_comment(line: &str, line_num: usize, markers: &[String]) -> Option<Comment> {
    // Find the earliest matching marker
    let mut best_match: Option<(usize, &str)> = None;

    for marker in markers {
        if let Some(pos) = line.find(marker.as_str()) {
            match best_match {
                None => best_match = Some((pos, marker)),
                Some((best_pos, _)) if pos < best_pos => best_match = Some((pos, marker)),
                _ => {}
            }
        }
    }

    let (pos, marker) = best_match?;
    let comment_text = line[pos + marker.len()..].trim();
    Some(Comment { line: line_num, text: comment_text.to_string() })
}

/// Find a block comment start using any of the block patterns
fn find_block_start<'a>(line: &str, blocks: &'a [BlockSyntax]) -> Option<(&'a BlockSyntax, usize)> {
    // Find the earliest matching block start
    let mut best_match: Option<(&BlockSyntax, usize)> = None;

    for block in blocks {
        if let Some(pos) = line.find(block.start.as_str()) {
            match best_match {
                None => best_match = Some((block, pos)),
                Some((_, best_pos)) if pos < best_pos => best_match = Some((block, pos)),
                _ => {}
            }
        }
    }

    best_match
}

#[cfg(test)]
mod tests {
    use super::*;

    fn syntax_python() -> CustomCommentSyntax {
        CustomCommentSyntax {
            lines: vec!["#".to_string()],
            blocks: vec![BlockSyntax { start: "\"\"\"".to_string(), end: "\"\"\"".to_string() }],
        }
    }

    fn syntax_html() -> CustomCommentSyntax {
        CustomCommentSyntax {
            lines: vec![],
            blocks: vec![BlockSyntax { start: "<!--".to_string(), end: "-->".to_string() }],
        }
    }

    fn syntax_shell() -> CustomCommentSyntax {
        CustomCommentSyntax {
            lines: vec!["#".to_string()],
            blocks: vec![BlockSyntax { start: ": '".to_string(), end: "'".to_string() }],
        }
    }

    // =========================================================================
    // Line comment tests
    // =========================================================================

    #[test]
    fn test_hash_line_comment() {
        let content = "# This is a comment";
        let comments = extract_comments(content, &syntax_python());
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text, "This is a comment");
    }

    #[test]
    fn test_hash_line_comment_after_code() {
        let content = "x = 1 # inline comment";
        let comments = extract_comments(content, &syntax_python());
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text, "inline comment");
    }

    #[test]
    fn test_multiple_hash_comments() {
        let content = "# first\n# second";
        let comments = extract_comments(content, &syntax_python());
        assert_eq!(comments.len(), 2);
    }

    #[test]
    fn test_hash_comment_japanese() {
        let content = "# これは日本語のコメント";
        let comments = extract_comments(content, &syntax_python());
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text, "これは日本語のコメント");
    }

    // =========================================================================
    // Block comment tests
    // =========================================================================

    #[test]
    fn test_html_block_comment() {
        let content = "<!-- HTML comment -->";
        let comments = extract_comments(content, &syntax_html());
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text, "HTML comment");
    }

    #[test]
    fn test_html_multiline_comment() {
        let content = "<!--\n  Multi-line\n  comment\n-->";
        let comments = extract_comments(content, &syntax_html());
        assert_eq!(comments.len(), 1);
        assert!(comments[0].text.contains("Multi-line"));
    }

    #[test]
    fn test_python_docstring() {
        let content = "\"\"\"Python docstring\"\"\"";
        let comments = extract_comments(content, &syntax_python());
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text, "Python docstring");
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_empty_content() {
        let comments = extract_comments("", &syntax_python());
        assert!(comments.is_empty());
    }

    #[test]
    fn test_no_comments() {
        let content = "x = 1\ny = 2";
        let comments = extract_comments(content, &syntax_python());
        assert!(comments.is_empty());
    }

    #[test]
    fn test_shell_heredoc_style() {
        let content = ": '\nBlock comment\ntext\n'";
        let comments = extract_comments(content, &syntax_shell());
        assert_eq!(comments.len(), 1);
        assert!(comments[0].text.contains("Block comment"));
    }

    // =========================================================================
    // Multiple patterns tests
    // =========================================================================

    #[test]
    fn test_jsx_multiple_blocks() {
        let syntax = CustomCommentSyntax {
            lines: vec!["//".to_string()],
            blocks: vec![
                BlockSyntax { start: "/*".to_string(), end: "*/".to_string() },
                BlockSyntax { start: "{/*".to_string(), end: "*/}".to_string() },
            ],
        };
        let content = "/* JS comment */ {/* JSX comment */}";
        let comments = extract_comments(content, &syntax);
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].text, "JS comment");
        assert_eq!(comments[1].text, "JSX comment");
    }

    #[test]
    fn test_no_line_markers() {
        let syntax = CustomCommentSyntax {
            lines: vec![],
            blocks: vec![BlockSyntax { start: "<!--".to_string(), end: "-->".to_string() }],
        };
        let content = "// not a comment\n<!-- real comment -->";
        let comments = extract_comments(content, &syntax);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text, "real comment");
    }
}
