use std::process::Command;

use crate::{Args, config::Config, core::storage::Storage};

/// This function's sole purpose is to run the editor and ensure that
/// the output status is correct.
pub fn open_editor(config: &Config, args: &Args) -> anyhow::Result<()> {
    let editor_path = &config.editor;

    let note_path = Storage::get_note_path(args)?;
    let status = Command::new(editor_path).arg(note_path).status()?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    Ok(())
}
