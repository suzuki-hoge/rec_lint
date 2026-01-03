pub mod java;
pub mod kotlin;
pub mod rust;

use crate::rule::parser::Visibility;

/// A missing doc violation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocViolation {
    pub line: usize,
    pub kind: DocKind,
    pub name: String,
}

/// Kind of item missing documentation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocKind {
    Type,
    Constructor,
    Function,
    Macro,
}

impl std::fmt::Display for DocKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocKind::Type => write!(f, "type"),
            DocKind::Constructor => write!(f, "constructor"),
            DocKind::Function => write!(f, "function"),
            DocKind::Macro => write!(f, "macro"),
        }
    }
}

/// Config for Java/Kotlin doc checks
#[derive(Debug, Clone, Default)]
pub struct JvmDocConfig {
    pub type_visibility: Option<Visibility>,
    pub constructor_visibility: Option<Visibility>,
    pub function_visibility: Option<Visibility>,
}

/// Config for Rust doc checks
#[derive(Debug, Clone, Default)]
pub struct RustDocConfig {
    pub type_visibility: Option<Visibility>,
    pub function_visibility: Option<Visibility>,
    pub check_macro: bool,
}
