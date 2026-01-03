pub mod review;
pub mod show;
pub mod validate;

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

#[derive(Parser)]
#[command(name = "rec_lint")]
#[command(version)]
#[command(about = "Recursive linter with hierarchical configuration")]
#[command(long_about = "A recursive linter that reads rec_lint.yaml files from directory hierarchy.\n\n\
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
Rules are collected from rec_lint.yaml files starting from the root (with 'root: true')\n\
down to the target directory. Output format:\n\n\
  required: <label> [keywords] @ <source_dir>\n\
  deny: <label> [keywords] @ <source_dir>\n\
  review: <message> @ <source_dir>")]
    Show {
        /// Target directory to show rules for
        #[arg(value_name = "DIR")]
        dir: PathBuf,
    },

    /// Validate files against rules
    #[command(long_about = "Validate files against required and deny rules.\n\n\
For directories, all files are recursively validated.\n\
Multiple paths can be specified.\n\n\
Validators:\n\
  - text: Check if keywords exist in file (substring match)\n\
  - regex: Check if patterns match in file\n\
  - custom: Run external command (exit code 0 = pass)")]
    Validate {
        /// Files or directories to validate
        #[arg(required = true, value_name = "PATH")]
        paths: Vec<PathBuf>,

        /// Sort order for output
        #[arg(long, short, value_enum, default_value = "rule")]
        sort: SortMode,
    },

    /// Show review points for a directory
    #[command(long_about = "Display review checklist items for the specified directory.\n\n\
Review items are informational reminders for code reviewers.")]
    Review {
        /// Target directory to show review points for
        #[arg(value_name = "DIR")]
        dir: PathBuf,
    },
}
