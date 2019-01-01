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
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int(i64),
}