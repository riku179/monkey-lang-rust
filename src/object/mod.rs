use crate::ast::{BlockStmt, Ident};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Return(Box<Object>),
    Func(Func),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Int(v) => write!(f, "{}", v),
            Object::Bool(v) => write!(f, "{}", v),
            Object::Return(box v) => write!(f, "return {}", v),
            Object::Func(func) => func.fmt(f),
            Object::Null => write!(f, "null"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    pub args: Vec<Ident>,
    pub body: BlockStmt,
    pub env: Env,
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"fn ({}) {{ {} }}"#,
            self.args
                .iter()
                .map(|i| i.0.clone())
                .collect::<Vec<String>>()
                .join(", "),
            self.body
                .iter()
                .map(|stmt| format!("{}", stmt))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::Int(_) => "INT",
            Object::Bool(_) => "BOOLEAN",
            Object::Return(_) => "RETURN",
            Object::Func(_) => "FUNCTION",
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

#[derive(Clone, PartialEq, Debug)]
pub struct Env {
    store: HashMap<String, Object>,
    source: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            source: None,
        }
    }

    pub fn wrap(source_env: Env) -> Self {
        Env {
            store: HashMap::new(),
            source: Some(Box::new(source_env)),
        }
    }

    pub fn get(&self, key: String) -> Option<&Object> {
        if let obj @ Some(_) = self.store.get(&key) {
            obj
        } else {
            if let Some(ref source) = self.source {
                source.get(key)
            } else {
                None
            }
        }
    }

    pub fn insert(&mut self, key: String, val: Object) -> Option<Object> {
        self.store.insert(key, val)
    }
}
