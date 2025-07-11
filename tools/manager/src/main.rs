pub mod model;
pub mod command;
use clap::Parser;
use anyhow::Result;
use crate::{command::run::run, model::{Cli, Commands}};

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("Parsed arguments: {:?}", cli);
    match &cli.command {
        Commands::Run(_args) => {
                        // println!("Running with target: {}", args.target);
                        run()?;
            }
        Commands::Test(args) => {
                println!("Testing with target: {}", args.target);
            }
        Commands::Build(args) => {
                if args.release {
                    println!("Building release target: {}", args.target);
                } else {
                    println!("Building debug target: {}", args.target);
                }
            }
        Commands::Configure(_args) => {

            }
        Commands::Make(_make_args) => {
            
        },
    }
    Ok(())
}
