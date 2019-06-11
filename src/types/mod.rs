use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Str(String),
    Boolean(bool),
    List(Vec<Value>),
    Hash(HashMap<String, Value>),
    Number(f32),
}

