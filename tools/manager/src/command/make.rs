use std::env;
use anyhow::Result;
use test_generator::model::Config;
use crate::model::MakeArgs;

pub fn make(config: &MakeArgs) -> Result<()> {
    let path = env::current_dir()?;
    let config = Config::new(
        path,
        vec![config.target.clone()]
    );
    test_generator::run(config)
}