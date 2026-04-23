use crate::lib::errors::DeserializationError;
use crate::lib::errors::DeserializationError::{UNEXPECTED_TOKEN};

#[derive(PartialEq, Debug)]

pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    DoubleQuote,
    Colon,
    Comma,
    Str(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl Token {
    pub fn expect(&self, expected: &Token) -> Result<&Self, DeserializationError> {
        let is_same_variant = match (self, expected) {
            (Self::Str(_), Self::Str(_)) => true,
            (Self::Number(_), Self::Number(_)) => true,
            (l, r) => l == r,
        };
        if is_same_variant {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token: expected '{}', found '{}'",
                expected.str_represenation(),
                self.str_represenation()
            )));
        }
    }

    pub fn str_represenation(&self) -> &'static str {
        match self {
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::LeftBracket => "[",
            Self::RightBracket => "]",
            Self::DoubleQuote => "\"",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Str(_) => "string",
            Self::Number(_) => "number",
            Self::Boolean(_) => "boolean",
            Self::Null => "null",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn expect_l_brace() {
        let token = Token::LeftBrace;
        assert_eq!(token.expect(&Token::LeftBrace), Ok(&Token::LeftBrace));
    }
    // TODO: Add more test cases
}
