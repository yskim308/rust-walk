use crate::{ast::expression::Expr, scanner::token::Token};

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    If(IfConditions),
    Print(Expr),
    Var(Token, Option<Expr>), // variables can be delcared unitialized
}

#[derive(Debug)]
pub struct IfConditions {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Box<Stmt>,
}

impl IfConditions {
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Stmt) -> Self {
        IfConditions {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }
}
