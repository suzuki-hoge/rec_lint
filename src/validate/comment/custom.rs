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
    let mut current_block_end = String::new();

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        let line_num = i + 1;

        if in_block_comment {
            // Continue block comment
            if let Some(end_pos) = line.find(current_block_end.as_str()) {
                // End of block comment - extract text before end marker
                let before_end = line[..end_pos].trim();
                if !before_end.is_empty() {
                    comments.push(Comment { line: line_num, text: before_end.to_string() });
                }

                in_block_comment = false;

                // Check for line comment after block end
                let after_end = &line[end_pos + current_block_end.len()..];
                if let Some(comment) = find_line_comment(after_end, line_num, &syntax.lines) {
                    comments.push(comment);
                }
            } else {
                // Middle of block comment - each line is a separate comment
                if !trimmed.is_empty() {
                    comments.push(Comment { line: line_num, text: trimmed.to_string() });
                }
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
                    current_block_end = block.end.clone();
                    // Extract first line content if any
                    let first_line_text = after.trim();
                    if !first_line_text.is_empty() {
                        comments.push(Comment { line: line_num, text: first_line_text.to_string() });
                    }
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
        let mut search_start = 0;
        while let Some(rel_pos) = line[search_start..].find(marker.as_str()) {
            let pos = search_start + rel_pos;

            // Skip "://" patterns (e.g., http://, https://, ftp://)
            if marker == "//" && pos > 0 && line.as_bytes().get(pos - 1) == Some(&b':') {
                search_start = pos + marker.len();
                continue;
            }

            match best_match {
                None => best_match = Some((pos, marker)),
                Some((best_pos, _)) if pos < best_pos => best_match = Some((pos, marker)),
                _ => {}
            }
            break;
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
