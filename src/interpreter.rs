use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    ast::expression::{Expr, LiteralValue},
    error::RuntimeSignal,
    interpreter::{
        callable::LoxCallable,
        environment::{EnvRef, Environment},
        stmt::Stmt,
        values::Value,
    },
    scanner::{token::Token, token_type::TokenType},
};

mod callable;
mod environment;
pub mod stmt;
pub mod values;

// native function(s)
fn clock(_args: Vec<Value>) -> Result<Value, RuntimeSignal> {
    let now = SystemTime::now();
    let since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards?");

    Ok(Value::Number(since_epoch.as_secs_f64()))
}

pub struct Interpreter {
    pub globals: EnvRef,
    environment: EnvRef,
}

pub fn create_global_env() -> EnvRef {
    let global = Environment::new_env_ref(None);

    let clock_value = Value::Callable(Rc::new(LoxCallable::Native {
        arity: 0,
        function: clock,
    }));
    global.borrow_mut().define("clock".into(), clock_value);

    global
}

impl Interpreter {
    pub fn new() -> Self {
        let global = create_global_env();
        Interpreter {
            globals: global.clone(),
            environment: global,
        }
    }

    pub fn resolve(&self, expr: &Expr, depth: usize) {
        todo!()
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            if let Err(e) = self.evaluate_statement(stmt) {
                match e {
                    RuntimeSignal::Error(_) => eprintln!("{e}"),
                    RuntimeSignal::Return(_) => {
                        eprintln!("Should not be returning from top level")
                    }
                }
                return;
            }
        }
    }

    fn evaluate_statement(&mut self, stmt: &Stmt) -> Result<(), RuntimeSignal> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("{}", value.as_string());
                Ok(())
            }
            Stmt::Var(token, initializer) => match initializer {
                Some(expr) => {
                    let value = self.evaluate_expression(expr)?;
                    self.environment
                        .borrow_mut()
                        .define(token.lexeme.clone(), value);
                    Ok(())
                }
                None => {
                    self.environment
                        .borrow_mut()
                        .define(token.lexeme.clone(), Value::Nil);
                    Ok(())
                }
            },
            Stmt::Block(statements) => self.execute_block(statements, self.environment.clone()),
            Stmt::If(conditions) => {
                if self.evaluate_expression(&conditions.condition)?.is_truthy() {
                    self.evaluate_statement(&conditions.then_branch)?;
                } else if let Some(else_branch) = &conditions.else_branch {
                    self.evaluate_statement(else_branch)?;
                }
                Ok(())
            }
            Stmt::While(conditions) => {
                while self.evaluate_expression(&conditions.condition)?.is_truthy() {
                    self.evaluate_statement(&conditions.stmt_body)?;
                }
                Ok(())
            }
            Stmt::Function(fun_def) => {
                let function = Value::Callable(Rc::new(LoxCallable::lox_function(
                    fun_def.clone(),
                    self.environment.clone(),
                )));
                self.environment
                    .borrow_mut()
                    .define(fun_def.name.lexeme.clone(), function);
                Ok(())
            }
            Stmt::Return(_, expr) => {
                let value = if let Some(expr) = expr {
                    Some(self.evaluate_expression(expr)?)
                } else {
                    None
                };
                Err(RuntimeSignal::Return(value))
            }
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        enclosing: EnvRef,
    ) -> Result<(), RuntimeSignal> {
        let previous = self.environment.clone();
        self.environment = Environment::new_env_ref(enclosing.clone());

        let mut result = Ok(());
        for stmt in statements {
            if let Err(e) = self.evaluate_statement(stmt) {
                result = Err(e);
                break;
            }
        }

        self.environment = previous;
        result
    }

    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value, RuntimeSignal> {
        match expr {
            Expr::Assignment { name, value } => {
                let right_value = self.evaluate_expression(value)?;
                self.environment.borrow_mut().assign(name, &right_value)?;
                Ok(right_value)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => self.evaluate_logical(left, operator, right),
            Expr::Binary {
                left_expr,
                operator,
                right_expr,
            } => self.evaluate_binary(left_expr, operator, right_expr),
            Expr::Unary { token, expression } => self.evaluate_unary(token, expression),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => self.evaluate_call(callee, paren, arguments),
            Expr::Grouping { expression } => self.evaluate_expression(expression),
            Expr::Literal { value } => Ok(self.literal_to_value(value)),
            Expr::Variable { token } => self.environment.borrow().get(token),
        }
    }

    fn evaluate_call(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &[Expr],
    ) -> Result<Value, RuntimeSignal> {
        let callee_value = self.evaluate_expression(callee)?;

        let mut argument_values = Vec::new();

        for expr_arguments in arguments {
            argument_values.push(self.evaluate_expression(expr_arguments)?);
        }

        if let Value::Callable(lox_callable) = callee_value {
            if argument_values.len() != lox_callable.arity() {
                return Err(RuntimeSignal::runtime_error(
                    paren.clone(),
                    format!(
                        "Expected {} arguments, but got {}",
                        lox_callable.arity(),
                        argument_values.len()
                    ),
                ));
            }
            lox_callable.call(self, argument_values)
        } else {
            Err(RuntimeSignal::runtime_error(
                paren.clone(),
                format!("Expr not callable: {callee}"),
            ))
        }
    }

    fn evaluate_logical(
        &mut self,
        left_expr: &Expr,
        operator: &Token,
        right_expr: &Expr,
    ) -> Result<Value, RuntimeSignal> {
        let left = self.evaluate_expression(left_expr)?;

        if operator.token_type == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else if !left.is_truthy() {
            return Ok(left);
        }

        self.evaluate_expression(right_expr)
    }

    fn literal_to_value(&self, literal: &LiteralValue) -> Value {
        match literal {
            LiteralValue::Nil => Value::Nil,
            LiteralValue::Number(n) => Value::Number(*n),
            LiteralValue::Boolean(b) => Value::Boolean(*b),
            LiteralValue::String(s) => Value::String(Rc::new(s.clone())),
        }
    }

    fn evaluate_unary(
        &mut self,
        operator: &Token,
        expression: &Expr,
    ) -> Result<Value, RuntimeSignal> {
        let right_val = self.evaluate_expression(expression)?;

        match operator.token_type {
            TokenType::Minus => {
                if right_val.is_numeric() {
                    Ok(Value::Number(-right_val.as_number()))
                } else {
                    Err(RuntimeSignal::runtime_error(
                        operator.clone(),
                        "{-} operation attempted on non numeric type".to_string(),
                    ))
                }
            }
            TokenType::Bang => Ok(Value::Boolean(!right_val.is_truthy())),
            _ => panic!(
                "evalute unary called when operator is neither Minus or Bang, 
                \nOperator: {operator}",
            ),
        }
    }

    fn evaluate_binary(
        &mut self,
        left_expr: &Expr,
        operator: &Token,
        right_expr: &Expr,
    ) -> Result<Value, RuntimeSignal> {
        let left_val = self.evaluate_expression(left_expr)?;
        let right_val = self.evaluate_expression(right_expr)?;

        match operator.token_type {
            // ============ numeric comparison =============
            TokenType::Greater => {
                self.both_are_numeric(&left_val, operator, &right_val)?;
                Ok(Value::Boolean(left_val.as_number() > right_val.as_number()))
            }
            TokenType::GreaterEqual => {
                self.both_are_numeric(&left_val, operator, &right_val)?;
                Ok(Value::Boolean(
                    left_val.as_number() >= right_val.as_number(),
                ))
            }
            TokenType::Less => {
                self.both_are_numeric(&left_val, operator, &right_val)?;
                Ok(Value::Boolean(left_val.as_number() < right_val.as_number()))
            }
            TokenType::LessEqual => {
                self.both_are_numeric(&left_val, operator, &right_val)?;
                Ok(Value::Boolean(
                    left_val.as_number() <= right_val.as_number(),
                ))
            }

            // ============ equality ================
            TokenType::EqualEqual => Ok(Value::Boolean(left_val == right_val)),
            TokenType::BangEqual => Ok(Value::Boolean(left_val != right_val)),

            // ============ arithmetic ============
            TokenType::Minus => {
                self.both_are_numeric(&left_val, operator, &right_val)?;
                Ok(Value::Number(left_val.as_number() - right_val.as_number()))
            }
            TokenType::Slash => {
                self.both_are_numeric(&left_val, operator, &right_val)?;
                Ok(Value::Number(left_val.as_number() / right_val.as_number()))
            }
            TokenType::Star => {
                self.both_are_numeric(&left_val, operator, &right_val)?;
                Ok(Value::Number(left_val.as_number() * right_val.as_number()))
            }

            // =========== arithmeitc and string concact ============
            TokenType::Plus => {
                if let Ok(true) = self.both_are_numeric(&left_val, operator, &right_val) {
                    Ok(Value::Number(left_val.as_number() + right_val.as_number()))
                } else {
                    let s = self.concatenate_strings(left_val, operator, right_val)?;
                    Ok(Value::String(Rc::new(s)))
                }
            }
            _ => panic!("Evaluate Unary called on invalid operator: {operator}"),
        }
    }

    fn both_are_numeric(
        &self,
        left_val: &Value,
        operator: &Token,
        right_val: &Value,
    ) -> Result<bool, RuntimeSignal> {
        if left_val.is_numeric() && right_val.is_numeric() {
            Ok(true)
        } else {
            let token_type = operator.token_type.clone();
            Err(RuntimeSignal::runtime_error(
                operator.clone(),
                format!("'{}' operation attempted on non numeric types", token_type),
            ))
        }
    }

    fn concatenate_strings(
        &self,
        left_val: Value,
        operator: &Token,
        right_val: Value,
    ) -> Result<String, RuntimeSignal> {
        // if either is a string, concat
        if left_val.is_stringy() || right_val.is_stringy() {
            Ok(left_val.as_string() + &right_val.as_string())
        } else {
            Err(RuntimeSignal::runtime_error(
                operator.clone(),
                format!(
                    "{} operation attempted on binary in which neither are of type String or Number",
                    operator.token_type.clone())
                )
            )
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
