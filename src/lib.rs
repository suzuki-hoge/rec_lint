pub mod collector;
pub mod commands;
pub mod config;
pub mod rule;
pub mod validator;

use clap::ValueEnum;

/// Sort mode for validate command output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum SortMode {
    /// Sort by rule order (output: message: file:line:col)
    #[default]
    Rule,
    /// Sort by file order (output: file:line:col: message)
    File,
}
