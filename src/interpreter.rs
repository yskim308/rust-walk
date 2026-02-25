use crate::{
    ast::expression::{Expr, LiteralValue},
    interpreter::values::Value,
};

pub mod values;

pub struct Interpreter {}

impl Interpreter {
    fn evaluated_literal(expr: Expr) -> Value {
        let Expr::Literal { value } = expr else {
            panic!("evaluate literal was called on non literal type: {:?}", expr)
        };

        match value {
            LiteralValue::Nil => Value::Nil,
            LiteralValue::Number(n) => Value::Number(n),
            LiteralValue::Boolean(b) => Value::Boolean(b),
            LiteralValue::String(s) => Value::String(s),
        }
    }
}
