use std::collections::HashMap;

use crate::{error::LoxError, interpreter::values::Value, scanner::token::Token};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(LoxError::new(
                name.line,
                format!("Undefined Variable {}", name.lexeme),
            )),
        }
    }
}
