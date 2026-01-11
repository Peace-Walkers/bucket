use std::path::PathBuf;

use crate::core::parsing::tokenizer::Tokenizer;

pub struct Interpretor {}

impl Interpretor {
    pub fn define_path(content: &str) -> anyhow::Result<PathBuf> {
        let lines: Vec<&str> = content.lines().collect();

        let group = if !lines.is_empty() {
            Tokenizer::load(lines[0]).parse()
        } else {
            None
        };

        dbg!(group);
        todo!()
    }
}
