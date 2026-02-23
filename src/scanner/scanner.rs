use crate::{
    error::LoxError,
    scanner::{
        token::{Literal, Token},
        token_type::TokenType,
    },
};

struct Cursor {
    start: usize,
    current: usize,
    line: usize,
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    cursor: Cursor,
    errors: Vec<LoxError>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.into_bytes(),
            tokens: Vec::new(),
            cursor: Cursor {
                start: 0,
                current: 0,
                line: 1,
            },
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.cursor.start = self.cursor.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            None,
            self.cursor.line,
        ));

        self.tokens.clone()
    }

    // to be honest probably don't need this abstraction but oh well
    fn is_at_end(&self) -> bool {
        self.cursor.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            Some(b'(') => self.add_token(TokenType::LeftParen, None),
            Some(b')') => self.add_token(TokenType::RightParen, None),
            Some(b'{') => self.add_token(TokenType::LeftBrace, None),
            Some(b'}') => self.add_token(TokenType::RightBrace, None),
            Some(b',') => self.add_token(TokenType::Comma, None),
            Some(b'.') => self.add_token(TokenType::Dot, None),
            Some(b'-') => self.add_token(TokenType::Minus, None),
            Some(b'+') => self.add_token(TokenType::Plus, None),
            Some(b';') => self.add_token(TokenType::Semicolon, None),
            Some(b'*') => self.add_token(TokenType::Star, None),
            None => self.errors.push(LoxError::new(
                self.cursor.line,
                format!("failed to get u8 at index {}", self.cursor.current),
            )),
            _ => todo!("handle unexpected tokens and error handling in scanner"),
        }
    }

    // not sure if we should be paniccing on invalid
    fn peak(&self) -> Option<u8> {
        if self.is_at_end() {
            return Some(b'\0');
        }
        self.source.get(self.cursor.current).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let c = self.peak();
        self.cursor.current += 1;

        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let byte_slice = &self.source[self.cursor.start..self.cursor.current];
        let string_slice = str::from_utf8(byte_slice).unwrap_or_else(|err| {
            panic!(
                "Invalid UTF-8 found on line {}, bytes: {:?}, with error: {}",
                self.cursor.line, byte_slice, err
            );
        });

        self.tokens.push(Token::new(
            token_type,
            string_slice.to_string(),
            literal,
            self.cursor.line,
        ));
    }
}
