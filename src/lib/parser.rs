use std::collections::HashMap;

use crate::lib::deserialized::Deserialized;
use crate::lib::errors::DeserializationError;
use crate::lib::errors::DeserializationError::{UNEXPECTED_EOF, UNEXPECTED_TOKEN};
use crate::lib::token::Token;

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
    fn parse_literal_val(&mut self) -> Result<Deserialized, DeserializationError> {
        match self.curr_token() {
            Token::DoubleQuote => match self.next_token() {
                Some(Token::Str(v)) => {
                    let str = v.clone();
                    self.expect_next(&Token::DoubleQuote)?;
                    self.advance_cursor();
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
                return self.parse_obj();
            }
            Token::LeftBracket => {
                return self.parse_array();
            }
            t => {
                return Err(UNEXPECTED_TOKEN(format!(
                    "expected any of '{{', '[' and json primitive, found {}",
                    t.str_represenation()
                )));
            }
        }
    }
    fn parse_obj(&mut self) -> Result<Deserialized, DeserializationError> {
        if !(self.curr_token() == &Token::LeftBrace) {
            return Err(UNEXPECTED_TOKEN("expected '{'".into()));
        }
        if let Some(Token::RightBrace) = self.peek_next() {
            self.advance_cursor();
            return Ok(Deserialized::Object(HashMap::new()));
        }
        let mut data: HashMap<String, Deserialized> = HashMap::new();
        loop {
            self.expect_next(&Token::DoubleQuote)?;
            self.advance_cursor();
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
            self.advance_cursor();
            self.expect_next(&Token::Colon)?;
            self.advance_cursor();
            self.advance_cursor();
            println!("curr token: {:?}", self.curr_token());
            println!("next token: {:?}", self.peek_next());
            let val = self.parse_literal_val()?;
            data.insert(key, val);
            match self.next_token() {
                Some(Token::Comma) => {
                    continue;
                }
                Some(Token::RightBrace) => {
                    return Ok(Deserialized::Object(data));
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
    fn parse_array(&mut self) -> Result<Deserialized, DeserializationError> {
        println!("parsing array");
        if !(self.curr_token() == &Token::LeftBracket) {
            return Err(UNEXPECTED_TOKEN("expected '['".into()));
        }
        let mut data: Vec<Deserialized> = vec![];
        if let Some(t) = self.peek_next() {
            if t == &Token::RightBracket {
                return Ok(Deserialized::Array(data));
            }
        } else {
            return Err(UNEXPECTED_EOF);
        }
        self.advance_cursor();
        loop {
            println!("curr token: {:?}", self.curr_token());
            // todo!();
            data.push(self.parse_literal_val()?);
            println!("pushed");

                match self.next_token() {
                    Some(Token::Comma) => {
                        self.advance_cursor();
                        continue;
                    }
                    Some(Token::RightBracket) => {
                        return Ok(Deserialized::Array(data));
                    }
                    Some(t) => {
                        return Err(UNEXPECTED_TOKEN(format!("expected any of ',' and ']', found {}", t.str_represenation())));
                    }
                    None => {
                        return Err(UNEXPECTED_EOF);
                    }
                }

            }
    }
    pub fn parse(&mut self) -> Result<Deserialized, DeserializationError> {
        return self.parse_literal_val();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::DeserializationError;
    use super::Deserialized;
    use super::Parser;
    use super::Token::{
        Boolean, Colon, Comma, DoubleQuote, LeftBrace, LeftBracket, Null, Number, RightBrace,
        RightBracket, Str,
    };
    #[test]
    fn only_sring() {
        let tokens = vec![DoubleQuote, Str("ungabunga".into()), DoubleQuote];
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
        let map = Deserialized::Object(HashMap::from([
            ("key".to_string(), Deserialized::Number(12f64)),
            ("key2".to_string(), Deserialized::Boolean(true)),
        ]));
        assert_eq!(parsed, Ok(map));
    }
    #[test]
    fn valid_arr() {
        let tokens = vec![
            LeftBracket,
            DoubleQuote,
            Str("v1".into()),
            DoubleQuote,
            Comma,
            Number(12f64),
            Comma,
            DoubleQuote,
            Str("v2".into()),
            DoubleQuote,
            Comma,
            Boolean(true),
            RightBracket,
        ];

        let parsed = Parser::new(tokens).parse();
        println!("{:?}", parsed);
    }
}
