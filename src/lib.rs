pub mod cmd;
pub mod config;
pub mod content;
pub mod dev;
pub mod error;
pub mod fs;
pub mod render;

pub use config::SiteConfig;
pub use error::{Error, Result};

pub fn build(config: &SiteConfig) -> Result<()> {
    render::builder::build_site(config)
}

pub fn serve(config: &SiteConfig) -> Result<()> {
    dev::server::serve(config)
}
