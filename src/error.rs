use std::fmt::Display;

use crate::{interpreter::values::Value, scanner::token::Token};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Static,
    Runtime,
}

#[derive(Debug, Clone)]
pub enum RuntimeSignal {
    Error(LoxError),
    Return(Option<Value>),
}

#[derive(Debug, Clone)]
pub struct LoxError {
    line: usize,
    message: String,
    kind: ErrorKind,
}

impl RuntimeSignal {
    pub fn static_error(line: usize, message: String) -> Self {
        RuntimeSignal::Error(LoxError {
            line,
            message,
            kind: ErrorKind::Static,
        })
    }

    pub fn runtime_error(token: Token, message: String) -> Self {
        RuntimeSignal::Error(LoxError {
            line: token.line,
            message,
            kind: ErrorKind::Runtime,
        })
    }
}

impl Display for RuntimeSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeSignal::Error(err) => match err.kind {
                ErrorKind::Static => {
                    write!(f, "Static Error on [line {}]: {}", err.line, err.message)
                }
                ErrorKind::Runtime => {
                    write!(f, "Runtime Error on [line {}:] {}", err.line, err.message)
                }
            },
            RuntimeSignal::Return(val) => write!(f, "Return value: {:#?}", val),
        }
    }
}
