#[path = "../../common/mod.rs"]
mod common;

use rec_lint::commands::SortMode;

fn validate_comment(relative: &str) -> Vec<String> {
    let file = common::project_file("require_japanese_comment", relative);
    rec_lint::commands::validate::run(&[file], SortMode::Rule).unwrap()
}

fn expect_comment_ok(relative: &str) {
    let result = validate_comment(relative);
    assert!(result.is_empty(), "expected no violations for {relative}, got {result:?}");
}

fn expect_comment_violation(relative: &str, expected: &str) {
    let result = validate_comment(relative);
    common::assert_output(&result, expected);
}

// Java like syntax (line)

#[test]
fn java_line_違反なし() {
    expect_comment_ok("java_like_syntax/line/違反なし.java");
}

#[test]
fn java_line_コメントなし() {
    expect_comment_ok("java_like_syntax/line/コメントなし.java");
}

#[test]
fn java_line_コメント行に英語コメントがある() {
    expect_comment_violation(
        "java_like_syntax/line/コメント行に英語コメントがある.java",
        r#"
            コメントを日本語にしてください: "english comment": require_japanese_comment/java_like_syntax/line/コメント行に英語コメントがある.java:1:1
        "#,
    );
}

#[test]
fn java_line_コード行末に英語コメントがある() {
    expect_comment_violation(
        "java_like_syntax/line/コード行末に英語コメントがある.java",
        r#"
            コメントを日本語にしてください: "description": require_japanese_comment/java_like_syntax/line/コード行末に英語コメントがある.java:1:1
        "#,
    );
}

#[test]
fn java_line_複数行に英語コメントがある() {
    expect_comment_violation(
        "java_like_syntax/line/複数行に英語コメントがある.java",
        r#"
            コメントを日本語にしてください: "first line": require_japanese_comment/java_like_syntax/line/複数行に英語コメントがある.java:1:1
            コメントを日本語にしてください: "second line": require_japanese_comment/java_like_syntax/line/複数行に英語コメントがある.java:2:1
        "#,
    );
}

#[test]
fn java_line_URL文字列がある() {
    expect_comment_ok("java_like_syntax/line/URL文字列がある.java");
}

#[test]
fn java_line_URL文字列と同じ行末にコメントがある() {
    expect_comment_violation(
        "java_like_syntax/line/URL文字列と同じ行末にコメントがある.java",
        r#"
            コメントを日本語にしてください: "description": require_japanese_comment/java_like_syntax/line/URL文字列と同じ行末にコメントがある.java:1:1
        "#,
    );
}

// Java like syntax (block)

#[test]
fn java_block_違反なし() {
    expect_comment_ok("java_like_syntax/block/違反なし.java");
}

#[test]
fn java_block_コメントなし() {
    expect_comment_ok("java_like_syntax/block/コメントなし.java");
}

#[test]
fn java_block_単一行ブロックコメントに英語がある() {
    expect_comment_violation(
        "java_like_syntax/block/単一行ブロックコメントに英語がある.java",
        r#"
            コメントを日本語にしてください: "english comment": require_japanese_comment/java_like_syntax/block/単一行ブロックコメントに英語がある.java:1:1
        "#,
    );
}

#[test]
fn java_block_複数行ブロックコメントに英語がある() {
    expect_comment_violation(
        "java_like_syntax/block/複数行ブロックコメントに英語がある.java",
        r#"
            コメントを日本語にしてください: "* first line": require_japanese_comment/java_like_syntax/block/複数行ブロックコメントに英語がある.java:2:1
            コメントを日本語にしてください: "* second line": require_japanese_comment/java_like_syntax/block/複数行ブロックコメントに英語がある.java:3:1
        "#,
    );
}

#[test]
fn java_block_コード行中のブロックコメントに英語がある() {
    expect_comment_violation(
        "java_like_syntax/block/コード行の中の単一行ブロックコメントに英語がある.java",
        r#"
            コメントを日本語にしてください: "english": require_japanese_comment/java_like_syntax/block/コード行の中の単一行ブロックコメントに英語がある.java:1:1
        "#,
    );
}

// Python like syntax (line)

#[test]
fn python_line_違反なし() {
    expect_comment_ok("python_like_syntax/line/違反なし.py");
}

#[test]
fn python_line_コメントなし() {
    expect_comment_ok("python_like_syntax/line/コメントなし.py");
}

#[test]
fn python_line_コメント行に英語コメントがある() {
    expect_comment_violation(
        "python_like_syntax/line/コメント行に英語コメントがある.py",
        r#"
            コメントを日本語にしてください: "english comment": require_japanese_comment/python_like_syntax/line/コメント行に英語コメントがある.py:1:1
        "#,
    );
}

#[test]
fn python_line_コード行末に英語コメントがある() {
    expect_comment_violation(
        "python_like_syntax/line/コード行末に英語コメントがある.py",
        r#"
            コメントを日本語にしてください: "description": require_japanese_comment/python_like_syntax/line/コード行末に英語コメントがある.py:1:1
        "#,
    );
}

#[test]
fn python_line_複数行に英語コメントがある() {
    expect_comment_violation(
        "python_like_syntax/line/複数行に英語コメントがある.py",
        r#"
            コメントを日本語にしてください: "first line": require_japanese_comment/python_like_syntax/line/複数行に英語コメントがある.py:1:1
            コメントを日本語にしてください: "second line": require_japanese_comment/python_like_syntax/line/複数行に英語コメントがある.py:2:1
        "#,
    );
}

#[test]
fn python_line_URL文字列がある() {
    expect_comment_ok("python_like_syntax/line/URL文字列がある.py");
}

#[test]
fn python_line_URL文字列と同じ行末にコメントがある() {
    expect_comment_violation(
        "python_like_syntax/line/URL文字列と同じ行末にコメントがある.py",
        r#"
            コメントを日本語にしてください: "description": require_japanese_comment/python_like_syntax/line/URL文字列と同じ行末にコメントがある.py:1:1
        "#,
    );
}

// Python like syntax (block)

#[test]
fn python_block_違反なし() {
    expect_comment_ok("python_like_syntax/block/違反なし.py");
}

#[test]
fn python_block_コメントなし() {
    expect_comment_ok("python_like_syntax/block/コメントなし.py");
}

#[test]
fn python_block_単一行ブロックコメントに英語がある() {
    expect_comment_violation(
        "python_like_syntax/block/単一行ブロックコメントに英語がある.py",
        r#"
            コメントを日本語にしてください: "english": require_japanese_comment/python_like_syntax/block/単一行ブロックコメントに英語がある.py:1:1
        "#,
    );
}

#[test]
fn python_block_複数行ブロックコメントに英語がある() {
    expect_comment_violation(
        "python_like_syntax/block/複数行ブロックコメントに英語がある.py",
        r#"
            コメントを日本語にしてください: "first line": require_japanese_comment/python_like_syntax/block/複数行ブロックコメントに英語がある.py:2:1
            コメントを日本語にしてください: "second line": require_japanese_comment/python_like_syntax/block/複数行ブロックコメントに英語がある.py:3:1
        "#,
    );
}

#[test]
fn python_block_コード行中のブロックコメントに英語がある() {
    expect_comment_violation(
        "python_like_syntax/block/コード行の中の単一行ブロックコメントに英語がある.py",
        r#"
            コメントを日本語にしてください: "english": require_japanese_comment/python_like_syntax/block/コード行の中の単一行ブロックコメントに英語がある.py:2:1
        "#,
    );
}

// Rust like syntax (line)

#[test]
fn rust_line_違反なし() {
    expect_comment_ok("rust_like_syntax/line/違反なし.rs");
}

#[test]
fn rust_line_コメントなし() {
    expect_comment_ok("rust_like_syntax/line/コメントなし.rs");
}

#[test]
fn rust_line_コメント行に英語コメントがある() {
    expect_comment_violation(
        "rust_like_syntax/line/コメント行に英語コメントがある.rs",
        r#"
            コメントを日本語にしてください: "english comment": require_japanese_comment/rust_like_syntax/line/コメント行に英語コメントがある.rs:1:1
        "#,
    );
}

#[test]
fn rust_line_コード行末に英語コメントがある() {
    expect_comment_violation(
        "rust_like_syntax/line/コード行末に英語コメントがある.rs",
        r#"
            コメントを日本語にしてください: "description": require_japanese_comment/rust_like_syntax/line/コード行末に英語コメントがある.rs:1:1
        "#,
    );
}

#[test]
fn rust_line_複数行に英語コメントがある() {
    expect_comment_violation(
        "rust_like_syntax/line/複数行に英語コメントがある.rs",
        r#"
            コメントを日本語にしてください: "first line": require_japanese_comment/rust_like_syntax/line/複数行に英語コメントがある.rs:1:1
            コメントを日本語にしてください: "second line": require_japanese_comment/rust_like_syntax/line/複数行に英語コメントがある.rs:2:1
        "#,
    );
}

#[test]
fn rust_line_URL文字列がある() {
    expect_comment_ok("rust_like_syntax/line/URL文字列がある.rs");
}

#[test]
fn rust_line_URL文字列と同じ行末にコメントがある() {
    expect_comment_violation(
        "rust_like_syntax/line/URL文字列と同じ行末にコメントがある.rs",
        r#"
            コメントを日本語にしてください: "description": require_japanese_comment/rust_like_syntax/line/URL文字列と同じ行末にコメントがある.rs:1:1
        "#,
    );
}

// Rust like syntax (block)

#[test]
fn rust_block_違反なし() {
    expect_comment_ok("rust_like_syntax/block/違反なし.rs");
}

#[test]
fn rust_block_コメントなし() {
    expect_comment_ok("rust_like_syntax/block/コメントなし.rs");
}

#[test]
fn rust_block_単一行ブロックコメントに英語がある() {
    expect_comment_violation(
        "rust_like_syntax/block/単一行ブロックコメントに英語がある.rs",
        r#"
            コメントを日本語にしてください: "english comment": require_japanese_comment/rust_like_syntax/block/単一行ブロックコメントに英語がある.rs:1:1
        "#,
    );
}

#[test]
fn rust_block_複数行ブロックコメントに英語がある() {
    expect_comment_violation(
        "rust_like_syntax/block/複数行ブロックコメントに英語がある.rs",
        r#"
            コメントを日本語にしてください: "* first line": require_japanese_comment/rust_like_syntax/block/複数行ブロックコメントに英語がある.rs:2:1
            コメントを日本語にしてください: "* second line": require_japanese_comment/rust_like_syntax/block/複数行ブロックコメントに英語がある.rs:3:1
        "#,
    );
}

#[test]
fn rust_block_コード行中のブロックコメントに英語がある() {
    expect_comment_violation(
        "rust_like_syntax/block/コード行の中の単一行ブロックコメントに英語がある.rs",
        r#"
            コメントを日本語にしてください: "english": require_japanese_comment/rust_like_syntax/block/コード行の中の単一行ブロックコメントに英語がある.rs:1:1
        "#,
    );
}
