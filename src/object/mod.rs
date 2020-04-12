use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Return(Box<Object>),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Int(v) => write!(f, "{}", v),
            Object::Bool(v) => write!(f, "{}", v),
            Object::Return(box v) => write!(f, "return {}", v),
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
            Object::Null => "NULL",
        }
        .to_string()
    }
}

pub type EvalResult<T> = Result<T, EvalError>;

#[derive(PartialEq, Debug)]
pub struct EvalError(pub String);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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
