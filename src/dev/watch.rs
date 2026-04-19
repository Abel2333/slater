use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct WatchSet {
    pub paths: Vec<PathBuf>,
}

impl WatchSet {
    pub fn add(&mut self, path: impl AsRef<Path>) {
        self.paths.push(path.as_ref().to_path_buf());
    }
}
