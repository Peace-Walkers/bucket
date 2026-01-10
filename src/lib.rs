pub mod config;
pub mod core;
pub mod system;

use clap::Parser;
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub groups: Option<Vec<String>>,

    #[arg(short, long)]
    pub name: Option<String>,
}
