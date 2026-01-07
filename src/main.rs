use bucket::config;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub groups: Option<Vec<String>>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = config::load_config()?;

    dbg!(config);

    dbg!(args);
    Ok(())
}
