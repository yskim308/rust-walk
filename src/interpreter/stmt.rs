use crate::{ast::expression::Expr, scanner::token::Token};

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>), // variables can be delcared unitialized
}
