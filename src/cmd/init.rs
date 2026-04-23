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

    /// Allow initialization in a non-empty directory
    #[arg(long)]
    pub force: bool,
}

struct InitPlan {
    root_dir: PathBuf,
    overwrite: bool,
    create_dirs: Vec<PathBuf>,
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

        let create_dirs = vec![
            root_dir.join("content"),
            root_dir.join("templates"),
            root_dir.join("static"),
        ];

        let write_files = vec![
            PlannedFile::new(
                root_dir.join("slater.toml"),
                render_init_config(&site_title)?,
            ),
            planned_asset_file(&root_dir, "content/hello-world.md")?,
            planned_asset_file(&root_dir, "templates/base.html")?,
            planned_asset_file(&root_dir, "templates/index.html")?,
            planned_asset_file(&root_dir, "templates/post.html")?,
            planned_asset_file(&root_dir, "static/style.css")?,
        ];

        Ok(Self {
            root_dir,
            overwrite: options.force,
            create_dirs,
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

fn planned_asset_file(root_dir: &Path, relative_path: &str) -> Result<PlannedFile> {
    let contents = read_init_asset(relative_path)?;
    Ok(PlannedFile::new(root_dir.join(relative_path), contents))
}

fn render_init_config(site_title: &str) -> Result<String> {
    let template = read_init_asset("slater.toml")?;
    Ok(template.replace("__SLATER_SITE_TITLE__", site_title))
}

/// Process Functions
fn validate_plan(plan: &InitPlan) -> Result<()> {
    if crate::fs::dir_has_entries(&plan.root_dir)? {
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

    // other directories
    for dir in &plan.create_dirs {
        crate::fs::ensure_dir(dir)?;
    }

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
