use std::collections::HashMap;

use crate::lib::token::Token;
use crate::lib::deserialized::Deserialized;
use crate::lib::errors::DeserializationError;
use crate::lib::errors::DeserializationError::{UNEXPECTED_EOF, UNEXPECTED_TOKEN};

pub struct Parser {
    input: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            input: tokens,
            cursor: 0,
        }
    }
    fn advance_cursor(&mut self) {
        if self.cursor == self.input.len() {
            ()
        } else {
            self.cursor += 1;
        }
    }
    fn next_token(&mut self) -> Option<&Token> {
        if let Some(t) = self.input.get(self.cursor + 1) {
            self.cursor += 1;
            Some(t)
        } else {
            None
        }
    }
    fn curr_token(&self) -> &Token {
        self.input.get(self.cursor).unwrap()
    }
    fn peek_next(&self) -> Option<&Token> {
        self.input.get(self.cursor + 1)
    }
    fn expect_curr(&self, expected: &Token) -> Result<&Token, DeserializationError> {
        match self.input.get(self.cursor) {
            Some(t) => {
                return t.expect(expected);
            }
            None => return Err(UNEXPECTED_EOF),
        }
    }

    fn expect_next(&self, expected: &Token) -> Result<&Token, DeserializationError> {
        match self.input.get(self.cursor + 1) {
            Some(t) => {
                return t.expect(expected);
            }
            None => return Err(UNEXPECTED_EOF),
        }
    }

    pub fn parse(&mut self) -> Result<Deserialized, DeserializationError> {
        match self.curr_token() {
            Token::DoubleQuote => match self.next_token() {
                Some(Token::Str(v)) => {
                    let str = v.clone();
                    self.expect_next(&Token::DoubleQuote)?;
                    self.next_token();
                    return Ok(Deserialized::Str(str));
                }
                Some(t) => {
                    println!("ummm");
                    return Err(UNEXPECTED_TOKEN(format!(
                        "expected 'string' found {}",
                        t.str_represenation()
                    )));
                }
                None => {
                    return Err(UNEXPECTED_EOF);
                }
            },
            Token::Number(v) => {
                return Ok(Deserialized::Number(*v));
            }

            Token::Boolean(v) => {
                return Ok(Deserialized::Boolean(*v));
            }

            Token::Null => {
                return Ok(Deserialized::Null);
            }
            Token::LeftBrace => {
                if let Some(Token::RightBrace) = self.peek_next() {
                    return Ok(Deserialized::Map(HashMap::new()));
                }
                let mut data: HashMap<String, Deserialized> = HashMap::new();
                loop {
                    self.expect_next(&Token::DoubleQuote)?;
                    self.next_token();
                    let key = match self.next_token() {
                        Some(Token::Str(k)) => k.clone(),
                        Some(other) => {
                            return Err(UNEXPECTED_TOKEN(format!(
                                "expected 'string' found {}",
                                other.str_represenation()
                            )));
                        }
                        None => {
                            return Err(UNEXPECTED_EOF);
                        }
                    };
                    self.expect_next(&Token::DoubleQuote)?;
                    self.next_token();
                    self.expect_next(&Token::Colon)?;
                    self.next_token();
                    self.next_token();
                    println!("curr token: {:?}", self.curr_token());
                    println!("next token: {:?}", self.peek_next());
                    let val = self.parse()?;
                    data.insert(key, val);
                    match self.next_token() {
                        Some(Token::Comma) => {
                            continue;
                        }
                        Some(Token::RightBrace) => {
                            return Ok(Deserialized::Map(data));
                        }
                        Some(t) => {
                            return Err(UNEXPECTED_TOKEN(format!(
                                "expected ',' or '}}' found: {}",
                                t.str_represenation()
                            )));
                        }
                        None => {
                            return Err(UNEXPECTED_EOF);
                        }
                    }
                }
            }
            Token::LeftBracket => {
                todo!();
            }
            t => {
                return Err(UNEXPECTED_TOKEN(format!(
                    "expected any of '{{', '[' and json primitive, found {}",
                    t.str_represenation()
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Parser;
    use super::Token::{
        Boolean, Colon, Comma, DoubleQuote, LeftBrace, Null, Number, RightBrace, Str,
    };
    use super::Deserialized;
    use super::DeserializationError;
    #[test]
    fn only_sring() {
        let tokens = vec![
            DoubleQuote,
            Str("ungabunga".into()),
            DoubleQuote,
        ];
        let parsed = Parser::new(tokens).parse();
        println!("{:?}", parsed);
        assert_eq!(parsed, Ok(Deserialized::Str("ungabunga".into())));
    }

    #[test]
    fn only_number() {
        let tokens = vec![Number(600f64)];

        let parsed = Parser::new(tokens).parse();
        println!("{:?}", parsed);
        assert_eq!(parsed, Ok(Deserialized::Number(600f64)));
    }

    #[test]
    fn only_boolean() {
        let tokens = vec![Boolean(true)];

        let parsed = Parser::new(tokens).parse();
        println!("{:?}", parsed);
        assert_eq!(parsed, Ok(Deserialized::Boolean(true)));
    }

    #[test]
    fn only_null() {
        let tokens = vec![Null];

        let parsed = Parser::new(tokens).parse();
        println!("{:?}", parsed);
        assert_eq!(parsed, Ok(Deserialized::Null));
    }

    #[test]
    fn only_l_brace() {
        let tokens = vec![LeftBrace];

        let parsed = Parser::new(tokens).parse();
        println!("{:?}", parsed);
        assert_eq!(parsed, Err(DeserializationError::UNEXPECTED_EOF));
    }

    #[test]
    fn valid_obj() {
        let tokens = vec![
            LeftBrace,
            DoubleQuote,
            Str("key".into()),
            DoubleQuote,
            Colon,
            Number(12f64),
            Comma,
            DoubleQuote,
            Str("key2".into()),
            DoubleQuote,
            Colon,
            Boolean(true),
            RightBrace,
        ];

        let parsed = Parser::new(tokens).parse();
        println!("{:?}", parsed);
        let map = Deserialized::Map(HashMap::from([
            ("key".to_string(), Deserialized::Number(12f64)),
            ("key2".to_string(), Deserialized::Boolean(true))
        ]));
        assert_eq!(parsed, Ok(map));
    }
}
