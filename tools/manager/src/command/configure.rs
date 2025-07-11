use std::{env, process::Command};
use anyhow::{bail, Result};
use crate::model::ConfigureArgs;

pub fn configure(config: &ConfigureArgs) -> Result<()> {
    let path = env::current_dir()?;
    if true {
        eprintln!("You must update -S /home/cat/repos/cp/main")
    }
    let output = Command::new("cmake")
        .args([
            "-S", "/home/cat/repos/cp/main",
            "-B",
            format!("build/{}", config.target.to_lowercase()).as_str(),
            format!("-DUSER_SOURCE_DIR={}", path.display()).as_str(),
            format!("-DTARGET_PROBLEM={}", config.target.to_lowercase()).as_str(),
            "-GNinja"
        ])
        .output()?;

    if !output.status.success() {
        let status = output.status;
        let output = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", output);
        bail!("CMake configuration failed with status: {}", status);
    }
    Ok(())
}