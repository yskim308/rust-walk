use std::collections::HashMap;

use crate::{
    ast::expression::Expr,
    interpreter::{stmt::Stmt, Interpreter},
    scanner::token::Token,
};

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block(stmts) => self.resolve_block(stmts),
            Stmt::Var(name, initializer) => self.resolve_var_stmt(name, initializer),
            _ => todo!(),
        }
    }

    fn resolve_expr(&self, expr: &Expr) {
        match expr {
            _ => todo!(),
        }
    }

    fn resolve_block(&mut self, stmts: &[Stmt]) {
        self.begin_scope();
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
        self.end_scope();
    }

    fn resolve_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) {
        self.declare(name);
        if let Some(expr) = initializer {
            self.resolve_expr(expr);
        }
        self.define(name);
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        todo!()
    }

    fn define(&mut self, name: &Token) {
        todo!()
    }
}
