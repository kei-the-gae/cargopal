use anyhow::{bail, Result};
use duct::{cmd, Handle};
use notify::{RecursiveMode, Watcher};
use serde::Deserialize;
use std::{fs, path::Path, sync::mpsc::channel, time::Duration};
use tracing::{debug, info};

use crate::context::AppContext;

#[derive(Debug, Deserialize)]
struct CargopalConfig {
    template: String,
}

fn run_server(ctx: &AppContext) -> Result<Handle> {
    let mut run_args = vec!["run"];
    if ctx.verbose {
        run_args.push("--verbose");
    }
    let handle = cmd("cargo", run_args).start()?;
    Ok(handle)
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

    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new("./src"), RecursiveMode::Recursive)?;

    info!("Starting dev server...");
    let mut child = run_server(ctx)?;

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                debug!("File change detected: {:?}", event);
                info!("Restarting server...");
                if let Err(e) = child.kill() {
                    debug!("Failed to kill process: {}", e);
                }
                child = run_server(ctx)?;
            }
            Err(_) => {
                // timeout, check if process is still running
                if child.try_wait()?.is_some() {
                    info!("Server process exited. Restarting...");
                    child = run_server(ctx)?;
                }
            }
        }
    }
}
