use std::{
    fs,
    process::{Command, Stdio},
};

use anyhow::{bail, Result};
use serde::Deserialize;
use tracing::info;

use crate::context::AppContext;

#[derive(Debug, Deserialize)]
struct CargopalConfig {
    template: String,
}

pub fn handle(ctx: &AppContext) -> Result<()> {
    // check for .cargopal.toml
    let config_str = match fs::read_to_string(".cargopal.toml") {
        Ok(s) => s,
        Err(_) => bail!("Not a cargopal project. Try `cargopal new` to create a new project."),
    };

    // parse the config file
    let config: CargopalConfig = toml::from_str(&config_str)?;

    // only allow `dev` command for `web` templates
    if config.template != "web" {
        bail!(
            "The `dev` command is only available for web templates. This is a '{}' project.",
            config.template
        );
    }

    info!("Starting dev server...");

    let mut cmd = Command::new("cargo");
    cmd.arg("run");

    if ctx.verbose {
        cmd.arg("--verbose");
    }

    let mut child = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;

    if !status.success() {
        bail!("Dev server failed with status: {}", status);
    }

    Ok(())
}
