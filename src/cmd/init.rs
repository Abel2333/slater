use std::path::{Path, PathBuf};

use clap::Args;

use crate::error::Result;

#[derive(Debug, Clone, Args)]
pub struct InitOptions {
    /// Directory where the new site will be created
    pub target_dir: Option<PathBuf>,

    /// Override the site title in slater.toml
    #[arg(long)]
    pub title: Option<String>,

    /// Built-in theme to initialize
    #[arg(long)]
    pub theme: Option<String>,

    /// Allow initialization in a non-empty directory
    #[arg(long)]
    pub force: bool,
}

struct InitPlan {
    root_dir: PathBuf,
    overwrite: bool,
    write_files: Vec<PlannedFile>,
}

struct PlannedFile {
    path: PathBuf,
    contents: String,
}

impl InitPlan {
    fn from_options(options: InitOptions) -> Result<Self> {
        let root_dir = match options.target_dir {
            Some(target) => target,
            None => PathBuf::from("."),
        };

        let site_title = options
            .title
            .unwrap_or_else(|| default_site_title(&root_dir));
        let theme = options.theme.unwrap_or_else(|| "default".to_string());

        let mut write_files = vec![PlannedFile::new(
            root_dir.join("slater.toml"),
            render_init_config(&site_title, &theme)?,
        )];

        write_files.extend(planned_files_from_dir(
            &root_dir.join("content"),
            Path::new("assets/init/common/content"),
        )?);

        write_files.extend(planned_files_from_dir(
            &root_dir.join("templates"),
            &PathBuf::from("assets/themes")
                .join(&theme)
                .join("templates"),
        )?);

        write_files.extend(planned_files_from_dir(
            &root_dir.join("static"),
            &PathBuf::from("assets/themes").join(&theme).join("static"),
        )?);

        Ok(Self {
            root_dir,
            overwrite: options.force,
            write_files,
        })
    }
}

impl PlannedFile {
    fn new(path: PathBuf, contents: String) -> Self {
        Self { path, contents }
    }
}

/// Helper Functions
fn default_site_title(root_dir: &Path) -> String {
    let Some(name) = root_dir.file_name().and_then(|name| name.to_str()) else {
        return "My Blog".to_string();
    };

    let trimmed = name.trim();
    if trimmed.is_empty() || trimmed == "." {
        return "My Blog".to_string();
    }

    trimmed
        .split(['-', '_', ' '])
        .filter(|part| !part.is_empty())
        .map(capitalize_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => {
            let mut result = first.to_uppercase().collect::<String>();
            result.push_str(chars.as_str());
            result
        }
        None => String::new(),
    }
}

fn read_init_asset(relative_path: &str) -> Result<String> {
    let path = PathBuf::from("assets/init").join(relative_path);

    crate::fs::read_to_string(&path).map_err(|error| {
        crate::error::Error::message(format!(
            "failed to load init asset {}: {error}",
            path.display()
        ))
    })
}

fn planned_files_from_dir(target_dir: &Path, source_dir: &Path) -> Result<Vec<PlannedFile>> {
    let paths = crate::fs::collect_files(source_dir)?;
    let mut files = Vec::new();

    for path in paths {
        let relative_path = path.strip_prefix(source_dir).map_err(|error| {
            crate::error::Error::message(format!(
                "failed to resolve relative path for {}: {error}",
                path.display()
            ))
        })?;

        let contents = crate::fs::read_to_string(&path).map_err(|error| {
            crate::error::Error::message(format!(
                "failed to load init asset {}: {error}",
                path.display()
            ))
        })?;

        files.push(PlannedFile::new(target_dir.join(relative_path), contents));
    }

    Ok(files)
}

fn render_init_config(site_title: &str, theme: &str) -> Result<String> {
    let template = read_init_asset("common/slater.toml")?;
    Ok(template
        .replace("__SLATER_SITE_TITLE__", site_title)
        .replace("__SLATER_THEME__", theme))
}

/// Process Functions
fn validate_plan(plan: &InitPlan) -> Result<()> {
    if crate::fs::dir_has_entries(&plan.root_dir)? && !plan.overwrite {
        return Err(crate::error::Error::message(
            "target directory is not empty; use --force to continue",
        ));
    }

    for file in &plan.write_files {
        if file.path.exists() && !plan.overwrite {
            return Err(crate::error::Error::message(format!(
                "file already exists: {}",
                file.path.display()
            )));
        }
    }

    Ok(())
}

fn apply_plan(plan: &InitPlan) -> Result<()> {
    // root directory
    crate::fs::ensure_dir(&plan.root_dir)?;

    // all files
    for file in &plan.write_files {
        crate::fs::write_string(&file.path, &file.contents)?;
    }

    Ok(())
}

fn print_success(plan: &InitPlan) {
    println!("initialized site at {}", plan.root_dir.display());
    println!();
    println!("next steps:");

    if plan.root_dir != Path::new(".") {
        println!("  cd {}", plan.root_dir.display());
    }

    println!("  slater serve");
}

pub fn execute(options: InitOptions) -> Result<()> {
    let plan = InitPlan::from_options(options)?;
    validate_plan(&plan)?;
    apply_plan(&plan)?;
    print_success(&plan);

    Ok(())
}
