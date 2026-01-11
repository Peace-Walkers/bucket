use crate::core::parsing::tokenizer::{Token, TokenStream};

impl TokenStream {
    /// This function look the first line of the file into token list
    /// if the line start with '[something]' this will be interpreted
    /// as group name for this note
    pub fn parse(mut self) -> Option<String> {
        let mut group_name = String::new();

        match self.next() {
            Some(Token::OpenBracket) => {}
            _ => return None,
        }

        while let Some(token) = self.next() {
            match token {
                Token::CloseBracket => return Some(group_name),
                Token::Char(c) => group_name.push(*c),
                Token::OpenBracket => return None,
            }
        }

        None
    }
}
