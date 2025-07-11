pub mod model;
pub mod command;
use clap::Parser;
use anyhow::Result;
use crate::model::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("Parsed arguments: {:?}", cli);
    match &cli.command {
        Commands::Run(_args) => {
            crate::command::run::run()?;
        }
        Commands::Test(args) => {
            println!("Testing with target: {}", args.target);
        }
        Commands::Build(args) => {
            crate::command::build::build(args)?;       
        }
        Commands::Configure(args) => {
            crate::command::configure::configure(args)?;
        }
        Commands::Make(args) => {
            crate::command::make::make(args)?;
        },
    }
    Ok(())
}
