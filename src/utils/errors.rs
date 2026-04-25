#[derive(Debug, PartialEq)]
pub enum DeserializationError{
    UnexpctedEOF,
    UnexpectedToken(String),
    InvalidNumber
}