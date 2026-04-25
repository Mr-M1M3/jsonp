use crate::{DeserializationError, Deserialized};

mod parser;

pub fn deserialize(input: String) -> Result<Deserialized, DeserializationError> {
    return parser::Parser::parse(input);
}