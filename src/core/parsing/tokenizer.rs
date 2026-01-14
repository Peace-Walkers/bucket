pub struct Tokenizer {}

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    OpenPar,
    ClosePar,
    Char(char),
}

pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '[' => Self::OpenBracket,
            ']' => Self::CloseBracket,
            '(' => Self::OpenPar,
            ')' => Self::ClosePar,
            other => Self::Char(other),
        }
    }
}

impl Tokenizer {
    /// This function take the first line of the note
    /// as &str and tokenize it into list of [`Token`] and return a [`TokenStream`]
    /// ready to be parsed
    pub fn load(content: &str) -> TokenStream {
        let tokens = content.chars().map(Token::from).collect();

        TokenStream { tokens, pos: 0 }
    }
}
