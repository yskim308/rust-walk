use crate::{
    ast::expression::{Expr, LiteralValue},
    error::LoxError,
    interpreter::values::Value,
    scanner::{token::Token, token_type::TokenType},
};

pub mod values;

pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&self, expr: Expr) -> Result<Value, LoxError> {
        match expr {
            Expr::Literal { value } => Ok(self.literal_to_value(value)),
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Unary { token, expression } => self.evaluate_unary(token, *expression),
            _ => todo!(),
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
        let right_val = self.evaluate(expression)?;

        match operator.token_type {
            TokenType::Minus => {
                if right_val.is_numeric() {
                    Ok(Value::Number(right_val.as_number()))
                } else {
                    todo!("handle if cast fails")
                }
            }
            TokenType::Bang => Ok(Value::Boolean(!right_val.is_truthy())),
            _ => todo!("handle if operator not minus/bang"),
        }
    }
}
