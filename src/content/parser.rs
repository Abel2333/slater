use std::path::Path;

use crate::content::front_matter::FrontMatter;
use crate::content::post::{Post, PostMeta};
use crate::error::Result;

pub fn parse_post(path: impl AsRef<Path>, source: &str) -> Result<Post> {
    let path = path.as_ref();
    let front_matter = FrontMatter::default();
    front_matter.validate()?;

    let title = front_matter
        .title
        .clone()
        .or_else(|| fallback_title(path))
        .unwrap_or_else(|| "untitled".to_string());
    let slug = front_matter
        .slug
        .clone()
        .unwrap_or_else(|| title.to_lowercase().replace(' ', "-"));

    Ok(Post {
        source_path: path.to_path_buf(),
        meta: PostMeta {
            title,
            slug,
            date: front_matter.date.clone(),
            tags: front_matter.tags.clone(),
            draft: front_matter.draft,
            summary: front_matter.summary.clone(),
        },
        body: source.to_string(),
        html: String::new(),
        excerpt: None,
    })
}

fn fallback_title(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|name| name.to_str())
        .map(|name| name.replace('-', " "))
}
