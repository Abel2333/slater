use crate::config::SiteConfig;
use crate::content::parser;
use crate::error::Result;
use crate::render::template::TemplateEngine;

pub fn build_site(config: &SiteConfig) -> Result<()> {
    crate::fs::ensure_dir(&config.output_dir)?;

    let engine = TemplateEngine::new();
    let post = parser::parse_post(config.content_dir.join("index.md"), "")?;
    let _rendered = engine.render_post(&post)?;

    println!("build skeleton is ready");
    Ok(())
}
