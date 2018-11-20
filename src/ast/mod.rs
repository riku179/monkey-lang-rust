use token;

pub trait Node {
    fn token_literal(&self) -> Option<&String>;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

#[derive(Debug)]
pub struct Program<S: Statement> {
    statements: Vec<S>,
}

impl<S: Statement> Node for Program<S> {
    fn token_literal(&self) -> Option<&String> {
        if let Some(s) = self.statements.iter().nth(0) {
            s.token_literal()
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    token: token::Token,
    name: Identifier,
    // value: Expression,
}

impl Statement for LetStatement {}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<&String> {
        return Some(&self.token.literal);
    }
}

#[derive(Debug)]
struct Identifier {
    token: token::Token,
    value: String,
}

impl Expression for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> Option<&String> {
        return Some(&self.token.literal)
    }
}
