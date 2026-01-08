use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

pub struct Note {
    path: PathBuf,
    name: String,
    group: Option<String>,
    mtime: std::time::SystemTime,
}

impl Note {
    pub fn new(path: PathBuf, group: Option<String>) -> anyhow::Result<Self> {
        let meta = fs::metadata(&path)?;
        let mtime = meta.modified()?;
        let name = path
            .file_name()
            .unwrap_or(OsStr::new("unknown"))
            .to_string_lossy()
            .to_string();

        Ok(Self {
            path,
            name,
            group,
            mtime,
        })
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
