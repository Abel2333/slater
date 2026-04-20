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
    /// Loads site configuration from an explicit path or a default location.
    ///
    /// Behavior:
    /// - If `path` is provided, load the configuration from that path.
    /// - If `path` is `None` and `./slater.toml` exists, load that file.
    /// - Otherwise, return the default configuration.
    pub fn load_config(path: Option<&Path>) -> Result<Self> {
        match path {
            Some(path) => Self::load_from_file(path),
            None => {
                let possible_path = PathBuf::from("./slater.toml");

                if possible_path.exists() {
                    Self::load_from_file(possible_path)
                } else {
                    Ok(Self::default())
                }
            }
        }
    }

    fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(Error::message(format!(
                "configuration file not found: {}",
                path.display()
            )));
        }

        let content = fs::read_to_string(path)?;
        let mut config: SiteConfig = toml::from_str(&content)?;

        let config_dir = path.parent().unwrap_or(Path::new("."));
        config.resolve_paths(config_dir);

        config.validate()?;

        Ok(config)
    }

    fn resolve_paths(&mut self, base_dir: &Path) {
        self.content_dir = resolve_path(base_dir, &self.content_dir);
        self.output_dir = resolve_path(base_dir, &self.output_dir);
        self.template_dir = resolve_path(base_dir, &self.template_dir);
        self.static_dir = resolve_path(base_dir, &self.static_dir);
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

fn resolve_path(base_dir: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use tempfile::tempdir;

    fn write_config(dir: &Path, content: &str) -> PathBuf {
        let path = dir.join("slater.toml");
        fs::write(&path, content).expect("failed to write test config");
        path
    }

    fn minimal_config() -> &'static str {
        r#"
title = "My Blog"
base_url = "http://127.0.0.1:3000"
"#
    }

    #[test]
    fn loads_from_explicit_path() {
        let dir = tempdir().expect("failed to create temp dir");
        let config_path = write_config(dir.path(), minimal_config());

        let config = SiteConfig::load_config(Some(config_path.as_path())).expect("load failed");

        assert_eq!(config.title, "My Blog");
        assert_eq!(config.base_url, "http://127.0.0.1:3000");
    }

    #[test]
    fn fills_missing_fields() {
        let dir = tempdir().expect("failed to create temp dir");
        let config_path = write_config(dir.path(), minimal_config());

        let config = SiteConfig::load_config(Some(config_path.as_path())).expect("load failed");

        assert_eq!(config.content_dir, dir.path().join("content"));
        assert_eq!(config.output_dir, dir.path().join("public"));
        assert_eq!(config.template_dir, dir.path().join("templates"));
        assert_eq!(config.static_dir, dir.path().join("static"));
        assert_eq!(config.dev.host, "127.0.0.1");
        assert_eq!(config.dev.port, 3000);
        assert!(config.dev.live_reload);
    }

    #[test]
    fn resolves_relative_paths() {
        let dir = tempdir().expect("failed to create temp dir");
        let content = r#"
title = "My Blog"
base_url = "http://127.0.0.1:3000"
content_dir = "posts"
output_dir = "dist"
template_dir = "views"
static_dir = "assets"
"#;
        let config_path = write_config(dir.path(), content);

        let config = SiteConfig::load_config(Some(config_path.as_path())).expect("load failed");

        assert_eq!(config.content_dir, dir.path().join("posts"));
        assert_eq!(config.output_dir, dir.path().join("dist"));
        assert_eq!(config.template_dir, dir.path().join("views"));
        assert_eq!(config.static_dir, dir.path().join("assets"));
    }

    #[test]
    fn error_missing_config_file() {
        let dir = tempdir().expect("failed to create temp dir");
        let missing_path = dir.path().join("missing.toml");

        let error =
            SiteConfig::load_config(Some(missing_path.as_path())).expect_err("expected failure");

        assert!(
            error.to_string().contains("configuration file not found"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn error_invalid_toml() {
        let dir = tempdir().expect("failed to create temp dir");
        let config_path = write_config(dir.path(), r#"title = "Broken"#);

        let result = SiteConfig::load_config(Some(config_path.as_path()));

        assert!(result.is_err(), "expected parse error");
    }

    #[test]
    fn error_for_invalid_values() {
        let dir = tempdir().expect("failed to create temp dir");
        let content = r#"
title = "My Blog"
base_url = "http://127.0.0.1:3000"
content_dir = "site"
output_dir = "site"
"#;
        let config_path = write_config(dir.path(), content);

        let error = SiteConfig::load_config(Some(config_path.as_path()))
            .expect_err("expected validation error");

        assert!(
            error
                .to_string()
                .contains("content_dir and output_dir must be different"),
            "unexpected error: {error}"
        );
    }
}
