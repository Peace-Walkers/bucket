pub struct Tokenizer {}

pub enum Token {
    OpenBracket,
    CloseBracket,
    Char(char),
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '[' => Self::OpenBracket,
            ']' => Self::CloseBracket,
            other => Self::Char(other),
        }
    }
}

impl Tokenizer {
    pub fn load(content: &str) -> Vec<Token> {
        content.chars().map(|c| Token::from(c)).collect()
    }
}
