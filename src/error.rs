use std::fmt::{write, Display};

#[derive(Debug, Clone)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn new(line: usize, message: String) -> Self {
        LoxError { line, message }
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error {}", self.line, self.message)
    }
}
