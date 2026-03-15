use std::collections::HashMap;

use crate::{
    ast::expression::Expr,
    error::RuntimeSignal,
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
            Stmt::Expression(expr) => self.resolve_expr(expr),
            Stmt::If(if_conditions) => {
                self.resolve_expr(&if_conditions.condition)?;
                self.resolve_stmt(&if_conditions.then_branch)?;
                if let Some(else_branch) = &if_conditions.else_branch {
                    self.resolve_stmt(else_branch)?
                }
                Ok(())
            }
            Stmt::Print(expr) => self.resolve_expr(expr),
            Stmt::Return(_, value) => {
                if let Some(expr) = value {
                    self.resolve_expr(expr)?
                }
                Ok(())
            }
            Stmt::While(while_conditions) => {
                self.resolve_expr(&while_conditions.condition)?;
                self.resolve_stmt(&while_conditions.stmt_body)
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
            Expr::Variable { token, .. } => self.resolve_var_expr(token, expr),
            Expr::Assignment { name, value, .. } => self.resolve_assign_expr(name, value, expr),
            Expr::Binary {
                id: _,
                left_expr,
                operator: _,
                right_expr,
            } => {
                self.resolve_expr(left_expr)?;
                self.resolve_expr(right_expr)
            }
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(callee)?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }
                Ok(())
            }
            Expr::Grouping { expression, .. } => self.resolve_expr(expression),
            Expr::Literal { .. } => Ok(()),
            Expr::Logical {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            Expr::Unary {
                token: _,
                expression,
                id: _,
            } => self.resolve_expr(expression),
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
