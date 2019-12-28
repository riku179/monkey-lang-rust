use crate::ast::{Expr, Ident, Infix, Literal, Prefix, Program, Stmt};
use crate::object::Object;
use std::fmt::{Debug, Display};

mod test;

pub fn eval(p: Program) -> Option<Object> {
    let mut result = None;
    for stmt in p.statements {
        result = eval_stmt(stmt)
    }
    result
}

fn eval_stmt(stmt: Stmt) -> Option<Object> {
    match stmt {
        Stmt::Expr(expr) => eval_expr(expr),
        Stmt::Block(stmts) => {
            let mut result = None;
            for stmt in stmts {
                result = eval_stmt(stmt)
            }
            result
        }
        _ => None,
    }
}

fn eval_expr(expr: Expr) -> Option<Object> {
    match expr {
        Expr::Literal(literal) => Some(eval_literal(literal)),
        Expr::Prefix(prefix, right) => Some(eval_prefix_expr(prefix, eval_expr(*right)?)),
        Expr::Infix(left, infix, right) => Some(eval_infix_expr(
            infix,
            eval_expr(*left)?,
            eval_expr(*right)?,
        )),
        Expr::If(cond, cons, alt) => eval_if_expr(*cond, *cons, alt),
        _ => None,
    }
}

fn eval_literal(literal: Literal) -> Object {
    match literal {
        Literal::Bool(v) => Object::Bool(v),
        Literal::Int(v) => Object::Int(v),
    }
}

fn eval_prefix_expr(operator: Prefix, right: Object) -> Object {
    match operator {
        Prefix::Not => eval_bang_operator_expr(right),
        Prefix::Minus => eval_minus_operator_expr(right),
        _ => Object::Null,
    }
}

fn eval_infix_expr(operator: Infix, left: Object, right: Object) -> Object {
    if let Object::Int(left_val) = left {
        if let Object::Int(right_val) = right {
            eval_int_infix_expr(operator, left_val, right_val)
        } else {
            unreachable!()
        }
    } else {
        match operator {
            Infix::Equal => Object::Bool(left == right),
            Infix::NotEqual => Object::Bool(left != right),
            _ => unreachable!(),
        }
    }
}

fn eval_int_infix_expr(operator: Infix, left: i64, right: i64) -> Object {
    match operator {
        Infix::Plus => Object::Int(left + right),
        Infix::Minus => Object::Int(left - right),
        Infix::Multiply => Object::Int(left * right),
        Infix::Divide => Object::Int(left / right),
        Infix::LessThan => Object::Bool(left < right),
        Infix::GreaterThan => Object::Bool(left > right),
        Infix::Equal => Object::Bool(left == right),
        Infix::NotEqual => Object::Bool(left != right),
        _ => unreachable!(),
    }
}

fn eval_bang_operator_expr(right: Object) -> Object {
    match right {
        Object::Bool(val) => Object::Bool(!val),
        Object::Null => Object::Bool(true),
        _ => Object::Bool(false),
    }
}

fn eval_minus_operator_expr(right: Object) -> Object {
    if let Object::Int(val) = right {
        Object::Int(-val)
    } else {
        Object::Null
    }
}

fn eval_if_expr(cond: Expr, cons: Stmt, alt: Option<Box<Stmt>>) -> Option<Object> {
    let cond_obj = eval_expr(cond);

    if is_truthy(cond_obj) {
        return eval_stmt(cons);
    };

    if let Some(stmt) = alt {
        return eval_stmt(*stmt);
    };

    None
}

fn is_truthy(obj: Option<Object>) -> bool {
    match obj {
        Some(Object::Null) => false,
        Some(Object::Bool(true)) => true,
        Some(Object::Bool(false)) => false,
        _ => true,
    }
}
