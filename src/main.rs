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
        Commands::New { dir, root } => commands::new::run(&dir, root)?,
    };

    for line in output {
        println!("{line}");
    }

    Ok(())
}
