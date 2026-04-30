use std::path::PathBuf;

use clap::Args;

use crate::config::SiteConfig;
use crate::error::Result;

#[derive(Debug, Clone, Args)]
pub struct ServeOptions {
    #[arg(long = "config")]
    pub config_path: Option<PathBuf>,

    #[arg(long = "port")]
    pub port: Option<u16>,

    #[arg(long = "host")]
    pub host: Option<String>,
}

pub async fn execute(options: ServeOptions) -> Result<()> {
    let mut config = SiteConfig::load_config(options.config_path.as_deref())?;
    if let Some(port) = options.port {
        config.dev.port = port
    }

    if let Some(host) = options.host {
        config.dev.host = host
    }

    crate::serve(&config).await
}
