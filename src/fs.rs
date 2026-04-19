use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Result;

pub fn ensure_dir(path: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn collect_files(root: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    let mut files = Vec::new();

    if !root.exists() {
        return Ok(files);
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }

    files.sort();
    Ok(files)
}
