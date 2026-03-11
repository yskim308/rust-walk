use crate::{
    error::LoxError,
    interpreter::{
        create_global_env, environment::Environment, stmt::Stmt, values::Value, Interpreter,
    },
};

#[derive(Debug)]
pub enum LoxCallable {
    Native {
        arity: usize,
        function: fn(Vec<Value>) -> Result<Value, LoxError>,
    },
    LoxFunction {
        declaration: Stmt,
    },
}

impl LoxCallable {
    pub fn call(&self, interpreter: Interpreter, args: Vec<Value>) -> Result<Value, LoxError> {
        match self {
            LoxCallable::Native { arity, function } => function(args),
            LoxCallable::LoxFunction { declaration } => {
                let fun_def = if let Stmt::Function(def) = declaration {
                    def.to_owned()
                } else {
                    panic!("funciton declaration is not of type function statement")
                };

                let mut env = create_global_env();
                for (i, param) in fun_def.params.iter().enumerate() {
                    env.define(param.lexeme, args[i]);
                }
                todo!()
            }
        }
    }

    pub fn arity(&self) -> usize {
        todo!()
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
