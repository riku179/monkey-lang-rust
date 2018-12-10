use crate::token;
use ascii::{AsciiChar, AsciiString};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Lexer {
    input: AsciiString,
    position: usize,
    read_position: usize,
    ch: AsciiChar,
}

impl Lexer {
    pub fn new(input: AsciiString) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: AsciiChar::Null,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if let Some(&ch) = self.input.chars().nth(self.read_position) {
            self.ch = ch
        } else {
            self.ch = AsciiChar::Null
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        let tok;
        match self.ch {
            AsciiChar::Equal => {
                if self.peek_char() == AsciiChar::Equal {
                    let current_ch = self.ch;
                    self.read_char();
                    let mut literal = AsciiString::new();
                    literal.push(current_ch);
                    literal.push(self.ch);
                    tok = token::Token {
                        token_type: token::EQ,
                        literal: literal.to_string(),
                    };
                } else {
                    tok = token::Token::new(token::ASSIGN, self.ch)
                }
            }
            AsciiChar::Semicolon => tok = token::Token::new(token::SEMICOLON, self.ch),
            AsciiChar::ParenOpen => tok = token::Token::new(token::LPAREN, self.ch),
            AsciiChar::ParenClose => tok = token::Token::new(token::RPAREN, self.ch),
            AsciiChar::Comma => tok = token::Token::new(token::COMMA, self.ch),
            AsciiChar::CurlyBraceOpen => tok = token::Token::new(token::LBRACE, self.ch),
            AsciiChar::CurlyBraceClose => tok = token::Token::new(token::RBRACE, self.ch),
            AsciiChar::Plus => tok = token::Token::new(token::PLUS, self.ch),
            AsciiChar::Minus => tok = token::Token::new(token::MINUS, self.ch),
            AsciiChar::Exclamation => {
                if self.peek_char() == AsciiChar::Equal {
                    let current_ch = self.ch;
                    self.read_char();
                    let mut literal = AsciiString::new();
                    literal.push(current_ch);
                    literal.push(self.ch);
                    tok = token::Token {
                        token_type: token::NOT_EQ,
                        literal: literal.to_string(),
                    };
                } else {
                    tok = token::Token::new(token::BANG, self.ch)
                }
            }
            AsciiChar::Slash => tok = token::Token::new(token::SLASH, self.ch),
            AsciiChar::Asterisk => tok = token::Token::new(token::ASTERISK, self.ch),
            AsciiChar::LessThan => tok = token::Token::new(token::LT, self.ch),
            AsciiChar::GreaterThan => tok = token::Token::new(token::GT, self.ch),
            _ => {
                if self.is_letter() {
                    return token::Token::new_by_literal(self.read_identifier());
                } else if self.ch.is_digit() {
                    return token::Token {
                        token_type: token::INT,
                        literal: self.read_number().to_string(),
                    };
                };
                tok = token::Token::new(token::EOF, AsciiChar::Null)
            }
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> AsciiString {
        let mut literal = AsciiString::new();
        while self.is_letter() {
            literal.push(self.ch);
            self.read_char();
        }
        literal
    }

    fn read_number(&mut self) -> AsciiString {
        let mut literal = AsciiString::new();
        while self.ch.is_digit() {
            literal.push(self.ch);
            self.read_char();
        }
        literal
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_blank() || self.ch.is_whitespace() {
            self.read_char()
        }
    }

    fn is_letter(&self) -> bool {
        self.ch.is_alphabetic() || self.ch == AsciiChar::UnderScore
    }

    fn peek_char(&mut self) -> AsciiChar {
        if let Some(&ch) = self.input.chars().nth(self.read_position) {
            ch
        } else {
            AsciiChar::Null
        }
    }
}
