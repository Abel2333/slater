use std::path::PathBuf;

use clap::Args;

use crate::config::SiteConfig;
use crate::error::Result;

#[derive(Debug, Clone, Args)]
pub struct ServeOptions {
    #[arg(long = "config")]
    pub config_path: Option<PathBuf>,
}

pub fn execute(options: ServeOptions) -> Result<()> {
    let config = SiteConfig::load_config(options.config_path.as_deref())?;

    crate::serve(&config)
}
