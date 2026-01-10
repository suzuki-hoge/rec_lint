use std::path::{Path, PathBuf};

/// ルートの test-projects/ から相対パスで取得する
pub fn test_project_path(subpath: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-projects").join(subpath)
}

#[allow(dead_code)]
pub fn project_dir(name: &str) -> PathBuf {
    test_project_path(Path::new("rules").join(name))
}

#[allow(dead_code)]
pub fn project_file(name: &str, relative: impl AsRef<Path>) -> PathBuf {
    project_dir(name).join(relative)
}

/// Vec<String> を1つの文字列に連結する
pub fn joined_output(lines: &[String]) -> String {
    lines.join("\n")
}

/// 複数行リテラル (インデント込み) を正規化して比較する
pub fn assert_output(actual: &[String], expected: &str) {
    assert_eq!(joined_output(actual), normalize_multiline(expected));
}

fn normalize_multiline(expected: &str) -> String {
    if expected.is_empty() {
        return String::new();
    }

    let mut content = expected;
    if let Some(stripped) = content.strip_prefix('\n') {
        content = stripped;
    }
    if let Some(stripped) = content.strip_suffix('\n') {
        content = stripped;
    }

    if content.is_empty() {
        return String::new();
    }

    let lines: Vec<&str> = content.lines().collect();
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|c| *c == ' ').count())
        .min()
        .unwrap_or(0);

    lines
        .iter()
        .map(|line| trim_indent(line, indent))
        .filter(|line| !line.is_empty() || lines.iter().filter(|l| !l.trim().is_empty()).count() > 0)
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

fn trim_indent(line: &str, indent: usize) -> String {
    if line.trim().is_empty() {
        return String::new();
    }
    if indent == 0 {
        return line.trim_end_matches('\r').to_string();
    }
    let mut trimmed = 0;
    let mut idx = 0;
    for ch in line.chars() {
        if trimmed >= indent || ch != ' ' {
            break;
        }
        trimmed += 1;
        idx += ch.len_utf8();
    }
    line[idx..].trim_end_matches('\r').to_string()
}
