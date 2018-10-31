use ast;
use lexer;
use token;

#[derive(Debug)]
struct Parser<'a> {
    lex: &'a lexer::Lexer,

    cur_token: &'a token::Token,
    peek_token: &'a token::Token,
}

impl<'a> Parser<'a> {
    fn new<'a>(lex: &'a mut lexer::Lexer) -> Parser<'a> {
        let mut parser = Parser {
            lex,
            cur_token: lex.next_token(),
            peek_token: lex.next_token(),
        };
        // 2つトークンを読む。cur_tokenとpeek_tokenがセットされる
        parser
    }

    fn next_token(&mut self) -> () {
        self.cur_token = self.peek_token;
        self.peek_token = self.lex.next_token()
    }

    // fn parse_program() -> ast::Program {
    // }
}