use std::fmt;
use crate::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug)]
pub enum Stmt {
    // Let(Ident, Expr),
    Let(Ident),
    // Return(Expr),
    Return,
    Expr(Expr),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    Prefix(Prefix, Box<Expr>),
    Infix(Box<Expr>, Infix, Box<Expr>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int(i64),
}

#[derive(PartialEq, Debug)]
pub enum Prefix {
    Plus,
    Minus,
    Not,
}

impl Prefix {
    pub fn from_token(tok: &Token) -> Result<Self, String> {
        match tok {
            Token::PLUS => Ok(Prefix::Plus),
            Token::MINUS => Ok(Prefix::Minus),
            Token::BANG => Ok(Prefix::Not),
            _ => Err(format!("this is not prefix token. got {:?}", tok))
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Prefix::Plus => write!(f, "+"),
            Prefix::Minus => write!(f, "-"),
            Prefix::Not => write!(f, "!"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
}

impl Infix {
    pub fn from_token(tok: &Token) -> Result<Self, String> {
        match tok {
            Token::PLUS => Ok(Infix::Plus),
            Token::MINUS => Ok(Infix::Minus),
            Token::SLASH => Ok(Infix::Divide),
            Token::ASTERISK => Ok(Infix::Multiply),
            Token::EQ => Ok(Infix::Equal),
            Token::NOTEQ => Ok(Infix::NotEqual),
            Token::GT => Ok(Infix::GreaterThan),
            Token::LT => Ok(Infix::LessThan),
            _ => Err(format!("this is not prefix token. got {:?}", tok))
        }
    }
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Divide => write!(f, "/"),
            Infix::Multiply => write!(f, "*"),
            Infix::Equal => write!(f, "="),
            Infix::NotEqual => write!(f, "!"),
            Infix::GreaterThan => write!(f, ">"),
            Infix::LessThan => write!(f, "<"),
        }
    }
}