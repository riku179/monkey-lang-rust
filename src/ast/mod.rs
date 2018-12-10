use crate::token;

pub trait Node {
    fn token_literal(&self) -> Option<&String>;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> Option<&String> {
        match self {
            Statement::Let(stmt) => stmt.token_literal(),
            Statement::Return(stmt) => stmt.token_literal(),
        }
    }
}

pub trait Expression: Node {}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> Option<&String> {
        if let Some(s) = self.statements.get(0) {
            s.token_literal()
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
//    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<&String> {
        Some(&self.token.literal)
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: token::Token,
//    pub return_value: Expression
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> Option<&String> {
        Some(&self.token.literal)
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Expression for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> Option<&String> {
        Some(&self.token.literal)
    }
}
