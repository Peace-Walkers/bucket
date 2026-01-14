use std::path::PathBuf;

use crate::core::{parsing::tokenizer::Tokenizer, storage::Storage};

pub struct Interpreter {}

impl Interpreter {
    pub fn define_path(content: &str) -> anyhow::Result<Option<PathBuf>> {
        let lines: Vec<&str> = content.lines().collect();

        let note_infos = if !lines.is_empty() {
            //TODO: what if group name is already provided by cli ?
            Tokenizer::load(lines[0]).parse()
        } else {
            None
        };

        dbg!(&note_infos);
        if let Some(infos) = note_infos {
            Ok(Some(Storage::get_note_path(&infos)?))
        } else {
            Ok(None)
        }
    }
}
