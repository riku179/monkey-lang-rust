use super::*;
use crate::{ast, ast::Statement};
use crate::lexer;
use ascii::AsciiString;
use crate::ast::Node;

#[test]
fn test_let_statements() {
    let input = AsciiString::from_ascii(
        r###"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "###,
    )
        .unwrap();

    let mut lex = lexer::Lexer::new(input);
    let mut p = Parser::new(&mut lex);

    let program = p.parse_program().expect("program");
    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements."
    );

    #[derive(Debug)]
    struct Expected {
        pub identifier: String,
    }

    let expected_results = [
        Expected {
            identifier: "x".to_string(),
        },
        Expected {
            identifier: "y".to_string(),
        },
        Expected {
            identifier: "foobar".to_string(),
        },
    ];

    for (i, expected) in expected_results.iter().enumerate() {
        let stmt = &program.statements[i];
        if !test_let_statement(stmt, &expected.identifier) {
            return
        }
    }
}

fn test_let_statement(stmt: &Statement, identifier_name: &String) -> bool {
    let literal = stmt.token_literal().expect("token literal");

    if literal != token::LET {
        eprintln!("token literal is not 'let'. got={:?}", literal);
        return false
    };
    
    match stmt {
        Statement::Let(ast::LetStatement{name, ..}) => {
            if name.value != *identifier_name {
                eprintln!("LetStatement.name.value not {:?}. got={:?}", identifier_name, name.value);
                false
            } else if name.token_literal().unwrap() != identifier_name {
                eprintln!("LetStatement.name not {:?}. got {:?}", identifier_name, name);
                false
            } else {
                true
            }
        }
        _ => {
            eprintln!("s not LetStatement. got={:?}", stmt);
            false
        }
    }
}

fn check_parser_errors(p: Parser) {
    let errors = p.errors;

    if errors.len() == 0 {
        return
    }

    eprintln!("parser has {} errors", errors.len());

    for msg in errors {
        eprintln!("parser error: \"{}\"", msg)
    };

    panic!("failed to parse!");
}
