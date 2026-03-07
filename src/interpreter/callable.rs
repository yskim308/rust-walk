use crate::{error::LoxError, interpreter::values::Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoxCallable {}

impl LoxCallable {
    pub fn call(&self) -> Result<Value, LoxError> {
        todo!()
    }

    pub fn arity(&self) -> usize {
        todo!()
    }
}
