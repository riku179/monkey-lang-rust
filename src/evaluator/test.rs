use super::*;
use crate::lexer::Lexer;
use crate::object::{Env, EvalResult, Object};
use crate::parser::Parser;

fn test_eval(input: &str) -> EvalResult<Object> {
    let mut l = Lexer::new(input.to_string()).unwrap();
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    println!("{:?}", program);
    let mut env = Env::new();

    eval(program, &mut env)
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
        assert_eq!(evaluated, EvalResult::Ok(Object::Int(expect)));
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
        assert_eq!(evaluated, EvalResult::Ok(Object::Bool(expect)));
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
        assert_eq!(evaluated, EvalResult::Ok(Object::Bool(expect)));
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
        match expect {
            Some(v) => assert_eq!(evaluated, EvalResult::Ok(Object::Int(v))),
            _ => assert_eq!(evaluated, EvalResult::Ok(Object::Null)),
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
        (
            r###"
        if (10 > 1) {
            if (10 > 1) {
                return 10;
            }
            return 1;
        }
        "###,
            10,
        ),
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input);
        assert_eq!(evaluated, EvalResult::Ok(Object::Int(expect)))
    }
}

#[test]
fn test_error_handling() {
    let test_cases = vec![
        ("5 + true;", "type mismatch: INT + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INT + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("5; true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        (
            r###"
        if (10 > 1) {
            if (10 > 1) {
                return true + false;
            }
            return 1;
        }
        "###,
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        ("foobar", "identifier not found: foobar"),
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input);
        assert_eq!(evaluated, EvalResult::Err(EvalError(expect.to_string())))
    }
}

#[test]
fn test_let_stmt() {
    let test_cases = vec![
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input);
        assert_eq!(evaluated, EvalResult::Ok(Object::Int(expect)))
    }
}

#[test]
fn test_function_obj() {
    let input = "fn(x) { x + 2; };";

    if let Object::Func(func) = test_eval(input).expect("sucess evaluation") {
        assert_eq!(format!("{}", func), "fn (x) { (x + 2) }")
    } else {
        unreachable!();
    };
}

#[test]
fn test_function_application() {
    let test_cases = vec![
        ("let identity = fn(x) { x; }; identity(5)", 5),
        ("let identity = fn(x) { return x; }; identity(5)", 5),
        ("let double = fn(x) { x * 2; }; double(5)", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5)", 10),
        ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5))", 20),
        ("fn(x) { x; }(5)", 5),
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input);
        assert_eq!(evaluated, EvalResult::Ok(Object::Int(expect)))
    }
}

#[test]
fn test_closures() {
    let input = r#"
        let newAdder = fn(x) { fn(y) { x + y } };
        let addTwo = newAdder(2);
        addTwo(2);
    "#;

    assert_eq!(test_eval(input), EvalResult::Ok(Object::Int(4)))
}
