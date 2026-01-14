impl NoteInfos for Args {
    fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

use clap::Parser;

use crate::core::traits::NoteInfos;
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub group: Option<String>,

    #[arg(short, long)]
    pub name: Option<String>,
}
