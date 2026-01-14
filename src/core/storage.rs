use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    config,
    core::{group::Group, notes::Note, traits::NoteInfos},
};

pub struct Storage {
    // root: PathBuf,
}

const DEFAULT_TMP_DIR: &str = "tmp";

impl Storage {
    pub fn load_groups(root: &Path) -> anyhow::Result<Vec<Group>> {
        if !root.exists() {
            // If the $NOTE_DIR path do not exist we create it
            fs::create_dir(root)?;
        }

        // We load root orphan notes first (not really orphan, their parent being 'root')
        let notes = Storage::load_notes(root)
            .map_err(|e| anyhow::anyhow!("Cannot open notes dir: \n\t{e}"))?;
        let group = Group::new("all", Some(notes));
        let mut groups = vec![group];

        // For each dir in 'root' we load all its notes
        // and store them in a group named after the directory
        for entry in
            fs::read_dir(root).map_err(|e| anyhow::anyhow!("Cannot open notes dir : {e}"))?
        {
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

    /// This function return the next index for tmp note name
    /// e.g:
    /// ```bash
    /// > ls ~/.bucket/tmp
    /// tmp_1.bck tmp_2.bck
    /// ```
    /// in this exemple the function return will be Ok(3)
    pub fn find_next_incremental_note() -> anyhow::Result<usize> {
        let bucket_tmp_path = config::default_bucket_path(Some(DEFAULT_TMP_DIR))?;
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
        Ok(last)
    }

    //TODO: this function need refactor to return PathBuf instead of
    //      String & improve format & parsing logic
    fn assign_default_path() -> anyhow::Result<String> {
        let next = Storage::find_next_incremental_note()?;

        let name = format!("{}/tmp_{}.bck", DEFAULT_TMP_DIR, next + 1);
        Ok(name)
    }

    /// This function resolves the note's path by creating a new path.
    /// There are three possible cases:
    /// - The name and group are provided -> we concatenate the two and add the default bucket path ('~/.bucket') at the beginning.
    /// - The name is not provided -> we simply define an incremental filename and concatenate it to the default bucket path.
    /// - The name is provided but not the group -> here the same logic applies: we concatenate the default bucket path and the name.
    pub fn get_note_path<N>(args: &N) -> anyhow::Result<PathBuf>
    where
        N: NoteInfos,
    {
        let note_name = args.name();
        let note_group = args.group();
        let filename = if let Some(name) = &note_name {
            let n = name;
            if let Some(group) = &note_group {
                format!("{group}/{n}")
            } else {
                n.to_string()
            }
        } else if let Some(group) = &note_group {
            group.to_string()
        } else {
            Self::assign_default_path()?
        };

        config::default_bucket_path(Some(&filename))
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

        assert_eq!(groups.len(), 2);
        let g = &groups[1];
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

        assert_eq!(groups.len(), 4);
        let all = &groups[0];
        assert_eq!(all.notes().len(), 0);
        assert_eq!(all.name(), "all");

        let a = &groups[1];
        assert_eq!(a.notes().len(), 1);
        assert_eq!(a.name(), "go");
        assert_eq!(a.notes()[0].name(), "borrowing.bck");
        let b = &groups[2];
        assert_eq!(b.notes().len(), 1);
        assert_eq!(b.name(), "python");
        assert_eq!(b.notes()[0].name(), "borrowing.bck");
        let c = &groups[3];
        assert_eq!(c.notes().len(), 1);
        assert_eq!(c.name(), "rust");
        assert_eq!(c.notes()[0].name(), "borrowing.bck");
        Ok(())
    }

    #[test]
    fn test_no_group_only_notes() -> anyhow::Result<()> {
        let dir = tempdir()?;

        let a_note_path = dir.path().join("borrowing.bck");
        let b_note_path = dir.path().join("pool.bck");
        let c_note_path = dir.path().join("hello.bck");
        File::create(&a_note_path)?;
        File::create(&b_note_path)?;
        File::create(&c_note_path)?;

        let mut groups = Storage::load_groups(dir.path())?;

        // dbg!(groups);
        assert_eq!(groups.len(), 1);
        let g = &mut groups[0];
        assert_eq!(g.notes().len(), 3);
        Ok(())
    }
}
