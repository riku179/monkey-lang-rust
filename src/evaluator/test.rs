use super::*;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

fn test_eval(input: &str) -> Option<Object> {
    let mut l = Lexer::new(input.to_string()).unwrap();
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    println!("{:?}", program);

    return eval(program);
}

#[test]
fn test_eval_int_expr() {
    let test_cases = vec![
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input);
        assert_eq!(evaluated, Some(Object::Int(expect)));
    }
}

#[test]
fn test_eval_bool_expr() {
    let test_cases = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input);
        assert_eq!(evaluated, Some(Object::Bool(expect)));
    }
}

#[test]
fn test_bang_operator() {
    let test_cases = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input);
        assert_eq!(evaluated, Some(Object::Bool(expect)));
    }
}

#[test]
fn test_if_else_expr() {
    let test_cases = vec![
        ("if (true) { 10 }", Some(10)),
        ("if (false) { 10 }", None),
        ("if (1) { 10 }", Some(10)),
        ("if (1 < 2) { 10 }", Some(10)),
        ("if (1 > 2) { 10 }", None),
        ("if (1 > 2) { 10 } else { 20 }", Some(20)),
        ("if (1 < 2) { 10 } else { 20 }", Some(10)),
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input);
        if let Some(v) = expect {
            assert_eq!(evaluated, Some(Object::Int(v)))
        } else {
            assert_eq!(evaluated, None)
        }
    }
}

#[test]
fn test_return_stmt() {
    let test_cases = vec![
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("return 10;", 10),
        ("9; return 2 * 5; 9;;", 10),
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input);
        assert_eq!(evaluated, Some(Object::Int(expect)))
    }
}
