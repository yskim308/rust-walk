use core::fmt;

use crate::scanner::token_type::TokenType;

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u64,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: u64) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
