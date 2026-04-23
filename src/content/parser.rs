use crate::Error;
use std::path::Path;

use crate::content::front_matter::FrontMatter;
use crate::content::post::{Post, PostMeta};
use crate::error::Result;

pub fn parse_post(path: impl AsRef<Path>, source: &str) -> Result<Post> {
    let path = path.as_ref();
    let (front_matter, body) = split_front_matter(source)?;
    let metadata = match front_matter {
        Some(raw) => FrontMatter::parse(raw)?,
        None => FrontMatter::default(),
    };

    let title = metadata
        .title
        .clone()
        .or_else(|| fallback_title(path))
        .unwrap_or_else(|| "untitled".to_string());

    let slug = metadata
        .slug
        .clone()
        .unwrap_or_else(|| title.to_lowercase().replace(' ', "-"));

    let html = render_markdown(body);

    Ok(Post {
        source_path: path.to_path_buf(),
        meta: PostMeta {
            title,
            slug,
            date: metadata.date.clone(),
            tags: metadata.tags.clone(),
            draft: metadata.draft,
            summary: metadata.summary.clone(),
        },
        body: body.to_string(),
        html,
        excerpt: None,
    })
}

fn fallback_title(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|name| name.to_str())
        .map(|name| name.replace('-', " "))
}

fn split_front_matter(source: &str) -> Result<(Option<&str>, &str)> {
    let delimiter = "+++\n";

    if !source.starts_with(delimiter) {
        return Ok((None, source));
    }

    let remaining = &source[delimiter.len()..];
    let Some(end_pos) = remaining.find(delimiter) else {
        return Err(Error::message(format!(
            "front matter starts with `{delimiter}` but is missing a closing delimiter",
        )));
    };

    let front_matter = &remaining[..end_pos];
    let body = &remaining[end_pos + delimiter.len()..];

    Ok((Some(front_matter), body))
}

fn render_markdown(input: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);

    let parser = pulldown_cmark::Parser::new_ext(input, options);
    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_no_front_matter() {
        let source = "Hello, world!\n";
        let (fm, body) = split_front_matter(source).unwrap();
        assert!(fm.is_none());
        assert_eq!(body, "Hello, world!\n");
    }

    #[test]
    fn test_split_empty_source() {
        let (fm, body) = split_front_matter("").unwrap();
        assert!(fm.is_none());
        assert_eq!(body, "");
    }

    #[test]
    fn test_split_valid_front_matter() {
        let source = "+++\ntitle = \"Hello\"\n+++\nBody content\n";
        let (fm, body) = split_front_matter(source).unwrap();
        assert_eq!(fm.unwrap(), "title = \"Hello\"\n");
        assert_eq!(body, "Body content\n");
    }

    #[test]
    fn test_split_empty_front_matter() {
        let source = "+++\n+++\nBody\n";
        let (fm, body) = split_front_matter(source).unwrap();
        assert_eq!(fm.unwrap(), "");
        assert_eq!(body, "Body\n");
    }
    #[test]
    fn test_split_empty_body() {
        let source = "+++\ntitle = \"Hi\"\n+++\n";
        let (fm, body) = split_front_matter(source).unwrap();
        assert_eq!(fm.unwrap(), "title = \"Hi\"\n");
        assert_eq!(body, "");
    }
    #[test]
    fn test_split_missing_closing_delimiter() {
        let source = "+++\ntitle = \"Hello\"\n";
        let result = split_front_matter(source);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("missing a closing delimiter"));
    }

    // ================================
    // render_markdown
    // ================================

    #[test]
    fn test_render_basic_markdown() {
        let html = render_markdown("# Hello\n\n**bold** and *italic*\n");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
    }
    #[test]
    fn test_render_strikethrough() {
        // ENABLE_STRIKETHROUGH 扩展
        let html = render_markdown("~~deleted~~");
        assert!(html.contains("<del>deleted</del>"));
    }
    #[test]
    fn test_render_table() {
        // ENABLE_TABLES 扩展
        let md = "| A | B |\n|---|---|\n| 1 | 2 |\n";
        let html = render_markdown(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>A</th>") || html.contains("<th>A"));
    }
    #[test]
    fn test_render_tasklist() {
        // ENABLE_TASKLISTS 扩展
        let md = "- [x] done\n- [ ] todo\n";
        let html = render_markdown(md);
        assert!(html.contains("checkbox"));
        assert!(html.contains("checked"));
    }
    #[test]
    fn test_render_empty_input() {
        let html = render_markdown("");
        assert_eq!(html, "");
    }
}
