use crate::core::parsing::tokenizer::{Token, TokenStream};

#[derive(Debug)]
pub struct NoteLabel {
    pub name: Option<String>,
    pub group: Option<String>,
}

impl TokenStream {
    /// This function look the first line of the file into token list
    /// if the line start with '[something]' this will be interpreted
    /// as group name for this note
    pub fn parse(&mut self) -> Option<NoteLabel> {
        let mut group_name = String::new();
        let mut note_name = None;

        match self.next() {
            Some(Token::OpenBracket) => {}
            _ => return None,
        }

        while let Some(token) = self.next() {
            match token {
                Token::OpenPar => note_name = self.parse_expr(Token::ClosePar),
                Token::CloseBracket => {
                    return Some(NoteLabel {
                        name: note_name,
                        group: Some(group_name),
                    });
                }
                Token::Char(c) => group_name.push(*c),
                Token::OpenBracket => return None,
                Token::ClosePar => return None,
            }
        }

        None
    }

    fn parse_expr(&mut self, end: Token) -> Option<String> {
        let mut result = String::new();
        while let Some(token) = self.next() {
            match token {
                Token::Char(c) => result.push(*c),
                other => {
                    if end == *other {
                        return Some(result);
                    }
                    break;
                }
            }
        }
        None
    }
}

//TODO: write somes test for parsing
