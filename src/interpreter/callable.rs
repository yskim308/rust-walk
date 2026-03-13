use std::rc::Rc;

use crate::{
    error::RuntimeSignal,
    interpreter::{
        environment::{EnvRef, Environment},
        stmt::FunctionDefinition,
        values::Value,
        Interpreter,
    },
};

#[derive(Debug)]
pub enum LoxCallable {
    Native {
        arity: usize,
        function: fn(Vec<Value>) -> Result<Value, RuntimeSignal>,
    },
    LoxFunction {
        closure: EnvRef,
        fun_def: Rc<FunctionDefinition>,
    },
}

impl LoxCallable {
    pub fn lox_function(fun_def: Rc<FunctionDefinition>, closure: EnvRef) -> Self {
        LoxCallable::LoxFunction { fun_def, closure }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, RuntimeSignal> {
        match self {
            LoxCallable::Native { arity: _, function } => function(args),
            LoxCallable::LoxFunction { fun_def, closure } => {
                let env = Environment::new_env_ref(closure.clone());
                for (i, param) in fun_def.params.iter().enumerate() {
                    env.borrow_mut()
                        .define(param.lexeme.to_string(), args[i].clone());
                }

                match interpreter.execute_block(&fun_def.body, env) {
                    Ok(()) => Ok(Value::Nil),
                    Err(RuntimeSignal::Return(value)) => Ok(value.unwrap_or(Value::Nil)),
                    Err(err) => Err(err),
                }
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            LoxCallable::Native { arity, function: _ } => *arity,
            LoxCallable::LoxFunction { fun_def, _ } => fun_def.params.len(),
        }
    }
}
