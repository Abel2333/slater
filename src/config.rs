use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{
    error::{Error, Result},
    fs,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct SiteConfig {
    pub title: String,
    pub base_url: String,
    pub content_dir: PathBuf,
    pub output_dir: PathBuf,
    pub template_dir: PathBuf,
    pub static_dir: PathBuf,
    pub dev: DevConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
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

        let content = fs::read_to_string(path)?;
        let config: SiteConfig = toml::from_str(&content)?;

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        self.validate_site_fields()?;
        self.validate_dev_config()?;
        self.validate_paths()?;

        Ok(())
    }

    fn validate_site_fields(&self) -> Result<()> {
        if self.title.trim().is_empty() {
            return Err(Error::message("site title cannot be empty"));
        }

        if self.base_url.trim().is_empty() {
            return Err(Error::message("base_url cannot be empty"));
        }

        if !(self.base_url.starts_with("http://") || self.base_url.starts_with("https://")) {
            return Err(Error::message(
                "base_url must start with `http://` or `https://`",
            ));
        }

        Ok(())
    }

    fn validate_dev_config(&self) -> Result<()> {
        if self.dev.host.trim().is_empty() {
            return Err(Error::message("dev.host cannot be empty"));
        }

        if self.dev.port == 0 {
            return Err(Error::message("dev.port must be greater than 0"));
        }

        Ok(())
    }

    fn validate_paths(&self) -> Result<()> {
        if self.content_dir == self.output_dir {
            return Err(Error::message(
                "content_dir and output_dir must be different",
            ));
        }

        if self.template_dir == self.output_dir {
            return Err(Error::message(
                "template_dir and output_dir must be different",
            ));
        }

        if self.static_dir == self.output_dir {
            return Err(Error::message(
                "static_dir and output_dir must be different",
            ));
        }

        Ok(())
    }
}
