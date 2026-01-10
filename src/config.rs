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

pub const DEFAULT_EDITOR: &str = "/usr/bin/vim";
pub const DEFAULT_NOTES_DIR: &str = ".bucket";

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

/// This function return default bucket path ('~/.bucket') by resolving
/// home directory
/// The function take one oprtional argument that represent a path extension
/// e.g name="rust" -> '~/.bucket/rust'
pub fn default_bucket_path(name: Option<&str>) -> anyhow::Result<PathBuf> {
    let home = std::env::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to fetch home dir"))?;
    if let Some(name) = name {
        Ok(home.join(DEFAULT_NOTES_DIR).join(name))
    } else {
        Ok(home.join(DEFAULT_NOTES_DIR))
    }
}

/// This function return the config from environement variables
/// in a [`Config`] structure.
pub fn load_config() -> anyhow::Result<Config> {
    let editor = if let Ok(editor) = std::env::var("EDITOR") {
        PathBuf::from(editor)
    } else {
        PathBuf::from(DEFAULT_EDITOR)
    };

    let notes_dir = if let Ok(notes_dir) = std::env::var("NOTE_DIR") {
        PathBuf::from(notes_dir)
    } else {
        default_bucket_path(None)?
    };

    if !check_perm(&editor) {
        anyhow::bail!("$EDITOR var conatain non executable path please check.")
    }

    Ok(Config::from(editor, notes_dir))
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
