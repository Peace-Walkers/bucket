use bucket::{Args, config, core::storage::Storage, system};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = config::load_config()?;

    let groups = Storage::load_groups(&config.note_dir)?;

    dbg!(&args);
    dbg!(&config);
    dbg!(groups);

    system::editor::open_editor(&config, &args)?;
    Ok(())
}
