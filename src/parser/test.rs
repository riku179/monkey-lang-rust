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

    let program = p.parse_program();
    check_parser_errors(p);
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
    
    if let Statement::Let(ast::LetStatement{name, ..}) = stmt {
        if name.value != *identifier_name {
            eprintln!("LetStatement.name.value not {:?}. got={:?}", identifier_name, name.value);
            false
        } else if name.token_literal().unwrap() != identifier_name {
            eprintln!("LetStatement.name not {:?}. got {:?}", identifier_name, name);
            false
        } else {
            true
        }
    } else {
        eprintln!("s not LetStatement. got={:?}", stmt);
        false
    }
}

#[test]
fn test_return_statement() {
    let input = AsciiString::from_ascii(r###"
    return 5;
    return 10;
    return 993322;
    "###).unwrap();
    
    let mut lex = lexer::Lexer::new(input);
    let mut psr = Parser::new(&mut lex);
    
    let program = psr.parse_program();
    check_parser_errors(psr);
    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements.",
     );
    
    for stmt in program.statements {
        if let Statement::Return(ast::ReturnStatement{token, ..}) = stmt {
            assert_eq!(token.literal, "return")
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
