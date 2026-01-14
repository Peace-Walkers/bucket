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

#[cfg(test)]
mod tokenizer_test {
    macro_rules! tokens {
    ($($t:expr),* $(,)?) => {
        vec![$($t),*]
    };
}
    use crate::core::parsing::tokenizer::{Token, Tokenizer};

    #[test]
    fn test_correct_content() {
        let content = "[group(name)]";

        let stream = Tokenizer::load(content);

        assert_eq!(
            stream.tokens(),
            tokens![
                Token::OpenBracket,
                Token::Char('g'),
                Token::Char('r'),
                Token::Char('o'),
                Token::Char('u'),
                Token::Char('p'),
                Token::OpenPar,
                Token::Char('n'),
                Token::Char('a'),
                Token::Char('m'),
                Token::Char('e'),
                Token::ClosePar,
                Token::CloseBracket,
            ]
        );
    }
}
