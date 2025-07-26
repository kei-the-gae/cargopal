use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cargopal", version, arg_required_else_help = true)]
/// Scaffold and run Rust apps with ease
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new project
    New {
        /// Project name
        name: String,
    },
    /// Run the development server
    Dev,
}
