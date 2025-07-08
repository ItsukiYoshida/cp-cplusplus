use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[arg(short = 'p', long)]
    project_root: Option<PathBuf>,

    #[arg(action = clap::ArgAction::Append)]
    problems: Option<Vec<String>>,
}

pub struct Config {
    pub project_root: Option<PathBuf>,
    pub problems: Option<Vec<String>>,
}

impl Config {
    pub fn new() -> Self {
        let args = Args::parse();
        args.into()
    }
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config { 
            project_root: args.project_root, 
            problems: args.problems 
        }
    }
}