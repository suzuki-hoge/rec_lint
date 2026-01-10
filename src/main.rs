use std::process::ExitCode;

use clap::Parser;

use rec_lint::commands;
use rec_lint::commands::{CheckMode, Cli, Commands};

fn main() -> ExitCode {
    match run() {
        Ok(exit_code) => exit_code,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> anyhow::Result<ExitCode> {
    let cli = Cli::parse();

    let is_validate = matches!(cli.command, Commands::Validate { .. });

    let output = match cli.command {
        Commands::Show { dir } => commands::show::run(&dir)?,
        Commands::Validate { paths, sort } => commands::validate::run(&paths, sort)?,
        Commands::Guideline { dir } => commands::guideline::run(&dir)?,
        Commands::Version => commands::version::run()?,
        Commands::Init { dir } => commands::init::run(&dir)?,
        Commands::Add { dir } => commands::add::run(&dir)?,
        Commands::Desc => commands::desc::run()?,
        Commands::Check { list, tree, schema } => {
            let mode = if list {
                CheckMode::List
            } else if tree {
                CheckMode::Tree
            } else if schema {
                CheckMode::Schema
            } else {
                // No option provided, show help
                return Err(anyhow::anyhow!(
                    "No option specified. Use --list, --tree, or --schema.\n\n\
                    Run 'rec_lint check --help' for more information."
                ));
            };
            commands::check::run(mode)?
        }
    };

    let has_violations = is_validate && !output.is_empty();

    for line in output {
        println!("{line}");
    }

    if has_violations {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
