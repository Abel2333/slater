use clap::Args;

use crate::config::SiteConfig;
use crate::error::Result;

#[derive(Debug, Clone, Args)]
pub struct BuildOptions {
    #[arg(long = "config")]
    pub config_path: Option<String>,
}

pub fn execute(options: BuildOptions) -> Result<()> {
    let config = load_config(options.config_path)?;
    crate::build(&config)
}

fn load_config(config_path: Option<String>) -> Result<SiteConfig> {
    match config_path {
        Some(path) => SiteConfig::load_from_file(path),
        None => Ok(SiteConfig::default()),
    }
}
