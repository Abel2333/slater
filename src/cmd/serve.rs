use clap::Args;

use crate::config::SiteConfig;
use crate::error::Result;

#[derive(Debug, Clone, Args)]
pub struct ServeOptions {
    #[arg(long = "config")]
    pub config_path: Option<String>,
}

pub fn execute(options: ServeOptions) -> Result<()> {
    let config = match options.config_path {
        Some(path) => SiteConfig::load_from_file(path)?,
        None => SiteConfig::default(),
    };

    crate::serve(&config)
}
