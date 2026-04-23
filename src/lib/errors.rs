#[derive(Debug, PartialEq)]
pub enum DeserializationError{
    UNEXPECTED_EOF,
    UNEXPECTED_TOKEN(String),
    INVALID_NUMBER
}