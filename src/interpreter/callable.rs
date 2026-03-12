use std::rc::Rc;

use crate::{
    error::LoxError,
    interpreter::{create_global_env, stmt::FunctionDefinition, values::Value, Interpreter},
};

#[derive(Debug)]
pub enum LoxCallable {
    Native {
        arity: usize,
        function: fn(Vec<Value>) -> Result<Value, LoxError>,
    },
    LoxFunction {
        fun_def: Rc<FunctionDefinition>,
    },
}

impl LoxCallable {
    pub fn lox_function(fun_def: Rc<FunctionDefinition>) -> Self {
        LoxCallable::LoxFunction { fun_def }
    }

    pub fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, LoxError> {
        match self {
            LoxCallable::Native { arity: _, function } => function(args),
            LoxCallable::LoxFunction { fun_def } => {
                let mut env = create_global_env();
                for (i, param) in fun_def.params.iter().enumerate() {
                    env.define(param.lexeme.to_string(), args[i].clone());
                }

                interpreter.execute_block(&fun_def.body, env)?;
                todo!("LoxCallable.call should return type Value")
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            LoxCallable::Native { arity, function: _ } => *arity,
            LoxCallable::LoxFunction { fun_def } => fun_def.params.len(),
        }
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, _other: &Self) -> bool {
        unreachable!("LoxCallable should NEVER be directly compared")
    }
}

impl Clone for LoxCallable {
    fn clone(&self) -> Self {
        unreachable!("LoxCallable should NEVER be cloned")
    }
}
