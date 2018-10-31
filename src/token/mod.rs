use ascii::{AsciiChar, AsciiString};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: AsciiChar) -> Token {
        let mut literal = AsciiString::new();
        literal.push(ch);
        Token {
            token_type,
            literal: literal.into(),
        }
    }

    pub fn illegal() -> Token {
        let literal = AsciiString::new();
        Token {
            token_type: ILLEGAL,
            literal: literal.into(),
        }
    }

    pub fn new_by_literal(ident: AsciiString) -> Token {
        let token_type = if let Some(tok) = KEYWORDS.get(ident.as_str()) {
            tok
        } else {
            IDENT
        };
        Token {
            token_type,
            literal: ident.into(),
        }
    }
}

pub type TokenType = &'static str;

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

// Identifiers + literals
pub const IDENT: TokenType = "IDENT"; // add, foobar, x, y, ...
pub const INT: TokenType = "INT"; // 1343456

// Operators
pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const MINUS: TokenType = "-";
pub const BANG: TokenType = "!";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";

pub const LT: TokenType = "<";
pub const GT: TokenType = ">";

pub const EQ: TokenType = "==";
pub const NOT_EQ: TokenType = "!=";

// Delimiters
pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";

pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";

// Keywords
pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";
pub const TRUE: TokenType = "TRUE";
pub const FALSE: TokenType = "FALSE";
pub const IF: TokenType = "IF";
pub const ELSE: TokenType = "ELSE";
pub const RETURN: TokenType = "RETURN";

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn", FUNCTION);
        m.insert("let", LET);
        m.insert("true", TRUE);
        m.insert("false", FALSE);
        m.insert("if", IF);
        m.insert("else", ELSE);
        m.insert("return", RETURN);
        m
    };
}
