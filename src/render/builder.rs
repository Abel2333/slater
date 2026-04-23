use crate::config::SiteConfig;
use crate::content::parser;
use crate::content::post::Post;
use crate::error::Result;
use crate::render::template::TemplateEngine;

pub fn build_site(config: &SiteConfig) -> Result<()> {
    crate::fs::ensure_dir(&config.output_dir)?;

    let engine = TemplateEngine::new();
    let posts = load_posts(config)?;

    write_post_pages(config, &engine, &posts)?;
    write_index_page(config, &engine, &posts)?;
    copy_static_assets(config)?;

    println!("build finished");
    Ok(())
}

fn load_posts(config: &SiteConfig) -> Result<Vec<Post>> {
    let files = crate::fs::collect_files(&config.content_dir)?;
    let mut posts = Vec::new();

    for path in files {
        let is_markdown = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("md"))
            .unwrap_or(false);

        if !is_markdown {
            continue;
        }

        let source = crate::fs::read_to_string(&path)?;
        let post = parser::parse_post(&path, &source)?;

        if post.meta.draft {
            continue;
        }

        posts.push(post);
    }

    posts.sort_by(|a, b| {
        b.meta
            .date
            .cmp(&a.meta.date)
            .then_with(|| a.meta.slug.cmp(&b.meta.slug))
    });

    Ok(posts)
}

fn write_post_pages(config: &SiteConfig, engine: &TemplateEngine, posts: &[Post]) -> Result<()> {
    for post in posts {
        let rendered = engine.render_post(post)?;
        let output_path = config.output_dir.join(&post.meta.slug).join("index.html");

        crate::fs::write_string(output_path, &rendered)?;
    }

    Ok(())
}

fn write_index_page(config: &SiteConfig, engine: &TemplateEngine, posts: &[Post]) -> Result<()> {
    let html = engine.render_index(config, posts)?;
    crate::fs::write_string(config.output_dir.join("index.html"), &html)?;
    Ok(())
}

fn copy_static_assets(config: &SiteConfig) -> Result<()> {
    if !config.static_dir.exists() {
        return Ok(());
    }

    crate::fs::copy_dir_contents(&config.static_dir, &config.output_dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use tempfile::tempdir;

    fn test_config(root: &std::path::Path) -> SiteConfig {
        SiteConfig {
            title: "Test Site".to_string(),
            base_url: "http://127.0.0.1:3000".to_string(),
            content_dir: root.join("content"),
            output_dir: root.join("public"),
            template_dir: root.join("templates"),
            static_dir: root.join("static"),
            dev: Default::default(),
        }
    }

    #[test]
    fn builds_posts_index_and_static_assets() {
        let dir = tempdir().expect("failed to create temp dir");
        let config = test_config(dir.path());

        crate::fs::ensure_dir(&config.content_dir).expect("failed to create content dir");
        crate::fs::ensure_dir(&config.static_dir).expect("failed to create static dir");

        crate::fs::write_string(
            config.content_dir.join("hello-world.md"),
            r#"+++
title = "Hello World"
slug = "hello-world"
date = "2026-04-20"
tags = ["test"]
draft = false
summary = "My first post."
+++

Welcome to **Slater**.
"#,
        )
        .expect("failed to write post");

        crate::fs::write_string(
            config.static_dir.join("style.css"),
            "body { color: black; }\n",
        )
        .expect("failed to write static asset");

        build_site(&config).expect("build failed");

        let index =
            fs::read_to_string(config.output_dir.join("index.html")).expect("missing index page");
        let post = fs::read_to_string(config.output_dir.join("hello-world").join("index.html"))
            .expect("missing post page");
        let css =
            fs::read_to_string(config.output_dir.join("style.css")).expect("missing static asset");

        assert!(index.contains("Test Site"));
        assert!(index.contains(r#"<a href="/hello-world/">Hello World</a>"#));
        assert!(index.contains("My first post."));
        assert!(post.contains("<strong>Slater</strong>"));
        assert_eq!(css, "body { color: black; }\n");
    }

    #[test]
    fn skips_draft_posts() {
        let dir = tempdir().expect("failed to create temp dir");
        let config = test_config(dir.path());

        crate::fs::ensure_dir(&config.content_dir).expect("failed to create content dir");

        crate::fs::write_string(
            config.content_dir.join("draft.md"),
            r#"+++
title = "Draft Post"
slug = "draft-post"
tags = []
draft = true
+++

This should not be published.
"#,
        )
        .expect("failed to write draft");

        build_site(&config).expect("build failed");

        assert!(
            !config
                .output_dir
                .join("draft-post")
                .join("index.html")
                .exists()
        );

        let index =
            fs::read_to_string(config.output_dir.join("index.html")).expect("missing index page");
        assert!(!index.contains("Draft Post"));
    }
}
