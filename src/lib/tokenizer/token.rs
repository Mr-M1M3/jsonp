use crate::lib::errors::DeserializationError;
use crate::lib::errors::DeserializationError::UNEXPECTED_TOKEN;
use crate::lib::tokenizer::TokenPosition;

#[derive(PartialEq, Debug)]

pub enum Token {
    LeftBrace { pos: TokenPosition },
    RightBrace { pos: TokenPosition },
    LeftBracket { pos: TokenPosition },
    RightBracket { pos: TokenPosition },
    DoubleQuote { pos: TokenPosition },
    Colon { pos: TokenPosition },
    Comma { pos: TokenPosition },
    Str { pos: TokenPosition, val: String },
    Number { pos: TokenPosition, val: f64 },
    Boolean { pos: TokenPosition, val: bool },
    Null { pos: TokenPosition },
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftBrace { pos } => write!(f, "{{ at {}", pos),
            Self::RightBrace { pos } => write!(f, "}} at {}", pos),
            Self::LeftBracket { pos } => write!(f, "[ at {}", pos),
            Self::RightBracket { pos } => write!(f, "] at {}", pos),
            Self::DoubleQuote { pos } => write!(f, "\" at {}", pos),
            Self::Colon { pos } => write!(f, ": at {}", pos),
            Self::Comma { pos } => write!(f, ", at {}", pos),
            Self::Str { pos, val } => write!(f, " string: {} at {}", val, pos),
            Self::Number { pos, val } => write!(f, " number: {} at {}", val, pos),
            Self::Boolean { pos, val } => write!(f, " boolean: {} at {}", val, pos),
            Self::Null { pos } => write!(f, "null at {}", pos),
        }
    }
}
impl Token {
    pub fn is_left_brace(&self) -> bool {
        match self {
            Self::LeftBrace { pos: _ } => true,
            _ => false,
        }
    }

    pub fn expect_left_brace(&self) -> Result<&Self, DeserializationError> {
        if self.is_left_brace() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }

    pub fn is_right_brace(&self) -> bool {
        match self {
            Self::RightBrace { pos: _ } => true,
            _ => false,
        }
    }

    pub fn expect_right_brace(&self) -> Result<&Self, DeserializationError> {
        if self.is_right_brace() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }

    pub fn is_left_bracket(&self) -> bool {
        match self {
            Self::LeftBracket { pos: _ } => true,
            _ => false,
        }
    }

    pub fn expect_left_bracket(&self) -> Result<&Self, DeserializationError> {
        if self.is_left_bracket() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }

    pub fn is_right_bracket(&self) -> bool {
        match self {
            Self::RightBracket { pos: _ } => true,
            _ => false,
        }
    }

    pub fn expect_right_bracket(&self) -> Result<&Self, DeserializationError> {
        if self.is_right_bracket() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }


    pub fn is_double_quote(&self) -> bool {
        match self {
            Self::DoubleQuote { pos: _ } => true,
            _ => false,
        }
    }

    pub fn expect_double_quote(&self) -> Result<&Self, DeserializationError> {
        if self.is_double_quote() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }

    pub fn is_colon(&self) -> bool {
        match self {
            Self::Colon { pos: _ } => true,
            _ => false,
        }
    }

    pub fn expect_colon(&self) -> Result<&Self, DeserializationError> {
        if self.is_colon() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }

    pub fn is_comma(&self) -> bool {
        match self {
            Self::Comma { pos: _ } => true,
            _ => false,
        }
    }

    pub fn expect_comma(&self) -> Result<&Self, DeserializationError> {
        if self.is_comma() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Self::Str { pos: _, val: _ } => true,
            _ => false,
        }
    }

    pub fn expect_string(&self) -> Result<&Self, DeserializationError> {
        if self.is_string() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Self::Number { pos: _, val: _ } => true,
            _ => false,
        }
    }

    pub fn expect_number(&self) -> Result<&Self, DeserializationError> {
        if self.is_number() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean { pos: _, val: _ } => true,
            _ => false,
        }
    }

    pub fn expect_boolean(&self) -> Result<&Self, DeserializationError> {
        if self.is_boolean() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Self::Null { pos: _ } => true,
            _ => false,
        }
    }

    pub fn expect_null(&self) -> Result<&Self, DeserializationError> {
        if self.is_null() {
            return Ok(self);
        } else {
            return Err(UNEXPECTED_TOKEN(format!(
                "unexpected token {}",
                self.to_string()
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::tokenizer::TokenPosition;

    use super::Token;

    #[test]
    fn token_enum_expect_l_brace() {
        let token = Token::LeftBrace {
            pos: TokenPosition::origin(),
        };
        assert_eq!(
            token.expect_left_brace(),
            Ok(&Token::LeftBrace {
                pos: TokenPosition::origin()
            })
        );
    }
    // TODO: Add more test cases
}
