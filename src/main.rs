use bucket::{config, core::storage::Storage, system};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub groups: Option<Vec<String>>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = config::load_config()?;

    let groups = Storage::load_groups(&config.note_dir)?;

    dbg!(args);
    dbg!(&config);
    dbg!(groups);

    system::editor::open_editor(&config)?;
    Ok(())
}
