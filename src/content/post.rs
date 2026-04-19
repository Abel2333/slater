use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Post {
    pub source_path: PathBuf,
    pub meta: PostMeta,
    pub body: String,
    pub html: String,
    pub excerpt: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PostMeta {
    pub title: String,
    pub slug: String,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub draft: bool,
    pub summary: Option<String>,
}
