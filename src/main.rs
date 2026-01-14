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
    let content = std::fs::read_to_string(note_path)?;
    let _group = Interpreter::define_path(&content)?;

    Ok(())
}
