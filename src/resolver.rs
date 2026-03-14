use std::collections::HashMap;

use crate::{
    ast::expression::Expr,
    error::{LoxError, RuntimeSignal},
    interpreter::{
        stmt::{FunctionDefinition, Stmt},
        Interpreter,
    },
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

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeSignal> {
        match stmt {
            Stmt::Block(stmts) => self.resolve_block(stmts),
            Stmt::Var(name, initializer) => self.resolve_var_stmt(name, initializer),
            Stmt::Function(fun_def) => {
                self.declare(&fun_def.name);
                self.define(&fun_def.name);
                self.resolve_function_stmt(fun_def)
            }
            _ => todo!(),
        }
    }

    fn resolve_function_stmt(&mut self, fun_def: &FunctionDefinition) -> Result<(), RuntimeSignal> {
        self.begin_scope();
        for param in &fun_def.params {
            self.declare(param);
            self.define(param);
        }

        self.resolve_block(&fun_def.body)?;
        self.end_scope();
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), RuntimeSignal> {
        match expr {
            Expr::Variable { token } => self.resolve_var_expr(token, expr),
            Expr::Assignment { name, value } => self.resolve_assign_expr(name, value, expr),
            _ => todo!(),
        }
    }

    fn resolve_assign_expr(
        &mut self,
        name: &Token,
        value: &Expr,
        expr: &Expr,
    ) -> Result<(), RuntimeSignal> {
        self.resolve_expr(value)?;
        self.resolve_var_local(expr, name)?;
        Ok(())
    }

    fn resolve_var_expr(&mut self, name: &Token, expr: &Expr) -> Result<(), RuntimeSignal> {
        if let Some(top_scope) = self.scopes.last_mut() && let Some(_) = top_scope.get(&name.lexeme) {
            return Err(RuntimeSignal::static_error(name.line, "Can't read local variable in own initializer".into()));
        } else {
            self.resolve_var_local(expr, name)?;
        }
        Ok(())
    }

    fn resolve_var_local(&mut self, expr: &Expr, name: &Token) -> Result<(), RuntimeSignal> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, i);
            }
        }
        Ok(())
    }

    fn resolve_block(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeSignal> {
        self.begin_scope();
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        self.end_scope();
        Ok(())
    }

    fn resolve_var_stmt(
        &mut self,
        name: &Token,
        initializer: &Option<Expr>,
    ) -> Result<(), RuntimeSignal> {
        self.declare(name);
        if let Some(expr) = initializer {
            self.resolve_expr(expr)?;
        }
        self.define(name);
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if let Some(top_scope) = self.scopes.last_mut() {
            top_scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(top_scope) = self.scopes.last_mut() {
            top_scope.insert(name.lexeme.clone(), true);
        }
    }
}
