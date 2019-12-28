use std::fmt;

#[derive(Debug, PartialEq)]
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
