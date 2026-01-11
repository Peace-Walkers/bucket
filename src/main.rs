use bucket::{
    Args, config,
    core::{interpretor::Interpretor, storage::Storage},
    system::{self, editor::SytemEditor},
};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = config::load_config()?;

    let groups = Storage::load_groups(&config.note_dir)?;

    dbg!(&args);
    dbg!(&config);
    dbg!(groups);

    let editor = SytemEditor;
    let note_path = system::editor::open_editor(&editor, &config, &args)?;
    let content = std::fs::read_to_string(note_path)?;
    let _group = Interpretor::define_path(&content)?;

    Ok(())
}
