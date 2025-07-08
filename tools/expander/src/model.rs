use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, 
    Debug)]
pub struct Args {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: PathBuf,
    #[arg(long, required = true)]
    include: Vec<PathBuf>,
    #[arg(long)]
    project_root: PathBuf,
}

#[derive(Debug)]
pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub include: Vec<PathBuf>,
    pub project_root: PathBuf,
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
            input: args.input, 
            output: args.output, 
            include: args.include, 
            project_root: args.project_root 
        }
    }
}