use crate::{error::LoxError, interpreter::values::Value};

#[derive(Debug, Clone, Copy)]
pub enum LoxCallable {
    Native {
        arity: usize,
        function: fn(Vec<Value>) -> Result<Value, LoxError>,
    },
}

impl LoxCallable {
    pub fn call(&self) -> Result<Value, LoxError> {
        todo!()
    }

    pub fn arity(&self) -> usize {
        todo!()
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                LoxCallable::Native {
                    arity: arity_a,
                    function: func_a,
                },
                LoxCallable::Native {
                    arity: arity_b,
                    function: func_b,
                },
            ) => {
                // Casting to usize bypasses the compiler warning entirely!
                arity_a == arity_b && std::ptr::fn_addr_eq(*func_a, *func_b)
            }
        }
    }
}
