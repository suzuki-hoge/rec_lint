use crate::rule::TextRule;
use crate::validate::Violation;

pub fn validate(content: &str, rule: &TextRule) -> Vec<Violation> {
    let mut violations = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        for keyword in &rule.keywords {
            if let Some(col) = line.find(keyword) {
                violations.push(Violation { line: line_num + 1, col: col + 1, found: line.to_string() });
                break;
            }
        }
    }
    violations
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use crate::matcher::Matcher;

    fn make_rule(keywords: Vec<&str>) -> TextRule {
        TextRule {
            label: "test".to_string(),
            keywords: keywords.into_iter().map(String::from).collect(),
            message: "test message".to_string(),
            matcher: Matcher::default(),
        }
    }

    // =========================================================================
    // 基本的なマッチング
    // =========================================================================

    #[test]
    fn マッチしない場合は空の結果を返す() {
        let rule = make_rule(vec!["println"]);
        let content = "let x = 42;";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn キーワードがマッチすると違反が検出される() {
        let rule = make_rule(vec!["println"]);
        let content = "println!(\"hello\");";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
        assert_eq!(violations[0].col, 1);
        assert_eq!(violations[0].found, "println!(\"hello\");");
    }

    #[test]
    fn 行中のキーワードも検出される() {
        let rule = make_rule(vec!["ng-word"]);
        let content = "// This is a ng-word item";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 14); // "// This is a " = 13 chars, ng-word starts at 14
    }

    #[test]
    fn 空のコンテンツは違反なし() {
        let rule = make_rule(vec!["println"]);
        let content = "";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // 複数キーワード
    // =========================================================================

    #[test]
    fn 複数キーワードでは先に定義されたものが優先される() {
        let rule = make_rule(vec!["alpha", "beta"]);
        let content = "beta alpha gamma";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        // "alpha" is first in keywords list, but "beta" appears first in line
        // However, we iterate keywords in order, so "alpha" is checked first
        assert_eq!(violations[0].col, 6); // "beta " = 5 chars, alpha starts at 6
    }

    #[test]
    fn 最初のキーワードが無ければ次のキーワードでマッチする() {
        let rule = make_rule(vec!["alpha", "beta"]);
        let content = "beta gamma";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn 各行で最初にマッチしたキーワードのみ報告される() {
        // Even if multiple keywords match, only first match per line is reported (break)
        let rule = make_rule(vec!["a", "b"]);
        let content = "a b c";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1); // 'a' at position 1
    }

    // =========================================================================
    // 複数行
    // =========================================================================

    #[test]
    fn 複数行の違反を検出できる() {
        let rule = make_rule(vec!["bad"]);
        let content = "good line\nbad line 1\ngood again\nbad line 2";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].line, 2);
        assert_eq!(violations[0].col, 1);
        assert_eq!(violations[1].line, 4);
        assert_eq!(violations[1].col, 1);
    }

    #[test]
    fn 行番号は1から始まる() {
        let rule = make_rule(vec!["target"]);
        let content = "line1\nline2\ntarget here";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 3);
    }

    // =========================================================================
    // エッジケース
    // =========================================================================

    #[test]
    fn 行末のキーワードも検出される() {
        let rule = make_rule(vec!["end"]);
        let content = "this is the end";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 13); // "this is the " = 12 chars
    }

    #[test]
    fn 大文字小文字を区別する() {
        let rule = make_rule(vec!["NGWORD"]);
        let content = "ngword item";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn 部分文字列もマッチする() {
        let rule = make_rule(vec!["print"]);
        let content = "println!()";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn キーワードが空の場合は違反なし() {
        let rule = make_rule(vec![]);
        let content = "any content";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn 検出結果には行全体が含まれる() {
        let rule = make_rule(vec!["x"]);
        let content = "let x = 42;";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].found, "let x = 42;");
    }
}
