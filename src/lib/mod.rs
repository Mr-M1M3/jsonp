mod deserialized;
mod errors;
mod parser;
mod token;
mod tokenizer;
use parser::Parser;

use crate::lib::{deserialized::Deserialized, errors::DeserializationError};

pub fn deserialize(input: String) -> Result<Deserialized, DeserializationError> {
    let tokens = tokenizer::tokenize(input)?;
    return Parser::new(tokens).parse();
}
