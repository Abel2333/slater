use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct SiteConfig {
    pub title: String,
    pub base_url: String,
    pub content_dir: PathBuf,
    pub output_dir: PathBuf,
    pub template_dir: PathBuf,
    pub static_dir: PathBuf,
    pub dev: DevConfig,
}

#[derive(Debug, Clone)]
pub struct DevConfig {
    pub host: String,
    pub port: u16,
    pub live_reload: bool,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "Slater".to_string(),
            base_url: "http://127.0.0.1:3000".to_string(),
            content_dir: PathBuf::from("content"),
            output_dir: PathBuf::from("public"),
            template_dir: PathBuf::from("templates"),
            static_dir: PathBuf::from("static"),
            dev: DevConfig::default(),
        }
    }
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            live_reload: true,
        }
    }
}

impl SiteConfig {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(Error::message(format!(
                "configuration file not found: {}",
                path.display()
            )));
        }

        Err(Error::message(
            "configuration parsing is not implemented yet",
        ))
    }

    pub fn validate(&self) -> Result<()> {
        if self.title.trim().is_empty() {
            return Err(Error::message("site title cannot be empty"));
        }

        if self.base_url.trim().is_empty() {
            return Err(Error::message("base_url cannot be empty"));
        }

        Ok(())
    }
}
