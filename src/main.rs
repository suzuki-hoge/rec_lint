mod cli;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
use rec_lint::commands;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Show { dir } => commands::show::run(&dir)?,
        Commands::Validate { paths, sort } => commands::validate::run(&paths, sort)?,
        Commands::Review { dir } => commands::review::run(&dir)?,
    };

    for line in output {
        println!("{line}");
    }

    Ok(())
}
