mod cli;
mod commands;
mod context;
mod utils;

use clap::Parser;
use cli::Cli;
use context::AppContext;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

fn main() {
    let cli = Cli::parse();

    let ctx = AppContext {
        verbose: cli.verbose,
    };

    match &cli.command {
        cli::Commands::New { template, name } => commands::new::handle(&ctx, template, name),
        cli::Commands::Dev => commands::dev::handle(&ctx),
    };
}
