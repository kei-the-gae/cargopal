mod cli;
mod commands;
mod context;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use context::AppContext;
use rust_embed::RustEmbed;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let filter = EnvFilter::builder()
        .with_default_directive(if cli.verbose {
            LevelFilter::DEBUG.into()
        } else {
            LevelFilter::INFO.into()
        })
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let ctx = AppContext {};

    match &cli.command {
        cli::Commands::New { template, name } => commands::new::handle(&ctx, template, name),
        cli::Commands::Dev => commands::dev::handle(&ctx),
    }
}
