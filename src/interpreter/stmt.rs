use crate::{ast::expression::Expr, scanner::token::Token};

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>), // variables can be delcared unitialized
}
