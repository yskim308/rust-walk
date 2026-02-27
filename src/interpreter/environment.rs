use std::collections::HashMap;

use crate::interpreter::values::Value;

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

    pub fn get(&self, name: &String) -> Result<Value, String> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Undefined variable {}", name)),
        }
    }
}
