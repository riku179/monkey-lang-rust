use super::*;
use crate::object::Object;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn test_eval_int_expr() {
    let test_cases = vec![
        ("5", 5),
        ("10", 10)
    ];

    for (input, expect) in test_cases {
        let evaluated = test_eval(input).unwrap();
        test_int_obj(evaluated, expect);
    }
}

fn test_eval(input: &str) -> Option<Object> {
    let mut l = Lexer::new(input.to_string()).unwrap();
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();

    return eval(program)
}

fn test_int_obj(obj: Object, expect: i64) {
    if let Object::Int(val) = obj {
        assert_eq!(val, expect);
    } else {
        unreachable!();
    }
}