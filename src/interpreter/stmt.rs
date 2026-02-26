use crate::ast::expression::Expr;

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}
