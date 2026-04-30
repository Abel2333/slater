use std::path::PathBuf;

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
    pub fn new(config: &SiteConfig) -> Result<Self> {
        let mut env = Environment::new();

        for name in ["base.html", "index.html", "post.html"] {
            let source = load_template_source(config, name)?;
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

fn builtin_theme_template_dir(theme: &str) -> PathBuf {
    PathBuf::from("assets/themes").join(theme).join("templates")
}

fn load_template_source(config: &SiteConfig, name: &str) -> Result<String> {
    let project_path = config.template_dir.join(name);
    if project_path.exists() {
        return crate::fs::read_to_string(&project_path);
    }

    let builtin_path = builtin_theme_template_dir(&config.theme).join(name);
    if builtin_path.exists() {
        return crate::fs::read_to_string(&builtin_path);
    }

    Err(crate::error::Error::message(format!(
        "template `{name}` not found in project directory ({}) or built-in theme `{}` ({})",
        config.template_dir.display(),
        config.theme,
        builtin_theme_template_dir(&config.theme).display(),
    )))
}
