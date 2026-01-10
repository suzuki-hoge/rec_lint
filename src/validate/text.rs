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
