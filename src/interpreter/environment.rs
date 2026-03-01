use std::collections::HashMap;

use crate::{error::LoxError, interpreter::values::Value, scanner::token::Token};

pub struct Environment {
    environment: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            environment: None,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, LoxError> {
        if let Some(env) = &self.environment {
            return env.get(name);
        }

        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(LoxError::new(
                name.line,
                format!("Undefined Variable {}", name.lexeme),
            )),
        }
    }

    pub fn assign(&mut self, left: Token, right: &Value) -> Result<(), LoxError> {
        if let Some(env) = &mut self.environment {
            return env.assign(left, right);
        }

        if let Some(key) = self.values.get_mut(&left.lexeme) {
            *key = right.clone();
            Ok(())
        } else {
            Err(LoxError::runtime(
                left,
                "invalid assignment target".to_string(),
            ))
        }
    }
}
