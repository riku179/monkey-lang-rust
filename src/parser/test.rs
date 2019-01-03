use super::*;
use crate::ast::{Expr, Ident, Literal, Prefix, Infix, Stmt};
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
        "program.statements does not contain 3 statements. got {:?}",
        program.statements
    );

    let expected_results = vec!["x", "y", "foobar"];

    for (i, expected) in expected_results.iter().enumerate() {
        let stmt = &program.statements[i];
        if let Stmt::Let(Ident(name)) = stmt {
            assert_eq!(name, expected);
        } else {
            panic!(format!("Type error. got {:?}", &program.statements[i]));
        }
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
        "program.statements does not contain 3 statements. got {:?}",
        program.statements
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
        "program.statements does not contain 1 statements. {:?}",
        program.statements
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
        "program.statements does not contain 1 statements. {:?}",
        program.statements
    );

    if let Stmt::Expr(Expr::Literal(Literal::Int(val))) = &program.statements[0] {
        assert_eq!(*val, 5);
    } else {
        panic!(format!("Type error. got {:?}", &program.statements[0]));
    }
}

#[test]
fn test_parse_prefix_expr() {
    let prefix_tests = vec![
            ("!5;", Prefix::Not, 5),
            ( "-15;", Prefix::Minus, 15),
    ];

    for (input, expect_prefix, expect_val) in prefix_tests {
        let mut l = lexer::Lexer::new(AsciiString::from_ascii(input).unwrap());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statements. got {:?}",
            program.statements
        );

        if let Stmt::Expr(Expr::Prefix(prefix, box Expr::Literal(Literal::Int(val)))) =
            &program.statements[0]
        {
            assert_eq!(*prefix, expect_prefix);
            assert_eq!(*val, expect_val);
        } else {
            panic!(format!("Type error. got {:?}", &program.statements[0]));
        };
    }
}

#[test]
fn test_parse_infix_expr() {
    let infix_tests = vec![
        ( "5 + 5;", 5, Infix::Plus, 5),
        ( "5 - 5", 5, Infix::Minus, 5),
        ( "5 * 5", 5, Infix::Multiply, 5),
        ( "5 / 5", 5, Infix::Divide, 5),
        ( "5 > 5", 5, Infix::GreaterThan, 5),
        ( "5 < 5", 5, Infix::LessThan, 5),
        ( "5 == 5", 5, Infix::Equal, 5),
        ( "5 != 5", 5, Infix::NotEqual, 5),
    ];

    for (input, expect_left, expect_infix, expect_right) in infix_tests {
        let mut l = lexer::Lexer::new(AsciiString::from_ascii(input).unwrap());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statements. got {:?}",
            program.statements
        );

        if let Stmt::Expr(Expr::Infix(box Expr::Literal(Literal::Int(left)), infix, box Expr::Literal(Literal::Int(right)))) = &program.statements[0] {
            assert_eq!(*left, expect_left);
            assert_eq!(*infix, expect_infix);
            assert_eq!(*right, expect_right);
        } else {
            panic!(format!("Type error. got {:?}", &program.statements[0]));
        }
    }
}