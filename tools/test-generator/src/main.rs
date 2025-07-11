use anyhow::Result;
use test_generator::{model::Config, run};

fn main() -> Result<()> {
    let config = Config::from_args();
    run(config)?;
    Ok(())
}
