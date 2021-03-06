use crate::token::Token;
use std::fmt;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for stmt in &self.statements {
            s.push_str(&format!("{}", stmt))
        }
        write!(f, "{}", s)
    }
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Ident(pub String);

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Stmt {
    Let(Ident, Expr),
    Return(Expr),
    Expr(Expr),
    Block(BlockStmt),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Let(ident, expr) => write!(f, "let {} = {}", ident, expr),
            Stmt::Return(expr) => write!(f, "return {}", expr),
            Stmt::Expr(expr) => write!(f, "{}", expr),
            Stmt::Block(stmts) => {
                let mut ret = Ok(());
                for stmt in stmts {
                    ret = write!(f, "{}", stmt);
                }
                ret
            }
        }
    }
}

pub type BlockStmt = Vec<Stmt>;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    Prefix(Prefix, Box<Expr>),                   // (prefix, expr)
    Infix(Box<Expr>, Infix, Box<Expr>),          // (left, infix, right)
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>), // (cond, cons, alter)
    Function(Vec<Ident>, BlockStmt),             // (args, body)
    Call(Box<Expr>, Vec<Expr>),                  // (function, args)
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Ident(ident) => write!(f, "{}", ident),
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Prefix(prefix, expr) => write!(f, "({}{})", prefix, expr),
            Expr::Infix(left, infix, right) => write!(f, "({} {} {})", left, infix, right),
            Expr::If(cond, cons, alter) => {
                if let Some(box stmt) = alter {
                    write!(f, "if {} {} else {}", cond, cons, stmt)
                } else {
                    write!(f, "if {} {}", cond, cons)
                }
            }
            Expr::Function(params, body) => {
                let params_string: Vec<String> =
                    params.iter().map(|param| param.0.clone()).collect();
                write!(
                    f,
                    "fn ({}) {{\n {} }}",
                    params_string.join(", "),
                    body.iter()
                        .map(|stmt| format!("{}", stmt))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            Expr::Call(box func, args) => {
                let params_string: Vec<String> =
                    args.iter().map(|param| format!("{}", param)).collect();
                write!(f, "{}({})", func, params_string.join(", "))
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int(i64),
    Bool(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Int(val) => write!(f, "{}", val),
            Literal::Bool(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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
            _ => Err(format!("this is not prefix token. got {:?}", tok)),
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

#[derive(Clone, PartialEq, Debug)]
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
            _ => Err(format!("this is not prefix token. got {:?}", tok)),
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
            Infix::Equal => write!(f, "=="),
            Infix::NotEqual => write!(f, "!="),
            Infix::GreaterThan => write!(f, ">"),
            Infix::LessThan => write!(f, "<"),
        }
    }
}
