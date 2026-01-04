use anyhow::Result;
use clap::Parser;

use rec_lint::commands;
use rec_lint::commands::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Show { dir } => commands::show::run(&dir)?,
        Commands::Validate { paths, sort } => commands::validate::run(&paths, sort)?,
        Commands::Guideline { dir } => commands::guideline::run(&dir)?,
        Commands::Version => commands::version::run()?,
        Commands::Init { dir } => commands::init::run(&dir)?,
        Commands::Add { dir } => commands::add::run(&dir)?,
        Commands::Desc => commands::desc::run()?,
    };

    for line in output {
        println!("{line}");
    }

    Ok(())
}
