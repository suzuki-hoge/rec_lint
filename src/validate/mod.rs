pub mod comment;
pub mod custom;
pub mod doc;
pub mod regex;
pub mod test;
pub mod text;

/// A violation found by text or regex validator
#[derive(Debug)]
pub struct Violation {
    pub line: usize,
    pub col: usize,
    pub found: String,
}

/// A violation found by custom validator
#[derive(Debug)]
pub struct CustomViolation {
    pub output: String,
}
