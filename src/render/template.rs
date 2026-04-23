use crate::SiteConfig;
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
            r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>{}</title>
  </head>
  <body>
    {}
  </body>
</html>
"#,
            post.meta.title, post.html,
        );

        Ok(html)
    }

    pub fn render_index(&self, config: &SiteConfig, posts: &[Post]) -> Result<String> {
        let mut items = String::new();

        for post in posts {
            let date = post.meta.date.as_deref().unwrap_or("");
            let summary = post
                .meta
                .summary
                .as_deref()
                .or(post.excerpt.as_deref())
                .unwrap_or("");

            items.push_str(&format!(
                r#"
            <article>
              <h2><a href="/{slug}/">{title}</a></h2>
              <p>{date}</p>
              <p>{summary}</p>
            </article>
                "#,
                slug = post.meta.slug,
                title = post.meta.title,
                date = date,
                summary = summary,
            ));
        }

        let html = format!(
            r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>{title}</title>
  </head>
  <body>
    <main>
      <h1>{title}</h1>
      {items}
    </main>
  </body>
</html>
"#,
            title = config.title,
            items = items,
        );

        Ok(html)
    }
}
