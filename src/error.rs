#[derive(Debug, Clone)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn new(line: usize, message: String) -> Self {
        LoxError { line, message }
    }

    pub fn report(&self) {
        eprint!("[line {}] Error {}", self.line, self.message)
    }
}
