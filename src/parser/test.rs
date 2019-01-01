use super::*;
use crate::ast::{Stmt, Expr, Ident, Literal};
use crate::lexer;
use ascii::AsciiString;

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

fn test_let_statement(stmt: &Stmt, identifier_name: &String) -> bool {
    if let Stmt::Let(Ident(name)) = stmt {
        name == identifier_name
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
        assert_eq!(stmt, Stmt::Return);
    }
}

#[test]
fn test_identifier_expression() {
    let input = AsciiString::from_ascii(r#"foobar;"#,).unwrap();

    let mut lex = lexer::Lexer::new(input);
    let mut p = Parser::new(&mut lex);

    let program = p.parse_program();
    check_parser_errors(p);   
    assert_eq!(program.statements.len(), 1, "program.statements does not contain 1 statements.");
    
    if let Stmt::Expr(Expr::Ident(Ident(value))) = &program.statements[0] {
        assert_eq!(value, "foobar");
    };
}

#[test]
fn test_integer_literal_expression() {
    let input = AsciiString::from_ascii(r#"5;"#).unwrap();

    let mut lex = lexer::Lexer::new(input);
    let mut p = Parser::new(&mut lex);

    let program = p.parse_program();
    check_parser_errors(p);
    assert_eq!(program.statements.len(), 1, "program.statements does not contain 1 statements.");

    if let Stmt::Expr(Expr::Literal(Literal::Int(val))) = &program.statements[0] {
        assert_eq!(*val, 5);
    }
}