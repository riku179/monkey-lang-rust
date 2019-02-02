use std::fmt::{Debug, Display};
use crate::ast::{Program, Ident, Stmt, Expr, Literal, Prefix, Infix};
use crate::object::Object;

mod test;

pub fn eval(p: Program) -> Option<Object> {
    let mut result = None;
    for stmt in p.statements {
        result = eval_stmt(stmt);
    }
    result
}

fn eval_stmt(stmt: Stmt) -> Option<Object> {
    match stmt {
        Stmt::Expr(expr) => eval_expr(expr),
        _ => None
    }
}

fn eval_expr(expr: Expr) -> Option<Object> {
    match expr {
        Expr::Literal(literal) => Some(eval_literal(literal)),
        Expr::Prefix(prefix, box right) => Some(eval_prefix_expr(prefix, eval_expr(right)?)),
        _ => None
    }
}

fn eval_literal(literal: Literal) -> Object {
    match literal {
        Literal::Bool(v) => Object::Bool(v),
        Literal::Int(v) => Object::Int(v)
    }
}

fn eval_prefix_expr(operator: Prefix, right: Object) -> Object {
    match operator {
        Prefix::Not => eval_bang_operator_expr(right),
        Prefix::Minus => eval_minus_operator_expr(right),
        _ => Object::Null
    }
}

fn eval_bang_operator_expr(right: Object) -> Object {
    match right {
        Object::Bool(val) => Object::Bool(!val),
        Object::Null => Object::Bool(true),
        _ => Object::Bool(false)
    }
}

fn eval_minus_operator_expr(right: Object) -> Object {
    if let Object::Int(val) = right {
        Object::Int(-val)
    } else {
        Object::Null
    }
}