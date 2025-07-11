use std::process::Command;

use anyhow::{bail, Result};

use crate::model::BuildArgs;

pub fn build(config: &BuildArgs) -> Result<()> {
    let output = Command::new("cmake")
        .args([
            "--build",
            format!("build/{}", config.target.to_lowercase()).as_str(),
        ])
        .output()?;

    if !output.status.success() {
        let status = output.status;
        let output = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", output);
        bail!("Build failed with status: {}", status);
    }
    Ok(())
}