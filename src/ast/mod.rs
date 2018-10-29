use token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {}

trait Expression: Node {}

#[derive(Debug)]
struct Program<S: Statement> {
    statements: Vec<S>,
}

impl<S: Statement> Node for Program<S> {
    fn token_literal(&self) -> String {
        if let Some(s) = self.statements.iter().nth(0) {
            s.token_literal()
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
struct LetStatement {
    token: token::Token,
    name: Identifier,
    // value: Expression,
}

impl Statement for LetStatement {}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token_literal();
    }
}

#[derive(Debug)]
struct Identifier {
    token: token::Token,
    value: String,
}

impl Expression for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.to_string();
    }
}
