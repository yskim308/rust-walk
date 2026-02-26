use std::fmt::Display;

use crate::scanner::token::Token;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Static,
    Runtime,
}

#[derive(Debug, Clone)]
pub struct LoxError {
    line: usize,
    message: String,
    kind: ErrorKind,
}

impl LoxError {
    pub fn new(line: usize, message: String) -> Self {
        LoxError {
            line,
            message,
            kind: ErrorKind::Static,
        }
    }

    pub fn runtime(token: Token, message: String) -> Self {
        LoxError {
            line: token.line,
            message,
            kind: ErrorKind::Runtime,
        }
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::Static => {
                write!(f, "Static Error on [line {}]: {}", self.line, self.message)
            }
            ErrorKind::Runtime => {
                write!(f, "Runtime Error on [line {}:] {}", self.line, self.message)
            }
        }
    }
}
