use super::{Token, TokenPosition};
use crate::lib::errors::DeserializationError;
use DeserializationError::{INVALID_NUMBER, UNEXPECTED_EOF, UNEXPECTED_TOKEN};

pub fn tokenize(input: String) -> Result<Vec<Token>, DeserializationError> {
    let mut token_position = TokenPosition::origin();
    let mut iterable_input = input.chars().enumerate().peekable();
    let mut token_accum: Vec<Token> = vec![];

    while let Some((_idx, ch)) = iterable_input.next() {
        match ch {
            '{' => {
                token_accum.push(Token::LeftBrace {
                    pos: token_position.clone(),
                });
                token_position.adv_col();
            }
            '}' => {
                token_accum.push(Token::RightBrace {
                    pos: token_position.clone(),
                });
                token_position.adv_col();
            }
            '[' => {
                token_accum.push(Token::LeftBracket {
                    pos: token_position.clone(),
                });

                token_position.adv_col();
            }
            ']' => {
                token_accum.push(Token::RightBracket {
                    pos: token_position.clone(),
                });

                token_position.adv_col();
            }
            '"' => {
                // TODO: Implement escape sequences
                token_accum.push(Token::DoubleQuote {
                    pos: token_position.clone(),
                });
                token_position.adv_col();
                let mut str_accum = String::new();
                let str_tok_pos: TokenPosition = token_position.clone();
                'inner_extrct_str: loop {
                    match iterable_input.next() {
                        Some((_, '"')) => {
                            break 'inner_extrct_str;
                        }
                        Some((_, part_of_str)) => {
                            str_accum.push(part_of_str);
                            match part_of_str {
                                '\n' => {
                                    token_position.adv_line();
                                    token_position.set_col(1);
                                }
                                _ => {
                                    token_position.adv_col();
                                }
                            }
                        }
                        None => {
                            return Err(UNEXPECTED_EOF);
                        }
                    }
                }
                token_accum.push(Token::Str {
                    pos: str_tok_pos,
                    val: str_accum,
                });
                token_accum.push(Token::DoubleQuote {
                    pos: token_position.clone(),
                });
                token_position.adv_col();
            }
            ':' => {
                token_accum.push(Token::Colon {
                    pos: token_position.clone(),
                });
                token_position.adv_col();
            }
            ',' => {
                token_accum.push(Token::Comma {
                    pos: token_position.clone(),
                });
                token_position.adv_col();
            }
            // handle null
            'n' => {
                let mut null_acum = String::with_capacity("null".len());
                null_acum.push('n');
                let n_pos = token_position.clone();
                token_position.adv_col();
                for _ in 1..=("null".len() - 1) {
                    if let Some((_, part_of_null)) = iterable_input.next() {
                        null_acum.push(part_of_null);
                    } else {
                        return Err(UNEXPECTED_EOF);
                    }
                    token_position.adv_col();
                }
                if null_acum == "null" {
                    token_accum.push(Token::Null {
                        pos: n_pos,
                    })
                } else {
                    return Err(UNEXPECTED_TOKEN("unexpected token n".into()));
                }
            }

            // handle true
            't' => {
                let mut true_acum = String::with_capacity("true".len());
                true_acum.push('t');
                let t_pos = token_position.clone();
                token_position.adv_col();
                for _ in 1..=("true".len() - 1) {
                    if let Some((_, part_of_true)) = iterable_input.next() {
                        true_acum.push(part_of_true);
                    } else {
                        return Err(UNEXPECTED_EOF);
                    }
                    token_position.adv_col();
                }
                if true_acum == "true" {
                    token_accum.push(Token::Boolean {
                        pos: t_pos,
                        val: true,
                    })
                } else {
                    return Err(UNEXPECTED_TOKEN("unexpected token t".into()));
                }
            }

            // handle false
            'f' => {
                let mut false_acum = String::with_capacity("false".len());
                false_acum.push('f');
                let f_pos = token_position.clone();
                token_position.adv_col();
                for _ in 1..=("false".len() - 1) {
                    if let Some((_, part_of_false)) = iterable_input.next() {
                        false_acum.push(part_of_false);
                    } else {
                        return Err(UNEXPECTED_EOF);
                    }
                    token_position.adv_col();
                }
                if false_acum == "false" {
                    token_accum.push(Token::Boolean {
                        pos: f_pos,
                        val: false,
                    });
                } else {
                    return Err(UNEXPECTED_TOKEN("unexpected token f".into()));
                }
            }
            other => {
                // ignore whitespace
                match other {
                    '\n' => {
                        token_position.adv_line();
                        token_position.set_col(1);
                        continue;
                    }
                    not_line_break => {
                        if not_line_break.is_ascii_whitespace() {
                            token_position.adv_col();
                            continue;
                        }
                    }
                }
                // handle numbers
                if (other == '-') || (other.is_ascii_digit()) || (other == '.') {
                    let num_pos = token_position.clone();
                    let mut num_accum = String::from(other);
                    // BOOKMARK
                    // token_position.adv_col();
                    // loops until next char is something other than digit
                    // if we called .next() here, we would lose the character that comes right after the number part ends
                    'extrct_num: while let Some((_, part_of_num)) = iterable_input.peek() {
                        if part_of_num.is_ascii_digit() || (part_of_num == &'.') {
                            // if next char was a digit, push it to num accumulator and call next as .peek() won't advance our uterator
                            num_accum.push(*part_of_num);
                            token_position.adv_col();
                            iterable_input.next();
                        } else {
                            // do not call .next() as it the next char will be handled by the next iteration
                            // but advance the token position, because we call .peek(), that checks for next item
                            // when .peek() doesn't return valid digit, that actually means our iterator is on the last digit
                            // so, we have to adv tokenn position by 1
                            token_position.adv_col();
                            break 'extrct_num;
                        }
                    }
                    let num = num_accum.parse::<f64>();
                    if num.is_err() {
                        return Err(INVALID_NUMBER);
                    } else {
                        token_accum.push(Token::Number {
                            pos: num_pos,
                            val: num.unwrap(),
                        });
                    }
                } else {
                    return Err(UNEXPECTED_TOKEN(format!("unexpected token {other}")));
                }
            }
        }
    }
    return Ok(token_accum);
}

#[cfg(test)]
mod tests {
    use crate::lib::tokenizer::TokenPosition;

    use super::{DeserializationError::*, Token::*, tokenize};
    #[test]
    fn tokenization_only_l_brace() {
        let input: String = "{".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![LeftBrace {
                pos: TokenPosition::origin()
            }])
        );
    }

    #[test]
    fn tokenization_two_l_brace() {
        let input: String = "{{".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                LeftBrace {
                    pos: TokenPosition::origin()
                },
                LeftBrace {
                    pos: TokenPosition::from((1, 2)),
                }
            ])
        );
    }

    #[test]
    fn tokenization_only_r_brace() {
        let input: String = "}".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![RightBrace {
                pos: TokenPosition::origin()
            }])
        );
    }

    #[test]
    fn tokenization_only_l_bracket() {
        let input: String = "[".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![LeftBracket {
                pos: TokenPosition::origin()
            }])
        );
    }

    #[test]
    fn tokenization_only_r_bracket() {
        let input: String = "]".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![RightBracket {
                pos: TokenPosition::origin()
            }])
        );
    }

    #[test]
    fn tokenization_only_colon() {
        let input: String = ":".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Colon {
                pos: TokenPosition::origin()
            }])
        );
    }

    #[test]
    fn tokenization_only_comma() {
        let input: String = ",".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Comma {
                pos: TokenPosition::origin()
            }])
        );
    }

    #[test]
    #[should_panic]
    fn tokenization_only_doublequote() {
        let input: String = "\"".to_string();
        let tokens = tokenize(input);
        tokens.unwrap();
    }

    #[test]
    #[should_panic]
    fn tokenization_unfinished_string() {
        let input: String = "\"name".to_string();
        let tokens = tokenize(input);
        tokens.unwrap();
    }

    #[test]
    fn tokenization_only_string_single_line() {
        let input: String = "\"test_string\"".to_string();
        let tokens = tokenize(input.clone());
        assert_eq!(
            tokens,
            Ok(vec![
                DoubleQuote {
                    pos: TokenPosition::origin()
                },
                Str {
                    pos: TokenPosition::from((1, 2)),
                    val: "test_string".into()
                },
                DoubleQuote {
                    pos: TokenPosition::from((1, 13))
                }
            ])
        );
    }


    #[test]
    fn tokenization_only_string_mul_line() {
        let input: String = "\"str-1 \n str-2\"".to_string();
        let tokens = tokenize(input.clone());
        assert_eq!(
            tokens,
            Ok(vec![
                DoubleQuote {
                    pos: TokenPosition::origin()
                },
                Str {
                    pos: TokenPosition::from((1, 2)),
                    val: "str-1 \n str-2".into()
                },
                DoubleQuote {
                    pos: TokenPosition::from((2, 7))
                }
            ])
        );
    }

    #[test]
    fn tokenization_only_number() {
        let input: String = "90".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Number {
                pos: TokenPosition::origin(),
                val: 90f64
            }])
        );
    }

    #[test]
    fn tokenization_only_number_with_radix() {
        let input: String = "90.09".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Number {
                pos: TokenPosition::origin(),
                val: 90.09f64
            }])
        );
    }

    #[test]
    fn tokenization_only_number_starting_with_radix() {
        let input: String = ".08".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Number {
                pos: TokenPosition::origin(),
                val: 0.08f64
            }])
        );
    }

    #[test]
    fn tokenization_only_negative_number() {
        let input: String = "-100".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Number {
                pos: TokenPosition::origin(),
                val: -100f64
            }])
        );
    }

    #[test]
    fn tokenization_only_negative_number_with_radix() {
        let input: String = "-10.10".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Number {
                pos: TokenPosition::origin(),
                val: -10.1f64
            }])
        );
    }

    #[test]
    fn tokenization_only_negative_number_immediately_followed_by_radix() {
        let input: String = "-.10".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Number {
                pos: TokenPosition::origin(),
                val: -0.1f64
            }])
        );
    }

    #[test]
    fn tokenization_only_bool_true() {
        let input: String = "true".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Boolean {
                pos: TokenPosition::origin(),
                val: true
            }])
        );
    }

    #[test]
    fn tokenization_only_bool_false() {
        let input: String = "false".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Boolean {
                pos: TokenPosition::origin(),
                val: false
            }])
        );
    }

    #[test]
    fn tokenization_only_null() {
        let input: String = "null".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![Null {
                pos: TokenPosition::origin()
            }])
        );
    }

    #[test]
    fn tokenization_all_tokens() {
        let input: String = "{}[]\"name\":,-100 true false \n null".to_string();
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                LeftBrace {
                    pos: TokenPosition::origin()
                },
                RightBrace {
                    pos: TokenPosition::from((1, 2))
                },
                LeftBracket {
                    pos: TokenPosition::from((1, 3))
                },
                RightBracket {
                    pos: TokenPosition::from((1, 4))
                },
                DoubleQuote {
                    pos: TokenPosition::from((1, 5))
                },
                Str {
                    pos: TokenPosition::from((1, 6)),
                    val: "name".into()
                },
                DoubleQuote {
                    pos: TokenPosition::from((1, 10))
                },
                Colon {
                    pos: TokenPosition::from((1, 11))
                },
                Comma {
                    pos: TokenPosition::from((1, 12))
                },
                Number {
                    pos: TokenPosition::from((1, 13)),
                    val: -100.0
                },
                Boolean {
                    pos: TokenPosition::from((1, 18)),
                    val: true
                },
                Boolean {
                    pos: TokenPosition::from((1, 23)),
                    val: false
                },
                Null {
                    pos: TokenPosition::from((2, 2))
                }
            ])
        );
    }
}
