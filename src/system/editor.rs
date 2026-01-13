use std::{path::PathBuf, process::Command};

use crate::{Args, config::Config, core::storage::Storage, system::traits::Editor};

pub struct SytemEditor;

impl Editor for SytemEditor {
    fn open(&self, editor: &std::path::Path, file: &std::path::Path) -> anyhow::Result<()> {
        let status = Command::new(editor).arg(file).status()?;

        if !status.success() {
            anyhow::bail!("Editor exited with non-zero status");
        }

        Ok(())
    }
}

/// This function's sole purpose is to run the editor and ensure that
/// the output status is correct.
pub fn open_editor<E: Editor>(editor: &E, config: &Config, args: &Args) -> anyhow::Result<PathBuf> {
    let note_path = Storage::get_note_path(args)?;
    dbg!(&note_path);
    editor.open(&config.editor, &note_path)?;
    Ok(note_path)
}
