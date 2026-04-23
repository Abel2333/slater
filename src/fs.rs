use std::fs;
use std::path::{Path, PathBuf};

use crate::Error;
use crate::error::Result;

/// Creates the directory and any missing parent directories.
pub fn ensure_dir(path: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

/// Ensures the parent directory of a path exists when the path has a parent.
pub fn ensure_parent_dir(path: impl AsRef<Path>) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        ensure_dir(parent)?;
    }
    Ok(())
}

/// Reads a UTF-8 text file into a string.
pub fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

/// Writes a UTF-8 text file, creating parent directories when needed.
pub fn write_string(path: impl AsRef<Path>, contents: &str) -> Result<()> {
    ensure_parent_dir(&path)?;
    fs::write(path, contents)?;
    Ok(())
}

/// Returns whether a directory exists and contains at least one entry.
pub fn dir_has_entries(path: impl AsRef<Path>) -> Result<bool> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(false);
    }

    let mut entries = fs::read_dir(path)?;
    Ok(entries.next().is_some())
}

/// Recursively collects all files under a directory and returns them sorted.
pub fn collect_files(root: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    let mut files = Vec::new();

    if !root.exists() {
        return Ok(files);
    }

    collect_files_into(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_into(root: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_files_into(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }

    Ok(())
}

/// Recursively copies the contents of one directory into another directory.
pub fn copy_dir_contents(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    if !from.exists() {
        return Err(Error::message(format!(
            "source directory does not exist: {}",
            from.display()
        )));
    }

    ensure_dir(to)?;

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let source = entry.path();
        let target = to.join(entry.file_name());

        if source.is_dir() {
            copy_dir_contents(&source, &target)?;
        } else if source.is_file() {
            ensure_parent_dir(&target)?;
            fs::copy(&source, &target)?;
        }
    }

    Ok(())
}
