use std::path::PathBuf;

pub struct Note {
    pub path: PathBuf,
    pub name: String,
    pub group: Option<String>,
    pub mtime: std::time::SystemTime,
}
