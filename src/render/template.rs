use std::path::Path;

use minijinja::{Environment, context};
use serde::Serialize;

use crate::SiteConfig;
use crate::content::post::Post;
use crate::error::Result;

#[derive(Serialize)]
struct SiteView<'a> {
    title: &'a str,
    base_url: &'a str,
}

#[derive(Serialize)]
struct PostView<'a> {
    title: &'a str,
    date: Option<&'a str>,
    content: &'a str,
    url: String,
    summary: Option<&'a str>,
}

impl<'a> PostView<'a> {
    fn from_post(post: &'a Post) -> Self {
        Self {
            title: &post.meta.title,
            date: post.meta.date.as_deref(),
            content: &post.html,
            url: format!("/{}/", post.meta.slug),
            summary: post.meta.summary.as_deref().or(post.excerpt.as_deref()),
        }
    }
}

#[derive(Debug, Default)]
pub struct TemplateEngine {
    env: Environment<'static>,
}

impl TemplateEngine {
    pub fn new(template_dir: &Path) -> Result<Self> {
        let mut env = Environment::new();

        for name in ["base.html", "index.html", "post.html"] {
            let path = template_dir.join(name);
            let source = crate::fs::read_to_string(&path)?;
            env.add_template_owned(name.to_string(), source)?;
        }

        Ok(Self { env })
    }

    pub fn render_post(&self, post: &Post, config: &SiteConfig) -> Result<String> {
        let site_view = SiteView {
            title: &config.title,
            base_url: &config.base_url,
        };

        let post_view = PostView::from_post(post);
        let content = self.env.get_template("post.html")?.render(context! {
            site => site_view,
            post => post_view
        })?;

        let page = self.env.get_template("base.html")?.render(context! {
            site=>site_view,
            title=>post.meta.title.as_str(),
            content=>content
        })?;

        Ok(page)
    }

    pub fn render_index(&self, config: &SiteConfig, posts: &[Post]) -> Result<String> {
        let site_view = SiteView {
            title: &config.title,
            base_url: &config.base_url,
        };

        let post_views = posts.iter().map(PostView::from_post).collect::<Vec<_>>();

        let content = self.env.get_template("index.html")?.render(context! {
            site => site_view,
            posts => post_views
        })?;

        let page = self.env.get_template("base.html")?.render(context! {
            site => site_view,
            title => config.title.as_str(),
            content => content
        })?;

        Ok(page)
    }
}
