use crate::core::{
    parsing::tokenizer::{Token, TokenStream},
    traits::NoteInfos,
};

#[derive(Debug, Clone, PartialEq)]
pub struct NoteLabel {
    pub name: Option<String>,
    pub group: Option<String>,
}

impl NoteInfos for NoteLabel {
    fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
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
            Some(Token::OpenPar) => {
                if let Some(name) = self.parse_expr(Token::ClosePar) {
                    return Some(NoteLabel {
                        name: Some(name),
                        group: None,
                    });
                } else {
                    return None;
                }
            }
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
                        return if !result.is_empty() {
                            Some(result)
                        } else {
                            None
                        };
                    }
                    break;
                }
            }
        }
        None
    }
}

//TODO: write somes test for parsing
#[cfg(test)]
mod parser_test {
    use crate::core::parsing::tokenizer::Tokenizer;

    #[test]
    fn test_correct_input() {
        let content = "[group(name)]";
        let mut stream = Tokenizer::load(content);

        let res = stream.parse().unwrap(); // should never panick
        let name = res.name.unwrap(); // should never panick
        let group = res.group.unwrap(); // should never panick

        assert_eq!(group, "group");
        assert_eq!(name, "name")
    }

    #[test]
    fn test_only_group() {
        let content = "[group]";

        let mut stream = Tokenizer::load(content);

        let res = stream.parse().unwrap(); // should never panick
        let group = res.group.unwrap(); // should never panick
        let name = res.name;

        assert_eq!(name, None);
        assert_eq!(group, "group");
    }

    #[test]
    fn test_only_group_empty_name() {
        let content = "[group]()";

        let mut stream = Tokenizer::load(content);

        let res = stream.parse().unwrap(); // should never panick
        let group = res.group.unwrap(); // should never panick
        let name = res.name;

        assert_eq!(name, None);
        assert_eq!(group, "group");
    }

    #[test]
    fn test_only_name() {
        let content = "(name)";

        let mut stream = Tokenizer::load(content);

        let res = stream.parse().unwrap(); // should never panick
        let name = res.name.unwrap(); // should never panick
        let group = res.group;

        assert_eq!(group, None);
        assert_eq!(name, "name");
    }
}

#[cfg(test)]
mod invalid_parser_test {
    use crate::core::parsing::tokenizer::Tokenizer;

    #[test]
    fn test_invalid_par_sequence() {
        let content = "((name)";
        let mut stream = Tokenizer::load(content);
        let res = stream.parse(); // should never panick

        assert_eq!(res, None);
    }

    #[test]
    fn test_invalid_bra_sequence() {
        let content = "[[group(name)]";
        let mut stream = Tokenizer::load(content);
        let res = stream.parse(); // should never panick

        assert_eq!(res, None);
    }
}
