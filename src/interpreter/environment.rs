use std::collections::HashMap;

use crate::{error::RuntimeSignal, interpreter::values::Value, scanner::token::Token};

#[derive(Clone, Default)]
pub struct Environment {
    environment: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(environment: Environment) -> Self {
        Environment {
            environment: Some(Box::new(environment)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn into_enclosing(self) -> Option<Environment> {
        self.environment.map(|env| *env)
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeSignal> {
        if let Some(value) = self.values.get(&name.lexeme) {
            match value {
                Value::Nil => {
                    return Err(RuntimeSignal::runtime_error(
                        name.clone(),
                        format!(
                            "Attempted to evaluate unitialized variable '{}'",
                            name.lexeme
                        ),
                    ))
                }
                val => return Ok(val.clone()),
            }
        }

        match &self.environment {
            Some(env) => env.get(name),
            None => Err(RuntimeSignal::static_error(
                name.line,
                format!("Undefined Variable '{}'", name.lexeme),
            )),
        }
    }

    pub fn assign(&mut self, left: &Token, right: &Value) -> Result<(), RuntimeSignal> {
        if let Some(key) = self.values.get_mut(&left.lexeme) {
            *key = right.clone();
            Ok(())
        } else if let Some(env) = &mut self.environment {
            env.assign(left, right)
        } else {
            let err_msg = format!("invalid assignment target: {left}");
            Err(RuntimeSignal::runtime_error(left.clone(), err_msg))
        }
    }
}
