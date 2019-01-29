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
    let test_cases = vec![
        (
            "let x = 5;",
            "x",
            Expr::Literal(Literal::Int(5))
        ),
        (
            "let y = true;",
            "y",
            Expr::Literal(Literal::Bool(true))
        ),
        (
            "let foobar = y;",
            "foobar",
            Expr::Ident(Ident("y".to_string()))
        )
    ];

    for (input, expect_ident, expect_expr) in test_cases {
        let mut lex = Lexer::new(input.to_string()).unwrap();
        let mut p = Parser::new(&mut lex);

        let program = p.parse_program();
        check_parser_errors(p);
        check_stmt_len(&program, 1);

        if let Stmt::Let(ident, expr) = &program.statements[0] {
            assert_eq!(format!("{}", ident), expect_ident);
            assert_eq!(*expr, expect_expr);
        } else {
            unreachable!()
        }
    }
}

#[test]
fn test_return_stmt() {
    let test_cases = vec![
        (
            "return 5;",
            Expr::Literal(Literal::Int(5))
        ),
        (
            "return true;",
            Expr::Literal(Literal::Bool(true))
        ),
        (
            "return foobar;",
            Expr::Ident(Ident("foobar".to_string()))
        )
    ];

    for (input, expect_expr) in test_cases {
        let mut lex = Lexer::new(input.to_string()).unwrap();
        let mut psr = Parser::new(&mut lex);

        let program = psr.parse_program();
        check_parser_errors(psr);
        check_stmt_len(&program, 1);

        if let Stmt::Return(expr) = &program.statements[0] {
            assert_eq!(*expr, expect_expr);
        }
    }

}

#[test]
fn test_ident_expr() {
    let input = r#"foobar;"#.to_string();

    let mut lex = Lexer::new(input).unwrap();
    let mut p = Parser::new(&mut lex);

    let program = p.parse_program();
    check_parser_errors(p);
    check_stmt_len(&program, 1);

    util::check_stmt(&program.statements[0], "foobar")
}

#[test]
fn test_integer_literal_expr() {
    let input = r#"5;"#.to_string();

    let mut lex = Lexer::new(input).unwrap();
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
        let mut lex = Lexer::new(input.to_string()).unwrap();
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
        let mut l = Lexer::new(input.to_string()).unwrap();
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
            unreachable!()
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
        let mut l = Lexer::new(input.to_string()).unwrap();
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
        ),
        (
            "a + add(b * c) + d",
            "((a + add((b * c))) + d)"
        ),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))"
        )
    ];

    for (input, expect) in test_cases {
        let mut l = Lexer::new(input.to_string()).unwrap();
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(format!("{}", program), expect, "debug: {:?}", program);
    }
}

#[test]
fn test_if_expr() {
    let input = r#"if (x < y) { x }"#.to_string();

    let mut l = Lexer::new(input).unwrap();
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    check_parser_errors(p);
    check_stmt_len(&program, 1);

    if let Stmt::Expr(Expr::If(box cond, box Stmt::Block(cons_stmts), None)) = &program.statements[0] {
        util::check_infix_expr(cond, "x", Infix::LessThan, "y");
        util::check_stmt(&cons_stmts[0], "x");
    } else {
        unreachable!()
    }
}

#[test]
fn test_if_else_expr() {
    let input = r#"if (x < y) { x } else { y }"#.to_string();
    let mut l = Lexer::new(input).unwrap();
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    check_parser_errors(p);
    check_stmt_len(&program, 1);

    if let Stmt::Expr(Expr::If(box cond, box Stmt::Block(cons_stmts), Some(box Stmt::Block(alter_stmts)))) = &program.statements[0] {
        util::check_infix_expr(cond, "x", Infix::LessThan, "y");
        util::check_stmt(&cons_stmts[0], "x");
        util::check_stmt(&alter_stmts[0], "y")
    } else {
        unreachable!()
    }
}

#[test]
fn test_function_literal_parse() {
    let input = r#"fn (x, y) { x + y; }"#.to_string();
    let mut l = Lexer::new(input).unwrap();
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    check_parser_errors(p);
    check_stmt_len(&program, 1);

    if let Stmt::Expr(Expr::Function(params, box Stmt::Block(stmts))) = &program.statements[0] {
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].0, "x");
        assert_eq!(params[1].0, "y");

        assert_eq!(stmts.len(), 1);
        util::check_infix_stmt(&stmts[0], "x", Infix::Plus, "y");
    } else {
        unreachable!()
    }
}

#[test]
fn test_function_param_parse() {
    let test_cases = vec![
        (
            "fn() {};",
            vec![]
        ),
        (
            "fn(x) {};",
            vec!["x"]
        ),
        (
            "fn(x, y, z) {};",
            vec!["x", "y", "z"]
        )
    ];

    for (input, expect) in test_cases {
        let mut l = Lexer::new(input.to_string()).unwrap();
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(p);

        if let Stmt::Expr(Expr::Function(params, _)) = &program.statements[0] {
            assert_eq!(params.len(), expect.len());

            for (Ident(actual_param), expect_param) in params.iter().zip(expect.iter()) {
                assert_eq!(actual_param, expect_param);
            }
        } else {
            unreachable!()
        }
    }
}

#[test]
fn test_call_expr_parse() {
    let input = "add(1, 2 * 3, 4 + 5)".to_string();
    let mut l = Lexer::new(input).unwrap();
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    check_parser_errors(p);
    check_stmt_len(&program, 1);

    if let Stmt::Expr(Expr::Call(box func, params)) = &program.statements[0] {
        util::check_expr(func, "add");
        assert_eq!(params.len(), 3);
        util::check_expr(&params[0], 1);
        util::check_infix_expr(&params[1], 2, Infix::Multiply, 3);
        util::check_infix_expr(&params[2], 4, Infix::Plus, 5);
    } else {
        unreachable!()
    }
}

#[test]
fn test_call_expr_param_parse() {
    let test_cases = vec![
        (
            "add();",
            "add",
            vec![]
        ),
        (
            "add(1);",
            "add",
            vec!["1"]
        ),
        (
            "add(1, 2 * 3, 4 + 5);",
            "add",
            vec!["1", "(2 * 3)", "(4 + 5)"]
        )
    ];

    for (input, expect_ident, expect_args) in test_cases {
        let mut l = Lexer::new(input.to_string()).unwrap();
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parser_errors(p);

        if let Stmt::Expr(Expr::Call(box func, params)) = &program.statements[0] {
            util::check_expr(func, expect_ident);
            assert_eq!(params.len(), expect_args.len());
            for (actual, expect) in params.iter().zip(expect_args.iter()) {
                assert_eq!(format!("{}", actual), *expect);
            }
        } else {
            unreachable!()
        }
    }
}