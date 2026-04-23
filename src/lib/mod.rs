mod deserialized;
mod errors;
mod parser;
mod token;
mod tokenizer;
use parser::Parser;
pub struct Json {
    data: deserialized::Deserialized,
}

impl Json {
    pub fn new(input: String) -> Result<Json, errors::DeserializationError> {
        let tokens = tokenizer::tokenize(input)?;
        let mut parsed_data = Parser::new(tokens);
        return Ok(Json {
            data: parsed_data.parse()?,
        });
    }
}
