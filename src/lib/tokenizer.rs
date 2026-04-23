use crate::lib::errors::DeserializationError;
use DeserializationError::{INVALID_NUMBER, UNEXPECTED_EOF, UNEXPECTED_TOKEN};
use crate::lib::token::Token;

pub fn tokenize(input: String) -> Result<Vec<Token>, DeserializationError> {
    let mut iterable_input = input.chars().enumerate().peekable();
    let mut token_accum: Vec<Token> = vec![];

    while let Some((_idx, ch)) = iterable_input.next() {
        match ch {
            '{' => {
                token_accum.push(Token::LeftBrace);
            }
            '}' => {
                token_accum.push(Token::RightBrace);
            }
            '[' => {
                token_accum.push(Token::LeftBracket);
            }
            ']' => {
                token_accum.push(Token::RightBracket);
            }
            '"' => {
                // TODO: Implement escape sequences
                token_accum.push(Token::DoubleQuote);
                let mut str_accum = String::new();
                'inner_extrct_str: loop {
                    match iterable_input.next() {
                        Some((_, '"')) => {
                            break 'inner_extrct_str;
                        }
                        Some((_, part_of_str)) => {
                            str_accum.push(part_of_str);
                        }
                        None => {
                            return Err(UNEXPECTED_EOF);
                        }
                    }
                }
                token_accum.push(Token::Str(str_accum));
                token_accum.push(Token::DoubleQuote);
            }
            ':' => {
                token_accum.push(Token::Colon);
            }
            ',' => {
                token_accum.push(Token::Comma);
            }
            // handle null
            'n' => {
                let mut null_acum = String::with_capacity("null".len());
                null_acum.push('n');
                for _ in 1..=("null".len() - 1) {
                    if let Some((_, part_of_null)) = iterable_input.next() {
                        null_acum.push(part_of_null);
                    } else {
                        return Err(UNEXPECTED_EOF);
                    }
                }
                if null_acum == "null" {
                    token_accum.push(Token::Null)
                } else {
                    return Err(UNEXPECTED_TOKEN("unexpected token n".into()));
                }
            }

            // handle true
            't' => {
                let mut true_acum = String::with_capacity("true".len());
                true_acum.push('t');
                for _ in 1..=("true".len() - 1) {
                    if let Some((_, part_of_true)) = iterable_input.next() {
                        true_acum.push(part_of_true);
                    } else {
                        return Err(UNEXPECTED_EOF);
                    }
                }
                if true_acum == "true" {
                    token_accum.push(Token::Boolean(true))
                } else {
                    return Err(UNEXPECTED_TOKEN("unexpected token t".into()));
                }
            }

            // handle false
            'f' => {
                let mut false_acum = String::with_capacity("false".len());
                false_acum.push('f');
                for _ in 1..=("false".len() - 1) {
                    if let Some((_, part_of_false)) = iterable_input.next() {
                        false_acum.push(part_of_false);
                    } else {
                        return Err(UNEXPECTED_EOF);
                    }
                }
                if false_acum == "false" {
                    token_accum.push(Token::Boolean(false));
                } else {
                    return Err(UNEXPECTED_TOKEN("unexpected token f".into()));
                }
            }
            other => {
                // ignore whitespace
                if other.is_ascii_whitespace() {
                    continue;
                }
                // handle numbers
                if (other == '-') || (other.is_ascii_digit()) || (other == '.') {
                    let mut num_accum = String::from(other);
                    // loops until next char is something other than digit
                    // if we called .next() here, we would lose the character that comes right after the number part ends
                    'extrct_num: while let Some((_, part_of_num)) = iterable_input.peek() {
                        if part_of_num.is_ascii_digit() || (part_of_num == &'.') {
                            // if next char was a digit, push it to num accumulator and call next as .peek() won't advance our uterator
                            num_accum.push(*part_of_num);
                            iterable_input.next();
                        } else {
                            // do not call next as it the next char will be handled by the next iteration
                            break 'extrct_num;
                        }
                    }
                    let num = num_accum.parse::<f64>();
                    if num.is_err() {
                        return Err(INVALID_NUMBER);
                    } else {
                        token_accum.push(Token::Number(num.unwrap()));
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
mod tests{
    use super::{tokenize, Token::*, DeserializationError::*};
    #[test]
    fn tokenization_only_l_brace(){
        let input: String = "{".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![LeftBrace]));
    }

    #[test]
    fn tokenization_only_r_brace(){
        let input: String = "}".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![RightBrace]));
    }

    #[test]
    fn tokenization_only_l_bracket(){
        let input: String = "[".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![LeftBracket]));
    }

    #[test]
    fn tokenization_only_r_bracket(){
        let input: String = "]".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![RightBracket]));
    }

      #[test]
    fn tokenization_only_colon(){
        let input: String = ":".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Colon]));
    }


      #[test]
    fn tokenization_only_comma(){
        let input: String = ",".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Comma]));
    }

    #[test]
    #[should_panic]
    fn tokenization_only_doublequote(){
        let input: String = "\"".to_string();
        let tokens = tokenize(input);
        tokens.unwrap();
    }

    #[test]
    #[should_panic]
    fn tokenization_unfinished_string(){
        let input: String = "\"name".to_string();
        let tokens = tokenize(input);
        tokens.unwrap();
    }

    #[test]
    fn tokenization_only_string(){
        let input: String = "\"test_string\"".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![DoubleQuote, Str("test_string".into()), DoubleQuote]));
    }

    #[test]
    fn tokenization_only_number(){
        let input: String = "90".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Number(90f64)]));
    }


    #[test]
    fn tokenization_only_number_with_radix(){
        let input: String = "90.09".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Number(90.09f64)]));
    }


    #[test]
    fn tokenization_only_number_starting_with_radix(){
        let input: String = ".08".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Number(0.08f64)]));
    }


    #[test]
    fn tokenization_only_negative_number(){
        let input: String = "-100".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Number(-100f64)]));
    }

    #[test]
    fn tokenization_only_negative_number_with_radix(){
        let input: String = "-10.10".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Number(-10.1f64)]));
    }

    #[test]
    fn tokenization_only_negative_number_immediately_followed_by_radix(){
        let input: String = "-.10".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Number(-0.1f64)]));
    }

    #[test]
    fn tokenization_only_bool_true(){
        let input: String = "true".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Boolean(true)]));
    }

    #[test]
    fn tokenization_only_bool_false(){
        let input: String = "false".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Boolean(false)]));
    }

    #[test]
    fn tokenization_only_null(){
        let input: String = "null".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![Null]));
    }


    #[test]
    fn tokenization_all_tokens(){
        let input: String = "{}[]\"name\":,-100 true false null".to_string();
        let tokens = tokenize(input);
        assert_eq!(tokens, Ok(vec![LeftBrace, RightBrace, LeftBracket, RightBracket, DoubleQuote, Str("name".into()), DoubleQuote, Colon, Comma, Number(-100.0), Boolean(true), Boolean(false), Null]));
    }
}