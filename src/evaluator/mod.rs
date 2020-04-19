use crate::ast::{Expr, Ident, Infix, Literal, Prefix, Program, Stmt};
use crate::object::{Env, EvalError, EvalResult, Func, Object};

mod test;

pub fn eval(p: Program, env: &mut Env) -> EvalResult<Object> {
    let mut result = Object::Null;

    for stmt in p.statements {
        result = eval_stmt(env, stmt)?;

        if let Object::Return(box val) = result {
            return Ok(val);
        };
    }

    Ok(result)
}

fn eval_stmt(env: &mut Env, stmt: Stmt) -> EvalResult<Object> {
    match stmt {
        Stmt::Expr(expr) => eval_expr(env, expr),
        Stmt::Block(stmts) => eval_block_stmt(env, stmts),
        Stmt::Return(expr) => {
            let val = eval_expr(env, expr);
            val.map(|v| Object::Return(Box::new(v)))
        }
        Stmt::Let(ident, expr) => {
            let val = eval_expr(env, expr)?;
            env.insert(ident.0, val);
            Ok(Object::Null)
        }
    }
}

fn eval_expr(env: &mut Env, expr: Expr) -> EvalResult<Object> {
    match expr {
        Expr::Literal(literal) => Ok(eval_literal(literal)),
        Expr::Prefix(prefix, right) => eval_prefix_expr(prefix, eval_expr(env, *right)?),
        Expr::Infix(left, infix, right) => {
            eval_infix_expr(infix, eval_expr(env, *left)?, eval_expr(env, *right)?)
        }
        Expr::If(cond, cons, alt) => eval_if_expr(env, *cond, *cons, alt),
        Expr::Ident(ident) => eval_ident(env, ident),
        Expr::Function(idents, stmts) => Ok(Object::Func(Func {
            args: idents,
            body: stmts,
            env: env.clone(),
        })),
        Expr::Call(box func_expr, args) => {
            let func_obj = eval_expr(env, func_expr)?;
            let args = args
                .into_iter()
                .map(|arg| eval_expr(env, arg))
                .collect::<EvalResult<Vec<Object>>>()?;
            let func = cast_obj_to_func(func_obj)?;
            apply_function(func, args)
        }
    }
}

fn eval_block_stmt(env: &mut Env, block: Vec<Stmt>) -> EvalResult<Object> {
    let mut result = Object::Null;

    for stmt in block {
        result = eval_stmt(env, stmt)?;

        if let Object::Return(_) = result {
            return Ok(result);
        };
    }

    Ok(result)
}

fn eval_literal(literal: Literal) -> Object {
    match literal {
        Literal::Bool(v) => Object::Bool(v),
        Literal::Int(v) => Object::Int(v),
    }
}

fn eval_prefix_expr(operator: Prefix, right: Object) -> EvalResult<Object> {
    match operator {
        Prefix::Not => Ok(eval_bang_operator_expr(right)),
        Prefix::Minus => eval_minus_operator_expr(right),
        _ => Err(EvalError(format!(
            "unknown operator: {}{}",
            operator,
            right.get_type()
        ))),
    }
}

fn eval_infix_expr(operator: Infix, left: Object, right: Object) -> EvalResult<Object> {
    if let Object::Int(left_val) = left {
        if let Object::Int(right_val) = right {
            Ok(eval_int_infix_expr(operator, left_val, right_val))
        } else {
            Err(EvalError(format!(
                "type mismatch: {} {} {}",
                left.get_type(),
                operator,
                right.get_type()
            )))
        }
    } else {
        match operator {
            Infix::Equal => Ok(Object::Bool(left == right)),
            Infix::NotEqual => Ok(Object::Bool(left != right)),
            _ => Err(EvalError(format!(
                "unknown operator: {} {} {}",
                left.get_type(),
                operator,
                right.get_type()
            ))),
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

fn eval_minus_operator_expr(right: Object) -> EvalResult<Object> {
    if let Object::Int(val) = right {
        Ok(Object::Int(-val))
    } else {
        Err(EvalError(format!(
            "unknown operator: -{}",
            right.get_type()
        )))
    }
}

fn eval_if_expr(
    env: &mut Env,
    cond: Expr,
    cons: Stmt,
    alt: Option<Box<Stmt>>,
) -> EvalResult<Object> {
    let cond_obj = eval_expr(env, cond)?;

    if is_truthy(cond_obj) {
        return eval_stmt(env, cons);
    };

    if let Some(stmt) = alt {
        return eval_stmt(env, *stmt);
    };

    Ok(Object::Null)
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Bool(true) => true,
        Object::Bool(false) => false,
        _ => true,
    }
}

fn eval_ident(env: &Env, ident: Ident) -> EvalResult<Object> {
    let val = env.get(ident.0.clone());
    if let Some(obj) = val {
        Ok(obj.clone())
    } else {
        Err(EvalError(format!(r#"identifier not found: {}"#, ident)))
    }
}

fn apply_function(func: Func, args: Vec<Object>) -> EvalResult<Object> {
    let mut wrapped_env = wrap_function_env(&func, args);
    let evaluated = eval_block_stmt(&mut wrapped_env, func.body)?;

    Ok(unwrap_return_value(evaluated))
}

fn wrap_function_env(func: &Func, args: Vec<Object>) -> Env {
    let mut env = Env::wrap(func.env.clone());

    func.args
        .iter()
        .zip(args.into_iter())
        .for_each(|(arg_name, arg_value)| {
            env.insert(arg_name.0.clone(), arg_value);
        });

    env
}

fn cast_obj_to_func(obj: Object) -> EvalResult<Func> {
    if let Object::Func(func) = obj {
        Ok(func)
    } else {
        Err(EvalError(format!("'{}' is not function object", obj)))
    }
}

fn unwrap_return_value(obj: Object) -> Object {
    if let Object::Return(box value) = obj {
        return value;
    } else {
        obj
    }
}
