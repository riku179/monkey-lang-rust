use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Int(v) => write!(f, "{}", v),
            Object::Bool(v) => write!(f, "{}", v),
            Object::Null => write!(f, "null"),
        }
    }
}
