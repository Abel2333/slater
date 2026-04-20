use std::path::PathBuf;

use clap::Args;

use crate::config::SiteConfig;
use crate::error::Result;

#[derive(Debug, Clone, Args)]
pub struct BuildOptions {
    #[arg(long = "config")]
    pub config_path: Option<PathBuf>,
}

pub fn execute(options: BuildOptions) -> Result<()> {
    let config = SiteConfig::load_config(options.config_path.as_deref())?;

    crate::build(&config)
}
