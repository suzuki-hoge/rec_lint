use crate::rule::RegexRule;
use crate::validate::Violation;

pub fn validate(content: &str, rule: &RegexRule) -> Vec<Violation> {
    let mut violations = Vec::new();
    for (line_num, line) in content.lines().enumerate() {
        for pattern in &rule.patterns {
            if let Some(m) = pattern.find(line) {
                violations.push(Violation { line: line_num + 1, col: m.start() + 1, found: line.to_string() });
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
    use regex::Regex;

    fn make_rule(patterns: Vec<&str>) -> RegexRule {
        let compiled: Vec<Regex> = patterns.iter().map(|p| Regex::new(p).unwrap()).collect();
        RegexRule {
            label: "test".to_string(),
            patterns: compiled,
            keywords: patterns.into_iter().map(String::from).collect(),
            message: "test message".to_string(),
            matcher: Matcher::default(),
        }
    }

    // =========================================================================
    // 基本的なマッチング
    // =========================================================================

    #[test]
    fn マッチしない場合は空の結果を返す() {
        let rule = make_rule(vec![r"println"]);
        let content = "let x = 42;";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn リテラルパターンがマッチする() {
        let rule = make_rule(vec![r"println"]);
        let content = "println!(\"hello\");";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn 空のコンテンツは違反なし() {
        let rule = make_rule(vec![r".*"]);
        let content = "";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    // =========================================================================
    // 正規表現固有の機能
    // =========================================================================

    #[test]
    fn ワイルドカードパターンがマッチする() {
        let rule = make_rule(vec![r"ng-word.*"]);
        let content = "// ng-word: fix this later";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 4); // "// " = 3 chars
    }

    #[test]
    fn 文字クラスがマッチする() {
        let rule = make_rule(vec![r"[0-9]+"]);
        let content = "let x = 42;";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 9); // "let x = " = 8 chars
    }

    #[test]
    fn 単語境界で完全一致する() {
        let rule = make_rule(vec![r"\bvar\b"]);
        let content = "var x = 1;";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn 単語境界は部分一致しない() {
        let rule = make_rule(vec![r"\bvar\b"]);
        let content = "variable x = 1;";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn 行頭アンカーがマッチする() {
        let rule = make_rule(vec![r"^import"]);
        let content = "import java.util.*;";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    #[test]
    fn 行頭アンカーは行中ではマッチしない() {
        let rule = make_rule(vec![r"^import"]);
        let content = "// import java.util.*;";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn オプショナルパターンが両方にマッチする() {
        let rule = make_rule(vec![r"colou?r"]);
        let content1 = "color is red";
        let content2 = "colour is blue";
        assert_eq!(validate(content1, &rule).len(), 1);
        assert_eq!(validate(content2, &rule).len(), 1);
    }

    #[test]
    fn 選択パターンがいずれかにマッチする() {
        let rule = make_rule(vec![r"foo|bar"]);
        let content = "baz bar qux";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 5); // "baz " = 4 chars
    }

    // =========================================================================
    // 複数パターン
    // =========================================================================

    #[test]
    fn 複数パターンでは先に定義されたパターンが優先される() {
        let rule = make_rule(vec![r"alpha", r"beta"]);
        let content = "beta alpha";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        // "alpha" is first pattern, checked first, finds match at col 6
        assert_eq!(violations[0].col, 6);
    }

    #[test]
    fn 最初のパターンが無ければ次のパターンでマッチする() {
        let rule = make_rule(vec![r"alpha", r"beta"]);
        let content = "beta gamma";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1);
    }

    // =========================================================================
    // 複数行
    // =========================================================================

    #[test]
    fn 複数行の違反を検出できる() {
        let rule = make_rule(vec![r"error"]);
        let content = "ok line\nerror line 1\nok again\nerror line 2";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].line, 2);
        assert_eq!(violations[1].line, 4);
    }

    #[test]
    fn 行番号は1から始まる() {
        let rule = make_rule(vec![r"target"]);
        let content = "line1\nline2\ntarget here";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].line, 3);
    }

    // =========================================================================
    // カラム位置
    // =========================================================================

    #[test]
    fn カラム位置は正規表現マッチの開始位置() {
        let rule = make_rule(vec![r"[A-Z]+"]);
        let content = "abc XYZ def";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].col, 5); // "abc " = 4 chars, match starts at 5
    }

    #[test]
    fn 貪欲マッチでもカラム位置は正しい() {
        let rule = make_rule(vec![r"a+"]);
        let content = "bbaaa";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].col, 3); // "bb" = 2 chars
    }

    // =========================================================================
    // エッジケース
    // =========================================================================

    #[test]
    fn パターンが空の場合は違反なし() {
        let rule = make_rule(vec![]);
        let content = "any content";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn ドットは任意の1文字にマッチする() {
        let rule = make_rule(vec![r"a.c"]);
        let content = "abc axc a c";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].col, 1); // first "abc" matches
    }

    #[test]
    fn 検出結果には行全体が含まれる() {
        let rule = make_rule(vec![r"\d+"]);
        let content = "value = 123;";
        let violations = validate(content, &rule);
        assert_eq!(violations[0].found, "value = 123;");
    }

    #[test]
    fn デフォルトでは大文字小文字を区別する() {
        let rule = make_rule(vec![r"NGWORD"]);
        let content = "ngword item";
        let violations = validate(content, &rule);
        assert!(violations.is_empty());
    }

    #[test]
    fn 大文字小文字を無視するフラグが使える() {
        let rule = make_rule(vec![r"(?i)NGWORD"]);
        let content = "ngword item";
        let violations = validate(content, &rule);
        assert_eq!(violations.len(), 1);
    }
}
