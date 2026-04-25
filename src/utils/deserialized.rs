use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Deserialized {
    Str(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Deserialized>),
    Object(HashMap<String, Deserialized>),
    Null
}
