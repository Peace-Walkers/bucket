use std::{path::PathBuf, process::Command};

use crate::{
    Args,
    config::{Config, default_bucket_path},
};

const DEFAULT_TMP_DIR: &str = "tmp";

fn assign_default_path() -> anyhow::Result<String> {
    let bucket_tmp_path = PathBuf::from(default_bucket_path(Some(DEFAULT_TMP_DIR))?);
    if !bucket_tmp_path.exists() {
        std::fs::create_dir_all(&bucket_tmp_path)?;
    }

    let mut last = 0;

    for entry in std::fs::read_dir(bucket_tmp_path)? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let mut name = entry.file_name().into_string().map_err(|_| {
                anyhow::anyhow!("Invalid tmp file name format in {}", entry.path().display())
            })?;

            if let Some(dot) = name.find('.') {
                name = name.drain(..dot).collect();
            } else {
                anyhow::bail!("Invalid tmp file name format in {}", entry.path().display());
            }

            let parts: Vec<&str> = name.split('_').collect();
            if parts.len() != 2 {
                anyhow::bail!("Invalid tmp file name format in {}", entry.path().display());
            }

            let file_n = parts[1].parse::<usize>().map_err(|_| {
                anyhow::anyhow!("Invalid tmp file name format in {}", entry.path().display())
            })?;
            last = if file_n > last { file_n } else { last };
        }
    }

    let name = format!("{}/tmp_{}.bck", DEFAULT_TMP_DIR, last + 1);
    Ok(name)
}

/// This function resolves the note's path by creating a new path.
/// There are three possible cases:
/// - The name and group are provided -> we concatenate the two and add the default bucket path ('~/.bucket') at the beginning.
/// - The name is not provided -> we simply define an incremental filename and concatenate it to the default bucket path.
/// - The name is provided but not the group -> here the same logic applies: we concatenate the default bucket path and the name.
fn get_note_path(args: &Args) -> anyhow::Result<String> {
    let filename = if let Some(name) = &args.name {
        let n = name.clone();
        if let Some(group) = &args.groups {
            let group_name = &group[0];
            format!("{group_name}/{n}")
        } else {
            n
        }
    } else {
        assign_default_path()?
    };

    dbg!(&filename);
    let file_path = default_bucket_path(Some(&filename))?;

    dbg!(&file_path);
    Ok(file_path)
}

/// This function's sole purpose is to run the editor and ensure that
/// the output status is correct.
pub fn open_editor(config: &Config, args: &Args) -> anyhow::Result<()> {
    let editor_path = &config.editor;

    let note_path = get_note_path(args)?;
    let status = Command::new(editor_path).arg(note_path).status()?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    Ok(())
}
