use anyhow::Result;
use expander::{model::Config, run};

fn main() -> Result<()> {
    let config = Config::new();
    run(config)?;
    Ok(())
}