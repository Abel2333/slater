pub mod build;
pub mod init;
pub mod new;
pub mod serve;

use clap::{Parser, Subcommand};

use crate::error::Result;

#[derive(Debug, Parser)]
#[command(name = "slater")]
#[command(about = "A static site generator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Build the site into the output directory
    Build(build::BuildOptions),

    /// Start the local development server
    Serve(serve::ServeOptions),

    /// Create a new content scaffold
    New(new::NewOptions),

    /// Initialize a new site in the target directory
    Init(init::InitOptions),
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build(options) => build::execute(options),
        Commands::Serve(options) => serve::execute(options),
        Commands::New(options) => new::execute(options),
        Commands::Init(options) => init::execute(options),
    }
}
