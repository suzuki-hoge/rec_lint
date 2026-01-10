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
