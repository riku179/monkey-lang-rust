use crate::ast::{Expr, Ident, Literal, Program, Stmt, Prefix};
use crate::lexer;
use crate::token;

#[cfg(test)]
mod test;

#[derive(PartialOrd, PartialEq)]
enum Priority {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

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

    /// entry point
    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while !self.cur_token_is(token::EOF) {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement)
            }
            self.next_token();
        }
        program
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token()
    }

    fn parse_statement(&mut self) -> Option<Stmt> {
        match self.cur_token.token_type {
            token::LET => self.parse_let_statement(),
            token::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Stmt> {
        if !self.expect_peek(token::IDENT) {
            return None;
        }

        let stmt = Stmt::Let(Ident(self.cur_token.literal.clone()));

        if !self.expect_peek(token::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token()
        }

        Some(stmt)
    }

    fn parse_return_statement(&mut self) -> Option<Stmt> {
        let stmt = Stmt::Return;

        self.next_token();

        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_expression_statement(&mut self) -> Option<Stmt> {
        if let Some(expr) = self.parse_expression(Priority::LOWEST) {
            let stmt = Stmt::Expr(expr);

            if self.peek_token_is(token::SEMICOLON) {
                self.next_token()
            };

            Some(stmt)
        } else {
            None
        }
    }

    fn parse_expression(&mut self, priority: Priority) -> Option<Expr> {
        match self.cur_token.token_type {
            token::IDENT => Some(self.parse_identifier()),
            token::INT => self.parse_integer_literal(),
            token::PLUS => self.parse_prefix_expr(),
            token::MINUS => self.parse_prefix_expr(),
            token::BANG => self.parse_prefix_expr(),
            _ => {
                self.errors.push(format!("unknown token in expression. got {:?}", self.cur_token.token_type));
                None
            },
        }
    }

    fn parse_identifier(&self) -> Expr {
        Expr::Ident(Ident(self.cur_token.literal.clone()))
    }

    fn parse_integer_literal(&mut self) -> Option<Expr> {
        match self.cur_token.literal.parse::<i64>() {
            Ok(val) => Some(Expr::Literal(Literal::Int(val))),
            Err(_) => {
                self.errors.push(format!(
                    "could not parse {:?} as interger",
                    self.cur_token.literal
                ));
                None
            }
        }
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let cur_token = self.cur_token.clone();

        self.next_token();

        if let Some(expr) = self.parse_expression(Priority::PREFIX) {
            match Prefix::from_token(&cur_token) {
                Ok(prefix) => Some(Expr::Prefix(prefix, Box::new(expr))),
                Err(err) => {
                    self.errors.push(err);
                    None
                }
            }
        } else {
            None
        }
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
            self.peek_error(tok);
            false
        }
    }

    fn peek_error(&mut self, tok: token::TokenType) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} instead",
            tok, self.peek_token.token_type
        ))
    }
}
