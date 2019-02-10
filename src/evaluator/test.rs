use super::*;
use crate::object::Object;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn test_eval(input: &str) -> Option<Object> {
    let mut l = Lexer::new(input.to_string()).unwrap();
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();
    println!("{:?}", program);

    return eval(program)
}

fn test_int_obj(obj: Object, expect: i64) {
    if let Object::Int(val) = obj {
        assert_eq!(val, expect);
    } else {
        unreachable!();
    }
}

fn test_bool_obj(obj: Object, expect: bool) {
    if let Object::Bool(val) = obj {
        assert_eq!(val, expect);
    } else {
        unreachable!()
    }
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
        let evaluated = test_eval(input).unwrap();
        test_int_obj(evaluated, expect);
    }
}

#[test]
fn test_eval_bool_expr() {
    let test_cases = vec![
        ("true", true),
        ("false", false)
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input).unwrap();
        test_bool_obj(evaluated, expect);
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
        ("!!5", true)
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input).unwrap();
        test_bool_obj(evaluated, expect);
    }
}