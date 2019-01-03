use crate::ast::{Expr, Ident, Literal, Program, Stmt, Prefix, Infix};
use crate::lexer;
use crate::token::Token;

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

    cur_token: Token,
    peek_token: Token,
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

        while !self.cur_token_is(&Token::EOF) {
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
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Stmt> {
        if let Token::IDENT(val) = self.peek_token.clone() {
            self.next_token();

            let stmt = Stmt::Let(Ident(val));

            if !self.expect_peek(&Token::ASSIGN) {
                return None;
            }

            while !self.cur_token_is(&Token::SEMICOLON) {
                self.next_token()
            }

            Some(stmt)
        } else {
            None
        }
    }

    fn parse_return_statement(&mut self) -> Option<Stmt> {
        let stmt = Stmt::Return;

        self.next_token();

        while !self.cur_token_is(&Token::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_expression_statement(&mut self) -> Option<Stmt> {
        if let Some(expr) = self.parse_expression(Priority::LOWEST) {
            let stmt = Stmt::Expr(expr);

            if self.peek_token_is(&Token::SEMICOLON) {
                self.next_token()
            };

            Some(stmt)
        } else {
            None
        }
    }

    fn parse_expression(&mut self, priority: Priority) -> Option<Expr> {

        // prefix
        let mut left_opt = match self.cur_token {
                Token::IDENT(_) => self.parse_identifier(),
                Token::INT(_) => self.parse_integer_literal(),
                Token::TRUE
                | Token::FALSE => self.parse_bool_literal(),
                Token::PLUS => self.parse_prefix_expr(),
                Token::MINUS => self.parse_prefix_expr(),
                Token::BANG => self.parse_prefix_expr(),
                Token::LPAREN => self.parse_grouped_expr(),
                _ => {
                    self.errors.push(format!("unknown token in expression. got {:?}", self.cur_token));
                    None
                }
            };
        
        // infix
        if let Some(mut left) = left_opt {
            // not end of a statement and next token has more priority than current token
            while !self.peek_token_is(&Token::SEMICOLON) && priority < self.peek_priority() {
                left_opt = match self.peek_token {
                    | Token::PLUS
                    | Token::MINUS
                    | Token::SLASH
                    | Token::ASTERISK
                    | Token::EQ
                    | Token::NOTEQ
                    | Token::LT
                    | Token::GT => {
                        self.next_token();
                        self.parse_infix_expr(left)
                    }
                    _ => return Some(left)
                };
                left = left_opt.expect("expression function not found.")
            }
            Some(left)
        } else {
            None
        }
    }

    fn parse_identifier(&self) -> Option<Expr> {
        if let Token::IDENT(val) = &self.cur_token {
            Some(Expr::Ident(Ident(val.clone())))
        } else {
            None
        }
    }

    fn parse_integer_literal(&mut self) -> Option<Expr> {
        if let Token::INT(val) = self.cur_token {
            Some(Expr::Literal(Literal::Int(val)))
        } else {
            None
        }
    }

    fn parse_bool_literal(&mut self) -> Option<Expr> {
        match self.cur_token {
            Token::TRUE => Some(Expr::Literal(Literal::Bool(true))),
            Token::FALSE => Some(Expr::Literal(Literal::Bool(false))),
            _ => None
        }
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let cur_token = self.cur_token.clone();// PLUS

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

    fn parse_infix_expr(&mut self, left: Expr) -> Option<Expr> {
        let cur_token = self.cur_token.clone();// PLUS
        let priority = self.cur_priority(); // SUM
        self.next_token();

        if let Some(right) = self.parse_expression(priority) {
            match Infix::from_token(&cur_token) {
                Ok(infix) => Some(Expr::Infix(Box::new(left), infix, Box::new(right))),
                Err(err) => {
                    self.errors.push(err);
                    None
                },
            }
        } else {
            None
        }
    }

    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        self.next_token();

        let expr = self.parse_expression(Priority::LOWEST);

        if !self.expect_peek(&Token::RPAREN) {
            None
        } else {
            expr
        }
    }

    fn cur_token_is(&self, tok: &Token) -> bool {
        self.cur_token == *tok
    }

    fn peek_token_is(&self, tok: &Token) -> bool {
        self.peek_token == *tok
    }

    fn expect_peek(&mut self, tok: &Token) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            true
        } else {
            self.peek_error(tok);
            false
        }
    }

    fn peek_error(&mut self, tok: &Token) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} instead",
            tok, self.peek_token
        ))
    }

    fn get_priority(tok: &Token) -> Priority {
        match tok {
            Token::EQ => Priority::EQUALS,
            Token::NOTEQ => Priority::EQUALS,
            Token::LT => Priority::LESSGREATER,
            Token::GT => Priority::LESSGREATER,
            Token::PLUS => Priority::SUM,
            Token::MINUS => Priority::SUM,
            Token::SLASH => Priority::PRODUCT,
            Token::ASTERISK => Priority::PRODUCT,
            _ => Priority::LOWEST,
        }
    }

    fn peek_priority(&self) -> Priority {
        Parser::get_priority(&self.peek_token)
    }

    fn cur_priority(&self) -> Priority {
        Parser::get_priority(&self.cur_token)
    }
}
