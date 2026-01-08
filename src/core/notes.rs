use std::path::{Path, PathBuf};

pub struct Note {
    path: PathBuf,
    name: String,
    group: Option<String>,
    mtime: std::time::SystemTime,
}

impl Note {
    pub fn new(name: &str, path: &Path, group: Option<String>) -> Self {
        Self {
            path: path.to_path_buf(),
            name: name.to_string(),
            group,
            mtime: std::time::SystemTime::now(),
        }
    }

    pub fn mtime(&self) -> std::time::SystemTime {
        self.mtime
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn group(&self) -> &Option<String> {
        &self.group
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn assign_group(&mut self, group: &str) {
        self.group = Some(group.to_string())
    }
}
