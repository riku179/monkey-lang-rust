// use ast;
use lexer;
use token;
use ast;

#[derive(Debug)]
struct Parser<'a> {
    lex: &'a mut lexer::Lexer,

    cur_token: token::Token,
    peek_token: token::Token,
}

impl<'a> Parser<'a> {
    fn new(lex: &'a mut lexer::Lexer) -> Parser<'a> {
        let cur_token = lex.next_token();
        let peek_token = lex.next_token();
        let parser = Parser {
            lex,
            cur_token,
            peek_token,
        };
        parser
    }

    fn next_token(&mut self) -> () {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token()
    }

    fn parse_program(&self) -> ast::Program {
    }
}

#[cfg(test)]
mod test {
    use ascii::AsciiString;
    use super::*;
    use ast;
    use lexer;

    #[test]
    fn test_let_statements() {
        let input = AsciiString::from_ascii(r###"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "###).unwrap();

        let mut lex = lexer::Lexer::new(input);
        let p = Parser::new(&mut lex);

        let program = p.parse_program();
        assert_eq!(program.statements.len(), 3, "program.statements does not contain 3 statements.");

        #[derive(Debug)]
        struct Expected {
            pub identifier: String
        }

        let expected_results = [
            Expected{ identifier: "x".to_string() },
            Expected{ identifier: "y".to_string() },
            Expected{ identifier: "foobar".to_string() },
        ];

        for (i, expected) in expected_results.iter().enumerate() {
            let stmt = program.statements[i];
            test_let_statement(stmt, expected.identifier);
        }
    }

    fn test_let_statement<S: ast::Statement>(s: S, name: String) -> () {
        let literal = s.token_literal().expect("token literal");
        assert_eq!(*literal, "let");
    }
}
