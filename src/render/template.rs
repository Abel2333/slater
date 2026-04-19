use crate::content::post::Post;
use crate::error::Result;

#[derive(Debug, Default)]
pub struct TemplateEngine;

impl TemplateEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn render_post(&self, post: &Post) -> Result<String> {
        let html = format!(
            "<html><head><title>{}</title></head><body>{}</body></html>",
            post.meta.title, post.body
        );
        Ok(html)
    }
}
