pub mod kotest;
pub mod phpunit;
pub mod rust;

use crate::rule::parser::TestRequireLevel;

/// A test existence violation (missing test file or untested public method)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestExistenceViolation {
    pub kind: TestExistenceViolationKind,
}

/// Kind of test existence violation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestExistenceViolationKind {
    /// Test file does not exist
    MissingTestFile { expected_path: String },
    /// Public method not tested
    UntestedPublicMethod { line: usize, method_name: String },
    /// Unit test does not exist
    MissingUnitTest,
    /// Public function not tested
    UntestedPublicFunction { line: usize, function_name: String },
}

impl std::fmt::Display for TestExistenceViolationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestExistenceViolationKind::MissingTestFile { expected_path } => {
                write!(f, "テストファイルが存在しません: {expected_path}")
            }
            TestExistenceViolationKind::UntestedPublicMethod { line, method_name } => {
                write!(f, "L{line}: public メソッド `{method_name}` がテストされていません")
            }
            TestExistenceViolationKind::MissingUnitTest => {
                write!(f, "ユニットテストが存在しません")
            }
            TestExistenceViolationKind::UntestedPublicFunction { line, function_name } => {
                write!(f, "L{line}: pub 関数 `{function_name}` がテストされていません")
            }
        }
    }
}

/// Config for external file test existence checks (PHPUnit, Kotest)
#[derive(Debug, Clone)]
pub struct ExternalFileTestConfig {
    pub test_directory: String,
    pub require: TestRequireLevel,
    pub test_file_suffix: String,
}

/// Config for same file test existence checks (Rust unit test)
#[derive(Debug, Clone)]
pub struct SameFileTestConfig {
    pub require: TestRequireLevel,
}

impl Default for SameFileTestConfig {
    fn default() -> Self {
        Self { require: TestRequireLevel::Exists }
    }
}
