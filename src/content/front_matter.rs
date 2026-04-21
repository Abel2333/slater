use serde::Deserialize;

use crate::error::{Error, Result};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub draft: bool,
    pub summary: Option<String>,
}

impl FrontMatter {
    fn validate(&self) -> Result<()> {
        if let Some(title) = &self.title
            && title.trim().is_empty()
        {
            return Err(Error::message("front matter title cannot be empty"));
        }

        if let Some(slug) = &self.slug
            && slug.trim().is_empty()
        {
            return Err(Error::message("front matter slug cannot be empty"));
        }

        if let Some(date) = &self.date
            && date.trim().is_empty()
        {
            return Err(Error::message("front matter date cannot be empty"));
        }

        if let Some(summary) = &self.summary
            && summary.trim().is_empty()
        {
            return Err(Error::message("front matter summary cannot be empty"));
        }

        if self.tags.iter().any(|tag| tag.trim().is_empty()) {
            return Err(Error::message(
                "front matter tags cannot contain empty values",
            ));
        }

        Ok(())
    }

    pub fn parse(input: &str) -> Result<Self> {
        let front_matter: Self = toml::from_str(input)?;
        front_matter.validate()?;
        Ok(front_matter)
    }
}
