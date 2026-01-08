use std::{fs, path::Path};

use crate::core::{group::Group, notes::Note};

pub struct Storage {
    // root: PathBuf,
}

impl Storage {
    pub fn load_groups(root: &Path) -> anyhow::Result<Vec<Group>> {
        let mut groups = vec![];

        for entry in fs::read_dir(root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let notes = Storage::load_notes(&entry.path())?;
                let group = Group::new(&entry.file_name().to_string_lossy(), Some(notes));
                groups.push(group);
            }
        }
        Ok(groups)
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
}

#[cfg(test)]
mod storage_loading {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_load_single_group() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let group_dir = dir.path().join("rust"); // group
        fs::create_dir(&group_dir)?;

        let note_path = group_dir.join("borrowing.bck");
        File::create(&note_path)?;

        let groups = Storage::load_groups(dir.path())?;

        assert_eq!(groups.len(), 1);
        let g = &groups[0];
        assert_eq!(g.notes().len(), 1);
        assert_eq!(g.name(), "rust");
        assert_eq!(g.notes()[0].name(), "borrowing.bck");
        Ok(())
    }

    #[test]
    fn test_load_multiple_group() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let a_group_dir = dir.path().join("go"); // group
        let b_group_dir = dir.path().join("python"); // group
        let c_group_dir = dir.path().join("rust"); // group

        fs::create_dir(&a_group_dir)?;
        fs::create_dir(&b_group_dir)?;
        fs::create_dir(&c_group_dir)?;

        let a_note_path = a_group_dir.join("borrowing.bck");
        let b_note_path = b_group_dir.join("borrowing.bck");
        let c_note_path = c_group_dir.join("borrowing.bck");

        File::create(&a_note_path)?;
        File::create(&b_note_path)?;
        File::create(&c_note_path)?;

        let mut groups = Storage::load_groups(dir.path())?;
        groups.sort_by(|a, b| a.name().cmp(b.name())); // sort by name

        assert_eq!(groups.len(), 3);
        let a = &groups[0];
        assert_eq!(a.notes().len(), 1);
        assert_eq!(a.name(), "go");
        assert_eq!(a.notes()[0].name(), "borrowing.bck");
        let b = &groups[1];
        assert_eq!(b.notes().len(), 1);
        assert_eq!(b.name(), "python");
        assert_eq!(b.notes()[0].name(), "borrowing.bck");
        let c = &groups[2];
        assert_eq!(c.notes().len(), 1);
        assert_eq!(c.name(), "rust");
        assert_eq!(c.notes()[0].name(), "borrowing.bck");
        Ok(())
    }
}
