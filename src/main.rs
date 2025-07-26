mod cli;
mod commands;
mod context;

use clap::Parser;
use cli::Cli;
use context::AppContext;

fn main() {
    let cli = Cli::parse();

    let ctx = AppContext {
        verbose: cli.verbose,
    };

    match &cli.command {
        cli::Commands::New { name } => commands::new::handle(&ctx, name),
        cli::Commands::Dev => commands::dev::handle(&ctx),
    };
}
