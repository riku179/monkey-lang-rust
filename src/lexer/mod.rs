use crate::token::Token;
use ascii::{AsciiChar, AsciiString, FromAsciiError};

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
    pub fn new(input: String) -> Result<Lexer, FromAsciiError<String>> {
        let mut l = Lexer {
            input: AsciiString::from_ascii(input)?,
            position: 0,
            read_position: 0,
            ch: AsciiChar::Null,
        };
        l.read_char();
        Ok(l)
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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token;
        match self.ch {
            AsciiChar::Equal => {
                if self.peek_char() == AsciiChar::Equal {
                    let current_ch = self.ch;
                    self.read_char();
                    let mut literal = AsciiString::new();
                    literal.push(current_ch);
                    literal.push(self.ch);
                    tok = Token::EQ
                } else {
                    tok = Token::ASSIGN
                }
            }
            AsciiChar::Semicolon => tok = Token::SEMICOLON,
            AsciiChar::ParenOpen => tok = Token::LPAREN,
            AsciiChar::ParenClose => tok = Token::RPAREN,
            AsciiChar::Comma => tok = Token::COMMA,
            AsciiChar::CurlyBraceOpen => tok = Token::LBRACE,
            AsciiChar::CurlyBraceClose => tok = Token::RBRACE,
            AsciiChar::Plus => tok = Token::PLUS,
            AsciiChar::Minus => tok = Token::MINUS,
            AsciiChar::Exclamation => {
                if self.peek_char() == AsciiChar::Equal {
                    let current_ch = self.ch;
                    self.read_char();
                    let mut literal = AsciiString::new();
                    literal.push(current_ch);
                    literal.push(self.ch);
                    tok = Token::NOTEQ;
                } else {
                    tok = Token::BANG;
                }
            }
            AsciiChar::Slash => tok = Token::SLASH,
            AsciiChar::Asterisk => tok = Token::ASTERISK,
            AsciiChar::LessThan => tok = Token::LT,
            AsciiChar::GreaterThan => tok = Token::GT,
            _ => {
                if self.is_letter() {
                    let ident = self.read_identifier();
                    return match ident.as_str() {
                        "fn" => Token::FUNCTION,
                        "let" => Token::LET,
                        "true" => Token::TRUE,
                        "false" => Token::FALSE,
                        "if" => Token::IF,
                        "else" => Token::ELSE,
                        "return" => Token::RETURN,
                        _ => Token::IDENT(ident)
                    }
                } else if self.ch.is_digit() {
                    return Token::INT(self.read_number());
                };
                tok = Token::EOF
            }
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let mut literal = AsciiString::new();
        while self.is_letter() {
            literal.push(self.ch);
            self.read_char();
        }
        literal.to_string()
    }

    fn read_number(&mut self) -> i64 {
        let mut literal = AsciiString::new();
        while self.ch.is_digit() {
            literal.push(self.ch);
            self.read_char();
        }
        literal.to_string().parse().expect("failed to parse number")
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
