use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Deserialized {
    Str(String),
    Number(f64),
    Boolean(bool),
    List(Vec<Deserialized>),
    Map(HashMap<String, Deserialized>),
    Null
}
