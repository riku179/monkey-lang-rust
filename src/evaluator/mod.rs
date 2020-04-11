use crate::ast::{Expr, Infix, Literal, Prefix, Program, Stmt};
use crate::object::{Env, Object};

mod test;

pub fn eval(p: Program, env: &mut Env) -> Option<Object> {
    let mut result = None;

    for stmt in p.statements {
        result = eval_stmt(env, stmt);

        if let Some(Object::Return(box val)) = result {
            return val;
        };

        if let Some(err @ Object::Error(_)) = result {
            return Some(err);
        };
    }

    result
}

fn eval_stmt(env: &mut Env, stmt: Stmt) -> Option<Object> {
    match stmt {
        Stmt::Expr(expr) => eval_expr(env, expr),
        Stmt::Block(stmts) => eval_block_stmt(env, stmts),
        Stmt::Return(expr) => {
            let val = eval_expr(env, expr);
            Some(Object::Return(Box::new(val)))
        }
        _ => None,
    }
}

fn eval_expr(env: &mut Env, expr: Expr) -> Option<Object> {
    match expr {
        Expr::Literal(literal) => Some(eval_literal(literal)),
        Expr::Prefix(prefix, right) => Some(eval_prefix_expr(prefix, eval_expr(env, *right)?)),
        Expr::Infix(left, infix, right) => Some(eval_infix_expr(
            infix,
            eval_expr(env, *left)?,
            eval_expr(env, *right)?,
        )),
        Expr::If(cond, cons, alt) => eval_if_expr(env, *cond, *cons, alt),
        _ => None,
    }
}

fn eval_block_stmt(env: &mut Env, block: Vec<Stmt>) -> Option<Object> {
    let mut result = None;
    for stmt in block {
        result = eval_stmt(env, stmt);

        if let Some(Object::Return(_)) = result {
            return result;
        };

        if let Some(Object::Error(_)) = result {
            return result;
        };
    }
    result
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
        _ => Object::Error(format!(
            "unknown operator: {}{}",
            operator,
            right.get_type()
        )),
    }
}

fn eval_infix_expr(operator: Infix, left: Object, right: Object) -> Object {
    if let Object::Int(left_val) = left {
        if let Object::Int(right_val) = right {
            eval_int_infix_expr(operator, left_val, right_val)
        } else {
            Object::Error(format!(
                "type mismatch: {} {} {}",
                left.get_type(),
                operator,
                right.get_type()
            ))
        }
    } else {
        match operator {
            Infix::Equal => Object::Bool(left == right),
            Infix::NotEqual => Object::Bool(left != right),
            _ => Object::Error(format!(
                "unknown operator: {} {} {}",
                left.get_type(),
                operator,
                right.get_type()
            )),
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
        Object::Error(format!("unknown operator: -{}", right.get_type()))
    }
}

fn eval_if_expr(env: &mut Env, cond: Expr, cons: Stmt, alt: Option<Box<Stmt>>) -> Option<Object> {
    let cond_obj = eval_expr(env, cond);

    if is_truthy(cond_obj) {
        return eval_stmt(env, cons);
    };

    if let Some(stmt) = alt {
        return eval_stmt(env, *stmt);
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
