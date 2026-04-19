use crate::config::SiteConfig;
use crate::error::Result;

pub fn serve(config: &SiteConfig) -> Result<()> {
    crate::render::builder::build_site(config)?;
    println!(
        "serve skeleton is ready at http://{}:{}",
        config.dev.host, config.dev.port
    );
    Ok(())
}
