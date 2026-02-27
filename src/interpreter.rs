use crate::{
    ast::expression::{Expr, LiteralValue},
    error::LoxError,
    interpreter::{stmt::Stmt, values::Value},
    scanner::{token::Token, token_type::TokenType},
};

pub mod stmt;
pub mod values;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&self, statements: Vec<Stmt>) {
        for stmt in statements {
            if let Err(e) = self.evaluate_statement(stmt) {
                eprint!("{e}");
            }
        }
    }

    fn evaluate_statement(&self, stmt: Stmt) -> Result<(), LoxError> {
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
            Stmt::Var(_, _) => todo!(),
        }
    }

    fn evaluate_expression(&self, expr: Expr) -> Result<Value, LoxError> {
        match expr {
            Expr::Literal { value } => Ok(self.literal_to_value(value)),
            Expr::Grouping { expression } => self.evaluate_expression(*expression),
            Expr::Unary { token, expression } => self.evaluate_unary(token, *expression),
            Expr::Binary {
                left_expr,
                operator,
                right_expr,
            } => self.evaluate_binary(*left_expr, operator, *right_expr),
            Expr::Variable { token } => todo!(),
        }
    }

    fn literal_to_value(&self, literal: LiteralValue) -> Value {
        match literal {
            LiteralValue::Nil => Value::Nil,
            LiteralValue::Number(n) => Value::Number(n),
            LiteralValue::Boolean(b) => Value::Boolean(b),
            LiteralValue::String(s) => Value::String(s),
        }
    }

    fn evaluate_unary(&self, operator: Token, expression: Expr) -> Result<Value, LoxError> {
        let right_val = self.evaluate_expression(expression)?;

        match operator.token_type {
            TokenType::Minus => {
                if right_val.is_numeric() {
                    Ok(Value::Number(-right_val.as_number()))
                } else {
                    Err(LoxError::runtime(
                        operator,
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
        &self,
        left_expr: Expr,
        operator: Token,
        right_expr: Expr,
    ) -> Result<Value, LoxError> {
        let left_val = self.evaluate_expression(left_expr)?;
        let right_val = self.evaluate_expression(right_expr)?;

        match operator.token_type {
            // ============ numeric comparison =============
            TokenType::Greater => {
                self.both_are_numeric(&left_val, &operator, &right_val)?;
                Ok(Value::Boolean(left_val.as_number() > right_val.as_number()))
            }
            TokenType::GreaterEqual => {
                self.both_are_numeric(&left_val, &operator, &right_val)?;
                Ok(Value::Boolean(
                    left_val.as_number() >= right_val.as_number(),
                ))
            }
            TokenType::Less => {
                self.both_are_numeric(&left_val, &operator, &right_val)?;
                Ok(Value::Boolean(left_val.as_number() < right_val.as_number()))
            }
            TokenType::LessEqual => {
                self.both_are_numeric(&left_val, &operator, &right_val)?;
                Ok(Value::Boolean(
                    left_val.as_number() <= right_val.as_number(),
                ))
            }

            // ============ equality ================
            TokenType::EqualEqual => Ok(Value::Boolean(left_val == right_val)),
            TokenType::BangEqual => Ok(Value::Boolean(left_val != right_val)),

            // ============ arithmetic ============
            TokenType::Minus => {
                self.both_are_numeric(&left_val, &operator, &right_val)?;
                Ok(Value::Number(left_val.as_number() - right_val.as_number()))
            }
            TokenType::Slash => {
                self.both_are_numeric(&left_val, &operator, &right_val)?;
                Ok(Value::Number(left_val.as_number() / right_val.as_number()))
            }
            TokenType::Star => {
                self.both_are_numeric(&left_val, &operator, &right_val)?;
                Ok(Value::Number(left_val.as_number() * right_val.as_number()))
            }

            // =========== arithmeitc and string concact ============
            TokenType::Plus => {
                if let Ok(true) = self.both_are_numeric(&left_val, &operator, &right_val) {
                    Ok(Value::Number(left_val.as_number() + right_val.as_number()))
                } else {
                    let s = self.concatenate_strings(left_val, &operator, right_val)?;
                    Ok(Value::String(s))
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
    ) -> Result<bool, LoxError> {
        if left_val.is_numeric() && right_val.is_numeric() {
            Ok(true)
        } else {
            let token_type = operator.token_type.clone();
            Err(LoxError::runtime(
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
    ) -> Result<String, LoxError> {
        // if either is a string, concat
        if left_val.is_stringy() || right_val.is_stringy() {
            Ok(left_val.as_string() + &right_val.as_string())
        } else {
            Err(LoxError::runtime(
                operator.clone(),
                format!(
                    "{} operation attempted on binary in which neither are of type String or Number",
                    operator.token_type.clone())
                )
            )
        }
    }
}
