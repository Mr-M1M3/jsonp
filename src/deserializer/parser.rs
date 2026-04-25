use std::collections::HashMap;

use crate::utils::deserialized::Deserialized;
use crate::utils::errors::DeserializationError;
use crate::utils::errors::DeserializationError::{UnexpctedEOF, UnexpectedToken};

mod tokenizer;
use tokenizer::{Token};

pub struct Parser {
    input: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn parse(input: String) -> Result<Deserialized, DeserializationError> {
        let tokens = tokenizer::tokenize(input)?;
        return Parser::parse_raw_tokens(tokens);
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

    fn expect_and_get_next(&self) -> Result<&Token, DeserializationError> {
        match self.input.get(self.cursor + 1) {
            Some(t) => {
                return Ok(t);
            }
            None => return Err(UnexpctedEOF),
        }
    }
    fn parse_raw_tokens(tokens: Vec<Token>) -> Result<Deserialized, DeserializationError> {
        let mut parser = Parser {
            input: tokens,
            cursor: 0,
        };

        let parsed_result = parser.parse_literal_val()?;
        match parser.peek_next() {
            Some(t) => {
                return Err(UnexpectedToken(format!("unexpected token {t}")));
            }
            None => {}
        }
        return Ok(parsed_result);
    }
    fn parse_literal_val(&mut self) -> Result<Deserialized, DeserializationError> {
        match self.curr_token() {
            Token::DoubleQuote { pos: _ } => match self.next_token() {
                Some(Token::Str { pos: _, val }) => {
                    let str = val.clone();
                    self.expect_and_get_next()?.expect_double_quote()?;
                    self.advance_cursor();
                    return Ok(Deserialized::Str(str));
                }
                Some(t) => {
                    return Err(UnexpectedToken(format!(
                        "unexpected token {}",
                        t.to_string()
                    )));
                }
                None => {
                    return Err(UnexpctedEOF);
                }
            },
            Token::Number { pos: _, val } => {
                return Ok(Deserialized::Number(*val));
            }

            Token::Boolean { pos: _, val } => {
                return Ok(Deserialized::Boolean(*val));
            }

            Token::Null { pos: _ } => {
                return Ok(Deserialized::Null);
            }
            Token::LeftBrace { pos: _ } => {
                return self.parse_obj();
            }
            Token::LeftBracket { pos: _ } => {
                return self.parse_array();
            }
            t => {
                println!("parsing {t}");
                return Err(UnexpectedToken(format!(
                    "expected any of '{{', '[' and json primitive, found {}",
                    t.to_string()
                )));
            }
        }
    }
    fn parse_obj(&mut self) -> Result<Deserialized, DeserializationError> {
        self.curr_token().expect_left_brace()?;
        match self.peek_next() {
            Some(t) if t.is_right_brace() => {
                self.advance_cursor();
                return Ok(Deserialized::Object(HashMap::new()));
            }
            _ => {}
        }
        let mut data: HashMap<String, Deserialized> = HashMap::new();
        loop {
            self.expect_and_get_next()?.expect_double_quote()?;
            self.advance_cursor();
            let key = match self.next_token() {
                Some(Token::Str { pos: _, val: k }) => k.clone(),
                Some(other) => {
                    return Err(UnexpectedToken(format!(
                        "unexpected token {}",
                        other.to_string()
                    )));
                }
                None => {
                    return Err(UnexpctedEOF);
                }
            };
            self.expect_and_get_next()?.expect_double_quote()?;
            self.advance_cursor();
            self.expect_and_get_next()?.expect_colon()?;
            self.advance_cursor();
            self.advance_cursor();
            let val = self.parse_literal_val()?;
            data.insert(key, val);
            match self.next_token() {
                Some(t) if t.is_comma() => {
                    continue;
                }
                Some(t) if t.is_right_brace() => {
                    return Ok(Deserialized::Object(data));
                }
                Some(t) => {
                    return Err(UnexpectedToken(format!(
                        "expected ',' or '}}' found: {}",
                        t.to_string()
                    )));
                }
                None => {
                    return Err(UnexpctedEOF);
                }
            }
        }
    }
    fn parse_array(&mut self) -> Result<Deserialized, DeserializationError> {
        self.curr_token().expect_left_bracket()?;

        let mut data: Vec<Deserialized> = vec![];
        match self.peek_next() {
            Some(t) if t.is_right_bracket() => {
                return Ok(Deserialized::Array(data));
            }
            _ => {}
        }
        self.advance_cursor();
        loop {
            data.push(self.parse_literal_val()?);
            match self.next_token() {
                Some(t) if t.is_comma() => {
                    self.advance_cursor();
                    continue;
                }
                Some(t) if t.is_right_bracket() => {
                    return Ok(Deserialized::Array(data));
                }
                Some(t) => {
                    return Err(UnexpectedToken(format!(
                        "expected any of ',' and ']', found {}",
                        t.to_string()
                    )));
                }
                None => {
                    return Err(UnexpctedEOF);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use super::tokenizer::tokenize;

    #[test]
    fn parser_valid_string() {
        let input = "[100, 300]}".to_string();
        let tokens = tokenize(input);
        println!("{:?}", Parser::parse_raw_tokens(tokens.unwrap()));
    }
}
