use crate::{ast, ast::Statement};
use crate::lexer;
use crate::token;

#[cfg(test)]
mod test;

#[derive(Debug)]
struct Parser<'a> {
    lex: &'a mut lexer::Lexer,
    pub errors: Vec<String>,

    cur_token: token::Token,
    peek_token: token::Token,
}

impl<'a> Parser<'a> {
    fn new(lex: &'a mut lexer::Lexer) -> Parser<'a> {
        let cur_token = lex.next_token();
        let peek_token = lex.next_token();
        Parser {
            lex,
            errors: Vec::new(),
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token()
    }

    fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program::new();
        while !self.cur_token_is(token::EOF) {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement)
            }
            self.next_token();
        }
        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            token::LET => self.parse_let_statement(),
            token::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(token::IDENT) {
            return None;
        }

        let stmt = ast::LetStatement {
            token: self.cur_token.clone(),
            name: ast::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            },
        };

        if !self.expect_peek(token::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token()
        }

        Some(Statement::Let(stmt))
    }
    
    fn parse_return_statement(&mut self) -> Option<Statement> {
        let stmt = ast::ReturnStatement {
            token: self.cur_token.clone(),
        };
        
        self.next_token();

        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        };
        
        Some(Statement::Return(stmt))
    }

    fn cur_token_is(&self, tok: token::TokenType) -> bool {
        self.cur_token.token_type == tok
    }

    fn peek_token_is(&self, tok: token::TokenType) -> bool {
        self.peek_token.token_type == tok
    }

    fn expect_peek(&mut self, tok: token::TokenType) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn peek_error(&mut self, tok: token::TokenType) {
        self.errors.push(format!(
            "expected next token to be {}, got {} instead",
            tok, self.peek_token.token_type
        ))
    }
}
