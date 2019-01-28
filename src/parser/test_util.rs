use super::*;

pub trait Literable: Sized {
    fn check_expr(self, expr: &Expr);

    fn check_stmt(self, stmt: &Stmt) {
        if let Stmt::Expr(expr) = stmt {
            self.check_expr(expr)
        } else {
            panic!(format!("this stmt is not include expr. got {:?}", stmt));
        }
    }
}

impl Literable for i64 {
    fn check_expr(self, expr: &Expr) {
        assert_eq!(&Expr::Literal(Literal::Int(self)), expr, "got {}", expr);
    }
}

impl Literable for &str {
    fn check_expr(self, expr: &Expr) {
        assert_eq!(&Expr::Ident(Ident(self.to_string())), expr, "got {}", expr);
    }
}

impl Literable for bool {
    fn check_expr(self, expr: &Expr) {
        assert_eq!(&Expr::Literal(Literal::Bool(self)), expr, "got {}", expr);
    }
}

pub fn check_expr<T: Literable>(expr: &Expr, val: T) {
    val.check_expr(expr);
}

pub fn check_stmt<T: Literable>(stmt: &Stmt, val: T) {
    val.check_stmt(stmt)
}

pub fn check_infix_expr<T: Literable>(expr: &Expr, expected_left: T, expected_infix: Infix, expected_right: T) {
    if let Expr::Infix(box left, infix, box right) = expr {
        expected_left.check_expr(left);
        assert_eq!(expected_infix, *infix);
        expected_right.check_expr(right);
    } else {
        panic!(format!("this expr is not 'infix'. got {}", expr));
    }
}

pub fn check_infix_stmt<T: Literable>(stmt: &Stmt, expect_left: T, expect_infix: Infix, expect_right: T) {
    if let Stmt::Expr(expr) = stmt {
        check_infix_expr(expr, expect_left, expect_infix, expect_right)
    } else {
        panic!(format!("this stmt is not expr. got {}", stmt));
    }
}

pub fn check_let_stmt(stmt: &Stmt, expected_name: &str) {
    if let Stmt::Let(Ident(name), _) = stmt {
        assert_eq!(expected_name, name)
    } else {
        panic!(format!("this stmt is not 'let'. got {}", stmt));
    }
}