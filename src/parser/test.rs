use ascii::AsciiString;
use super::*;
use super::test_util::Literable;
use super::test_util as util;
use crate::ast::{Expr, Ident, Literal, Prefix, Infix, Stmt};
use crate::lexer::Lexer;


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

fn check_stmt_len(program: &Program, len: usize) {
    assert_eq!(
        program.statements.len(),
        len,
        "program.statements does not contain {} statements. got {}",
        len,
        program
    );
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

    let mut lex = Lexer::new(input);
    let mut p = Parser::new(&mut lex);

    let program = p.parse_program();
    check_parser_errors(p);
    check_stmt_len(&program, 3);

    let expected_results = vec!["x", "y", "foobar"];

    for (i, expected) in expected_results.iter().enumerate() {
        util::check_let_stmt(&program.statements[i], expected)
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

    let mut lex = Lexer::new(input);
    let mut psr = Parser::new(&mut lex);

    let program = psr.parse_program();
    check_parser_errors(psr);
    check_stmt_len(&program, 3);

    for stmt in program.statements {
        assert_eq!(stmt, Stmt::Return);
    }
}

#[test]
fn test_ident_expr() {
    let input = AsciiString::from_ascii(r#"foobar;"#).unwrap();

    let mut lex = Lexer::new(input);
    let mut p = Parser::new(&mut lex);

    let program = p.parse_program();
    check_parser_errors(p);
    check_stmt_len(&program, 1);

    util::check_stmt(&program.statements[0], "foobar")
}

#[test]
fn test_integer_literal_expr() {
    let input = AsciiString::from_ascii(r#"5;"#).unwrap();

    let mut lex = Lexer::new(input);
    let mut p = Parser::new(&mut lex);

    let program = p.parse_program();
    check_parser_errors(p);
    check_stmt_len(&program, 1);

    util::check_stmt(&program.statements[0], 5)
}

#[test]
fn test_boolean_literal_expr() {
    let test_cases = vec![
        ("true;", true),
        ("false;", false)
    ];

    for (input, expect) in test_cases {
        let mut lex = Lexer::new(AsciiString::from_ascii(input).unwrap());
        let mut p = Parser::new(&mut lex);
        let program = p.parse_program();

        check_parser_errors(p);
        check_stmt_len(&program, 1);
        util::check_stmt(&program.statements[0], expect)
    }
}

#[test]
fn test_parse_prefix_expr() {
    let prefix_tests = vec![
            ("!5;", Prefix::Not, 5),
            ( "-15;", Prefix::Minus, 15),
    ];

    for (input, expect_prefix, expect_val) in prefix_tests {
        let mut l = Lexer::new(AsciiString::from_ascii(input).unwrap());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(p);
        check_stmt_len(&program, 1);

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
        let mut l = Lexer::new(AsciiString::from_ascii(input).unwrap());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(p);
        check_stmt_len(&program, 1);

        util::check_infix_stmt(&program.statements[0], expect_left, expect_infix, expect_right)
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let test_cases = vec![
        (
            "-a * b",
            "((-a) * b)",
        ),
        (
            "!-a",
            "(!(-a))"
        ),
        (
            "a + b + c",
            "((a + b) + c)"
        ),
        (
            "a + b - c",
            "((a + b) - c)"
        ),
        (
            "a * b * c",
            "((a * b) * c)"
        ),
        (
            "a * b / c",
            "((a * b) / c)"
        ),
        (
            "a + b / c",
            "(a + (b / c))"
        ),
        (
            "a + b * c + d / e - f",
            "(((a + (b * c)) + (d / e)) - f)"
        ),
        (
            "3 + 4; -5 * 5",
            "(3 + 4)((-5) * 5)"
        ),
        (
            "5 > 4 == 3 < 4",
            "((5 > 4) == (3 < 4))"
        ),
        (
            "5 < 4 != 3 > 4",
            "((5 < 4) != (3 > 4))"
        ),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"
        ),
        (
            "1 + (2 + 3) + 4",
            "((1 + (2 + 3)) + 4)"
        ),
        (
            "(5 + 5) * 2",
            "((5 + 5) * 2)"
        ),
        (
            "2 / (5 + 5)",
            "(2 / (5 + 5))"
        ),
        (
            "-(5 + 5)",
            "(-(5 + 5))"
        ),
        (
            "!(true == true)",
            "(!(true == true))"
        )
    ];

    for (input, expect) in test_cases {
        let mut l = Lexer::new(AsciiString::from_ascii(input).unwrap());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(format!("{}", program), expect, "debug: {:?}", program);
    }
}

#[test]
fn test_if_expr() {
    let input = r#"if (x < y) { x }"#;

    let mut l = Lexer::new(AsciiString::from_ascii(input).unwrap());
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    check_parser_errors(p);
    check_stmt_len(&program, 1);

    if let Stmt::Expr(Expr::If(box cond, box cons, alter)) = &program.statements[0] {
        util::check_infix_expr(cond, "x", Infix::LessThan, "y");
        util::check_stmt(cons, "x");
        assert!(alter.is_none())
    }
}