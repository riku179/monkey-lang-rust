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

pub fn eval_stmt(stmt: Stmt) -> Option<Object> {
    match stmt {
        Stmt::Expr(expr) => eval_expr(expr),
        _ => None
    }
}

pub fn eval_expr(expr: Expr) -> Option<Object> {
    match expr {
        Expr::Literal(literal) => Some(eval_literal(literal)),
        _ => None
    }
}

pub fn eval_literal(literal: Literal) -> Object {
    match literal {
        Literal::Bool(v) => Object::Bool(v),
        Literal::Int(v) => Object::Int(v)
    }
}