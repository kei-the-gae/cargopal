use anyhow::{bail, Result};
use duct::{cmd, Handle};
use notify::{RecursiveMode, Watcher};
use serde::Deserialize;
use std::{
    fs,
    path::Path,
    sync::mpsc::{channel, RecvTimeoutError},
    time::Duration,
};
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
    let watch_path = Path::new("./src");
    watcher.watch(watch_path, RecursiveMode::Recursive)?;

    info!("Starting dev server...");
    let mut child = run_server(ctx)?;

    loop {
        // wait for a file change event, or check if the server has exited
        match rx.recv_timeout(Duration::from_millis(100)) {
            // file change was detected
            Ok(Ok(event)) => {
                debug!("File change detected: {:?}", event);
                info!("Change detected. Waiting for a quiet period before restarting...");

                // debounce: keep consuming events until there are no more for a short period
                loop {
                    match rx.recv_timeout(Duration::from_millis(200)) {
                        Ok(_) => continue,                       // another event came in, reset the timer
                        Err(RecvTimeoutError::Timeout) => break, // quiet period elapsed
                        Err(e) => bail!("File watcher channel error during debounce: {}", e),
                    }
                }

                info!("Restarting server...");

                // stop watching for changes to prevent a feedback loop during the build.
                watcher.unwatch(watch_path)?;

                if let Err(e) = child.kill() {
                    debug!("Failed to kill process (it may have already exited): {}", e);
                }
                // wait for the process to ensure it's fully terminated.
                let _ = child.wait();

                child = run_server(ctx)?;

                // give the build process a moment to complete before watching for new changes.
                std::thread::sleep(Duration::from_secs(1));

                // resume watching for file changes.
                info!("Resuming file watcher.");
                watcher.watch(watch_path, RecursiveMode::Recursive)?;
            }
            // file watcher error occurred
            Ok(Err(e)) => {
                bail!("File watcher error: {}", e);
            }
            // no file changes, check if the server process has exited
            Err(RecvTimeoutError::Timeout) => {
                if let Some(status) = child.try_wait()? {
                    info!(
                        "Server process exited with status: {:?}. Restarting...",
                        status
                    );
                    child = run_server(ctx)?;
                }
            }
            // file watcher channel was disconnected
            Err(RecvTimeoutError::Disconnected) => {
                bail!("File watcher channel disconnected.");
            }
        }
    }
}
