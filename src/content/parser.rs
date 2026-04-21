use crate::Error;
use std::path::Path;

use crate::content::front_matter::FrontMatter;
use crate::content::post::{Post, PostMeta};
use crate::error::Result;

pub fn parse_post(path: impl AsRef<Path>, source: &str) -> Result<Post> {
    let path = path.as_ref();
    let front_matter = FrontMatter::default();

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

fn extract_front_matter(source: &str) -> Result<Option<&str>> {
    let delimiter = "+++\n";
    let mut remaining = source;

    // find start
    let Some(start_pos) = remaining.find(delimiter) else {
        return Ok(None);
    };
    remaining = &remaining[start_pos + delimiter.len()..];

    // find end
    let Some(end_pos) = remaining.find(delimiter) else {
        return Err(Error::message(format!(
            "could not find the second delimiter {delimiter}"
        )));
    };

    Ok(Some(&remaining[..end_pos]))
}
