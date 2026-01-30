pub mod add;
pub mod check;
pub mod desc;
pub mod guideline;
pub mod init;
pub mod show;
pub mod validate;
pub mod version;

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

/// Sort mode for validate command output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum SortMode {
    /// Sort by rule order (output: message: file:line:col)
    #[default]
    Rule,
    /// Sort by file order (output: file:line:col: message)
    File,
}

/// Check mode for check command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckMode {
    List,
    Tree,
    Schema,
    Html,
}

#[derive(Parser)]
#[command(name = "rec_lint")]
#[command(version)]
#[command(about = "Recursive linter with hierarchical configuration")]
#[command(long_about = "A recursive linter that reads .rec_lint.yaml files from directory hierarchy.\n\n\
Rules defined in parent directories are inherited by child directories.\n\
The root configuration file must have 'root: true'.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show effective rules for a directory
    #[command(long_about = "Display all effective rules for the specified directory.\n\n\
Rules are collected from .rec_lint.yaml files starting from the root (with 'root: true')\n\
down to the target directory. Output format:\n\n\
  [ rule ] <label>\n\
  [ rule ] <source_dir>: <label>\n\
  [ guideline ] <source_dir>: <message>")]
    Show {
        /// Target directory to show rules for (default: current directory)
        #[arg(value_name = "DIR", default_value = ".")]
        dir: PathBuf,
    },

    /// Validate files against rules
    #[command(long_about = "Validate files against rules.\n\n\
For directories, all files are recursively validated.\n\
Multiple paths can be specified.\n\n\
Validators:\n\
  - text: Check if keywords exist in file (substring match)\n\
  - regex: Check if patterns match in file\n\
  - custom: Run external command (exit code 0 = pass)")]
    Validate {
        /// Files or directories to validate (default: current directory)
        #[arg(value_name = "PATH", default_value = ".")]
        paths: Vec<PathBuf>,

        /// Sort order for output
        #[arg(long, short, value_enum, default_value = "rule")]
        sort: SortMode,
    },

    /// Show guideline points for a directory
    #[command(long_about = "Display guideline checklist items for the specified directory.\n\n\
Guideline items are informational reminders for code reviewers.")]
    Guideline {
        /// Target directory to show guideline points for (default: current directory)
        #[arg(value_name = "DIR", default_value = ".")]
        dir: PathBuf,
    },

    /// Show version
    Version,

    /// Initialize a new project (create .rec_lint_config.yaml)
    Init {
        /// Target directory (default: current directory)
        #[arg(value_name = "DIR", default_value = ".")]
        dir: PathBuf,
    },

    /// Add a new .rec_lint.yaml file
    Add {
        /// Target directory (default: current directory)
        #[arg(value_name = "DIR", default_value = ".")]
        dir: PathBuf,
    },

    /// Show description of rec_lint
    Desc,

    /// Check project configuration
    #[command(long_about = "Check project configuration structure.\n\n\
Options:\n\
  --list:   List directories with .rec_lint.yaml and their rule types\n\
  --tree:   Show directory tree with rule types at each level\n\
  --schema: Validate all .rec_lint.yaml files against JSON Schema\n\
  --html:   Open interactive HTML view in browser")]
    Check {
        /// List directories with .rec_lint.yaml and their rule types
        #[arg(long, group = "mode")]
        list: bool,

        /// Show directory tree with rule types
        #[arg(long, group = "mode")]
        tree: bool,

        /// Validate all .rec_lint.yaml files against JSON Schema
        #[arg(long, group = "mode")]
        schema: bool,

        /// Open interactive HTML view in browser
        #[arg(long, group = "mode")]
        html: bool,
    },
}
