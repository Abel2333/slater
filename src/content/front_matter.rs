use crate::error::{Error, Result};

#[derive(Debug, Clone, Default)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub draft: bool,
    pub summary: Option<String>,
}

impl FrontMatter {
    pub fn validate(&self) -> Result<()> {
        if let Some(title) = &self.title {
            if title.trim().is_empty() {
                return Err(Error::message("front matter title cannot be empty"));
            }
        }

        Ok(())
    }
}
