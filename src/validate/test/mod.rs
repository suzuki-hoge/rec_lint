pub mod kotest;
pub mod phpunit;
pub mod rust;

use super::comment::contains_japanese;

/// A test name violation (test name without Japanese)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestViolation {
    pub line: usize,
    pub name: String,
}

/// Filter test names that don't contain Japanese
pub fn filter_non_japanese(tests: Vec<(usize, String)>) -> Vec<TestViolation> {
    tests
        .into_iter()
        .filter(|(_, name)| !contains_japanese(name))
        .map(|(line, name)| TestViolation { line, name })
        .collect()
}
