use std::{fs, path::Path};

use bucket::{
    cli::Args,
    config,
    core::{interpreter::Interpreter, storage::Storage},
    system::{self, editor::SystemEditor},
};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = config::load_config()?;

    let groups = Storage::load_groups(&config.note_dir)?;

    dbg!(&args);
    dbg!(&config);
    dbg!(groups);

    let editor = SystemEditor;
    let note_path = system::editor::open_editor(&editor, &config, &args)?;

    let content = std::fs::read_to_string(&note_path)?;
    let interpreted_path = Interpreter::define_path(&content)?;

    dbg!(&note_path);
    dbg!(&interpreted_path);

    if let Some(path) = interpreted_path {
        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent)?;
        }

        std::fs::rename(note_path, path)?;
    }

    Ok(())
}
