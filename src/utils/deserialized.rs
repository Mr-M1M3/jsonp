use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum Deserialized {
    Str(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Deserialized>),
    Object(HashMap<String, Deserialized>),
    Null,
}

impl Deserialized {
    pub fn is_string(&self) -> bool {
        if let Self::Str(_) = self { true } else { false }
    }
    pub fn is_number(&self) -> bool {
        if let Self::Number(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_boolean(&self) -> bool {
        if let Self::Boolean(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_null(&self) -> bool {
        if let Self::Null = self { true } else { false }
    }
    pub fn is_obj(&self) -> bool {
        if let Self::Object(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_arr(&self) -> bool {
        if let Self::Array(_) = self {
            true
        } else {
            false
        }
    }
    pub fn read_as_string(&self) -> Option<&String> {
        if let Self::Str(v) = self {
            return Some(v);
        } else {
            return None;
        }
    }

    pub fn read_as_number(&self) -> Option<f64> {
        if let Self::Number(v) = self {
            return Some(*v);
        } else {
            return None;
        }
    }
    pub fn read_as_boolean(&self) -> Option<bool> {
        if let Self::Boolean(v) = self {
            return Some(*v);
        } else {
            return None;
        }
    }
    pub fn read_as_null(&self) -> Option<()> {
        if let Self::Null = self {
            return Some(());
        } else {
            return None;
        }
    }
    pub fn read_as_obj(&self) -> Option<&HashMap<String, Self>> {
        if let Self::Object(map) = self {
            return Some(map);
        } else {
            return None;
        }
    }
    pub fn read_as_array(&self) -> Option<&Vec<Self>> {
        if let Self::Array(map) = self {
            return Some(map);
        } else {
            return None;
        }
    }
}

impl Deserialized {
    pub fn serialize(&self) -> String {
        return self
            .serialize_formatted(0)
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect();
    }
    fn serialize_formatted(&self, indent_lev: usize) -> String {
        let mut str_accum = String::new();
        match self {
            Self::Str(v) => {
                str_accum.push_str(format!("\"{}\"", v).as_str());
            }
            Self::Number(v) => {
                str_accum.push_str(v.to_string().as_str());
            }
            Self::Boolean(v) => {
                str_accum.push_str(v.to_string().as_str());
            }

            Self::Null => {
                str_accum.push_str("null");
            }
            Self::Object(obj) => {
                let indent_lev = if indent_lev == 0 { 1 } else{ indent_lev };
                let mut indent_space_for_val = String::new();
                for _ in 0..indent_lev {
                    // indent_space_for_val.push_str(indent_lev.to_string().as_str());
                    indent_space_for_val.push(' ');
                }

                 let mut indent_space_for_brace = String::new();
                for _ in 0..(indent_lev - 1) {
                    // indent_space_for_val.push_str(indent_lev.to_string().as_str());
                    indent_space_for_brace.push(' ');
                }
                // str_accum.push_str(&indent_space_for_val);
                str_accum.push('{');
                str_accum.push('\n');
                str_accum.push_str(&indent_space_for_brace);
                let mut obj = obj.iter().collect::<Vec<(&String, &Deserialized)>>();
                obj.sort_by_key(|(k, _)| *k);
                obj.iter().for_each(|(k, v)| {
                    str_accum.push_str(format!("\"{}\"", k).as_str());
                    str_accum.push(':');
                    str_accum.push(' ');
                    str_accum.push_str(v.serialize_formatted(indent_lev + 1).as_str());
                    str_accum.push(',');
                    str_accum.push('\n');
                    str_accum.push_str(&indent_space_for_val);
                });
                // removes the last comma that is syntax error in json
                str_accum = str_accum.trim_end().to_string();
                if str_accum.ends_with(",") {
                    str_accum.pop();
                }
                str_accum.push('\n');
                str_accum.push_str(&indent_space_for_brace);
                str_accum.push('}');
            }
            Self::Array(arr) => {
                str_accum.push('[');
                arr.iter().for_each(|item| {
                    str_accum.push_str(item.serialize().as_str());
                    str_accum.push(',');
                });
                // removes the last comma that is syntax error in json
                if str_accum.ends_with(',') {
                    str_accum.pop();
                }
                str_accum.push(']');
            }
        };
        return str_accum;
    }
}

impl Display for Deserialized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize_formatted(1))
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Deserialized;
    #[test]
    fn serialize_only_str() {
        let to_serialize = Deserialized::Str("test string".into());
        let serialized = to_serialize.serialize();
        assert_eq!(serialized, "\"test string\"".to_string());
    }

    #[test]
    fn serialize_only_number() {
        let to_serialize = Deserialized::Number(-100.99f64);
        let serialized = to_serialize.serialize();
        assert_eq!(serialized, (-100.99f64).to_string());
    }

    #[test]
    fn serialize_only_boolean() {
        let to_serialize = Deserialized::Boolean(true);
        let serialized = to_serialize.serialize();
        assert_eq!(serialized, "true".to_string());
    }

    #[test]
    fn serialize_only_null() {
        let to_serialize = Deserialized::Null;
        let serialized = to_serialize.serialize();
        assert_eq!(serialized, "null".to_string());
    }

    #[test]
    fn serialize_empty_obj() {
        let to_serialize = Deserialized::Object(HashMap::new());
        let serialized = to_serialize.serialize();
        assert_eq!(serialized, "{}".to_string());
    }

    #[test]
    fn serialize_obj_one_key() {
        let obj = HashMap::from([("key".into(), Deserialized::Str("val".into()))]);
        let to_serialize = Deserialized::Object(obj);
        let serialized = to_serialize.serialize();
        assert_eq!(serialized, "{\"key\":\"val\"}".to_string());
    }

    #[test]
    fn serialize_obj_mul_key() {
        let obj = HashMap::from([
            ("key1".into(), Deserialized::Str("val".into())),
            ("key2".into(), Deserialized::Number(-99.909)),
        ]);
        let to_serialize = Deserialized::Object(obj);
        let serialized = to_serialize.serialize();
        assert_eq!(
            serialized,
            "{\"key1\":\"val\",\"key2\":-99.909}".to_string()
        );
    }

    #[test]
    fn serialize_empty_arr() {
        let to_serialize = Deserialized::Array(vec![]);
        let serialized = to_serialize.serialize();
        assert_eq!(serialized, "[]".to_string());
    }

    #[test]
    fn serialize_arr_one_item() {
        let arr = vec![Deserialized::Str("lol".into())];
        let to_serialize = Deserialized::Array(arr);
        let serialized = to_serialize.serialize();
        assert_eq!(serialized, "[\"lol\"]".to_string());
    }

    #[test]
    fn serialize_arr_mul_item() {
        let arr = vec![
            Deserialized::Str("val".into()),
            Deserialized::Number(-99.909),
        ];
        let to_serialize = Deserialized::Array(arr);
        let serialized = to_serialize.serialize();
        assert_eq!(serialized, "[\"val\",-99.909]".to_string());
    }
}
