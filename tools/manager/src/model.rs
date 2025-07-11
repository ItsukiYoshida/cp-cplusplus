use clap::{Args, Parser, Subcommand};

pub enum Output {
    Stdout(String),
    Stderr(String),
}

#[derive(Parser, Debug)]
#[command(author = "ItsukiYoshida", version = "0.1", about = "競技プログラミング用コマンド", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "実行")]
    Run(RunArgs),
    #[command(about = "テスト実行 (結果をクリップボードにコピー)")]
    Test(TestArgs),
    #[command(about = "ビルド")]
    Build(BuildArgs),
    #[command(about = "初期設定")]
    Configure(ConfigureArgs),
    #[command(about = "テスト生成")]
    Make(MakeArgs),
}

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg(value_name = "TARGET")]
    pub target: String,

    #[arg(short, long)]
    pub release: bool,
}

#[derive(Args, Debug)]
pub struct TestArgs {
    #[arg(value_name = "TARGET")]
    pub target: String,

    #[arg(short, long)]
    pub release: bool,
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    #[arg(value_name = "TARGET")]
    pub target: String,

    #[arg(short, long)]
    pub release: bool,
}

#[derive(Args, Debug)]
pub struct ConfigureArgs {
    #[arg(value_name = "TARGET")]
    pub target: String,

    #[arg(short, long)]
    pub release: bool,
}

#[derive(Args, Debug)]
pub struct MakeArgs {
    #[arg(value_name = "TARGET")]
    pub target: String,

    #[arg(short, long)]
    pub release: bool,
}