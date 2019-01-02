use super::*;
use crate::ast::{Expr, Ident, Literal, Prefix, Stmt};
use crate::lexer;
use ascii::AsciiString;

fn check_parser_errors(p: Parser) {
    let errors = p.errors;

    if errors.len() == 0 {
        return;
    }

    eprintln!("parser has {} errors", errors.len());

    for msg in errors {
        eprintln!("parser error: \"{}\"", msg)
    }

    panic!("failed to parse!");
}

#[test]
fn test_let_stmts() {
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
        pub identifier: &'static str,
    }

    let expected_results = [
        Expected { identifier: "x" },
        Expected { identifier: "y" },
        Expected {
            identifier: "foobar",
        },
    ];

    for (i, expected) in expected_results.iter().enumerate() {
        let stmt = &program.statements[i];
        if !test_let_stmt(stmt, &expected.identifier.to_string()) {
            return;
        }
    }
}

fn test_let_stmt(stmt: &Stmt, identifier_name: &String) -> bool {
    if let Stmt::Let(Ident(name)) = stmt {
        name == identifier_name
    } else {
        eprintln!("s not LetStatement. got={:?}", stmt);
        false
    }
}

#[test]
fn test_return_stmt() {
    let input = AsciiString::from_ascii(
        r###"
        return 5;
        return 10;
        return 993322;
        "###,
    )
    .unwrap();

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
fn test_ident_expr() {
    let input = AsciiString::from_ascii(r#"foobar;"#).unwrap();

    let mut lex = lexer::Lexer::new(input);
    let mut p = Parser::new(&mut lex);

    let program = p.parse_program();
    check_parser_errors(p);
    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statements."
    );

    if let Stmt::Expr(Expr::Ident(Ident(value))) = &program.statements[0] {
        assert_eq!(value, "foobar");
    };
}

#[test]
fn test_integer_literal_expr() {
    let input = AsciiString::from_ascii(r#"5;"#).unwrap();

    let mut lex = lexer::Lexer::new(input);
    let mut p = Parser::new(&mut lex);

    let program = p.parse_program();
    check_parser_errors(p);
    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statements."
    );

    if let Stmt::Expr(Expr::Literal(Literal::Int(val))) = &program.statements[0] {
        assert_eq!(*val, 5);
    } else {
        panic!(format!("Type error. got {:?}", &program.statements[0]));
    }
}

#[test]
fn test_parse_prefix_expr() {
    #[derive(Debug)]
    struct PrefixTest {
        pub input: &'static str,
        pub prefix: Prefix,
        pub value: i64,
    }

    let prefix_tests = vec![
        PrefixTest {
            input: "!5;",
            prefix: Prefix::Not,
            value: 5,
        },
        PrefixTest {
            input: "-15;",
            prefix: Prefix::Minus,
            value: 15,
        },
    ];

    for test in prefix_tests {
        let mut l = lexer::Lexer::new(AsciiString::from_ascii(test.input).unwrap());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statements."
        );

        if let Stmt::Expr(Expr::Prefix(prefix, box Expr::Literal(Literal::Int(val)))) =
            &program.statements[0]
        {
            assert_eq!(*prefix, test.prefix);
            assert_eq!(*val, test.value);
        } else {
            panic!(format!("Type error. got {:?}", &program.statements[0]));
        };
    }
}
