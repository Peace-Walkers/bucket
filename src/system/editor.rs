use std::process::Command;

use crate::config::Config;

//TODO: take filename: Option<String> to represent note name
//      and add an incremental default value

/// This function's sole purpose is to run the editor and ensure that
/// the output status is correct.
pub fn open_editor(config: &Config) -> anyhow::Result<()> {
    let editor_path = &config.editor;

    let status = Command::new(editor_path).status()?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    Ok(())
}
