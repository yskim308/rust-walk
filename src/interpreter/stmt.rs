use std::rc::Rc;

use crate::{ast::expression::Expr, scanner::token::Token};

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Function(Rc<FunctionDefinition>),
    If(IfConditions),
    Print(Expr),
    Var(Token, Option<Expr>), // variables can be delcared unitialized
    While(WhileConditions),
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
struct WhileConditions {
    pub condition: Expr,
    pub stmt_body: Box<Stmt>,
}

#[derive(Debug)]
struct IfConditions {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

impl Stmt {
    pub fn function(name: Token, params: Vec<Token>, body: Stmt) -> Self {
        let fun_body = if let Stmt::Block(block) = body {
            block
        } else {
            panic!("body for func {} is not a block statement!", name)
        };

        Stmt::Function(Rc::new(FunctionDefinition {
            name,
            params,
            body: fun_body,
        }))
    }

    pub fn while_statement(condition: Expr, stmt_body: Stmt) -> Self {
        Stmt::While(WhileConditions {
            condition,
            stmt_body: Box::new(stmt_body),
        })
    }

    pub fn if_statement(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Stmt::If(IfConditions {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }
}
