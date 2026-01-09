use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Config {
    pub editor: PathBuf,
    pub note_dir: PathBuf,
}

const DEFAULT_EDITOR: &str = "/usr/bin/vim";
const DEFAULT_NOTES_DIR: &str = ".notes";

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let home =
            std::env::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to fetch home dir"))?;
        Ok(Config {
            editor: PathBuf::from("/usr/bin/vim"),
            note_dir: home.join(".notes"),
        })
    }

    pub fn from(editor: PathBuf, note_dir: PathBuf) -> Self {
        Config { editor, note_dir }
    }
}

fn check_perm(p: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(p)
        && metadata.is_file()
    {
        let perm = metadata.permissions();
        let mode = perm.mode();

        return mode & 0o111 != 0;
    }

    false
}

/// TODO: this function do so much useless convertion. Refactor it
fn default_notes_dir() -> anyhow::Result<String> {
    let home = std::env::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to fetch home dir"))?;

    Ok(home.join(DEFAULT_NOTES_DIR).to_string_lossy().to_string())
}

pub fn load_config() -> anyhow::Result<Config> {
    let editor = std::env::var("EDITOR").unwrap_or(DEFAULT_EDITOR.to_string());
    let notes_dir = std::env::var("NOTE_DIR").unwrap_or(default_notes_dir()?);

    let editor_path = PathBuf::from(editor);
    if !check_perm(&editor_path) {
        anyhow::bail!("$EDITOR var conatain non executable path please check.")
    }

    let notes_path = PathBuf::from(notes_dir);

    Ok(Config::from(editor_path, notes_path))
}

#[cfg(test)]
#[cfg(unix)]
mod config_tests {
    use std::fs::{self, File};
    use std::os::unix::fs::PermissionsExt;

    use crate::config::check_perm;
    #[test]
    fn check_perm_detects_executable_file() {
        let mut path = std::env::temp_dir();
        path.push("check_perm_noexec_test");

        File::create(&path).expect("failed to create temp file");

        let perms = fs::Permissions::from_mode(0o755);
        fs::set_permissions(&path, perms).expect("failed to set permissions");

        assert!(check_perm(&path));

        fs::remove_file(&path).ok();
    }

    #[test]
    fn check_perm_detects_non_executable_file() {
        let mut path = std::env::temp_dir();
        path.push("check_perm_exec_test");

        File::create(&path).expect("failed to create temp file");

        let perms = fs::Permissions::from_mode(0o644);
        fs::set_permissions(&path, perms).expect("failed to set permissions");

        assert!(!check_perm(&path));

        fs::remove_file(&path).ok();
    }
}
