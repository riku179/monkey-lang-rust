use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Return(Box<Option<Object>>),
    Error(String),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Int(v) => write!(f, "{}", v),
            Object::Bool(v) => write!(f, "{}", v),
            Object::Return(v) => match v {
                box Some(v) => write!(f, "return {}", v),
                box None => write!(f, "return"),
            },
            Object::Error(v) => write!(f, "ERROR: {}", v),
            Object::Null => write!(f, "null"),
        }
    }
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::Int(_) => "INT",
            Object::Bool(_) => "BOOLEAN",
            Object::Return(_) => "RETURN",
            Object::Error(_) => "ERROR",
            Object::Null => "NULL",
        }
        .to_string()
    }
}

#[derive(PartialEq, Debug)]
pub struct Env {
    store: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<&Object> {
        self.store.get(&key)
    }

    pub fn insert(&mut self, key: String, val: Object) -> Option<Object> {
        self.store.insert(key, val)
    }
}
