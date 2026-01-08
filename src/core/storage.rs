use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::core::{group::Group, notes::Note};

pub struct Storage {
    root: PathBuf,
}

//NOTE: This group of functions (load_groups, load_notes) currently only allows
//      searching one level of folder, which does not allow managing subgroups.
fn load_notes(group_path: &Path) -> anyhow::Result<Vec<Note>> {
    let mut notes = vec![];
    let group_name = group_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string());

    for entry in fs::read_dir(group_path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let note = Note::new(entry.path(), group_name.clone())?;
            notes.push(note);
        }
    }
    Ok(notes)
}

impl Storage {
    pub fn load_groups(&self) -> anyhow::Result<Vec<Group>> {
        let mut groups = vec![];

        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let notes = load_notes(&entry.path())?;
                let group = Group::new(&entry.file_name().to_string_lossy(), Some(notes));
                groups.push(group);
            }
        }
        Ok(groups)
    }
}
