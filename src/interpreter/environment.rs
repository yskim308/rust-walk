use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeSignal, interpreter::values::Value, scanner::token::Token};

pub type EnvRef = Rc<RefCell<Environment>>;

#[derive(Clone, Default)]
pub struct Environment {
    enclosing: Option<EnvRef>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: EnvRef) -> Self {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn new_env_ref(enclosing: impl Into<Option<EnvRef>>) -> EnvRef {
        if let Some(env_ref) = enclosing.into() {
            Rc::new(RefCell::new(Self::new(env_ref)))
        } else {
            Rc::new(RefCell::new(Self::default()))
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
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

        match &self.enclosing {
            Some(env) => env.borrow().get(name),
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
        } else if let Some(env) = &mut self.enclosing {
            env.borrow_mut().assign(left, right)
        } else {
            let err_msg = format!("invalid assignment target: {left}");
            Err(RuntimeSignal::runtime_error(left.clone(), err_msg))
        }
    }
}
