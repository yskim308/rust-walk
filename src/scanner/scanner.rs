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

    pub fn scan_tokens(&mut self) -> (Vec<Token>, Vec<LoxError>) {
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

        (self.tokens.clone(), self.errors.clone())
    }

    // to be honest probably don't need this abstraction but oh well
    fn is_at_end(&self) -> bool {
        self.cursor.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = match self.advance() {
            Some(c) => c,
            None => {
                self.errors.push(LoxError::new(
                    self.cursor.line,
                    format!("failed to get u8 at index {}", self.cursor.current),
                ));
                return;
            }
        };

        match c {
            // ================== OPERATORS =====================
            b'(' => self.add_token(TokenType::LeftParen, None),
            b')' => self.add_token(TokenType::RightParen, None),
            b'{' => self.add_token(TokenType::LeftBrace, None),
            b'}' => self.add_token(TokenType::RightBrace, None),
            b',' => self.add_token(TokenType::Comma, None),
            b'.' => self.add_token(TokenType::Dot, None),
            b'-' => self.add_token(TokenType::Minus, None),
            b'+' => self.add_token(TokenType::Plus, None),
            b';' => self.add_token(TokenType::Semicolon, None),
            b'*' => self.add_token(TokenType::Star, None),
            b'!' => self.add_conditional_token(b'=', TokenType::BangEqual, TokenType::Bang),
            b'=' => self.add_conditional_token(b'=', TokenType::EqualEqual, TokenType::Equal),
            b'<' => self.add_conditional_token(b'=', TokenType::LessEqual, TokenType::Less),
            b'>' => self.add_conditional_token(b'=', TokenType::GreaterEqual, TokenType::Greater),
            b'/' => {
                if self.match_current(b'/') {
                    while self.peek() != Some(b'\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            b'\n' => self.cursor.line += 1,

            // =============== LITERALS ==================
            b'"' => self.handle_string(),
            c => {
                if c.is_ascii_digit() {
                    self.handle_number();
                } else {
                    self.errors.push(LoxError::new(
                        self.cursor.line,
                        format!("unexpected token: {}", c),
                    ));
                }
            }
        }
    }

    // not sure if we should be paniccing on invalid
    fn peek(&self) -> Option<u8> {
        if self.is_at_end() {
            return None;
        }
        self.source.get(self.cursor.current).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        if self.cursor.current + 1 >= self.source.len() {
            return Some(b'\0');
        }

        self.source.get(self.cursor.current + 1).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let c = self.peek();
        if c.is_some() {
            self.cursor.current += 1;
        }

        c
    }

    fn add_token(&mut self, token_type: TokenType, mut literal: Option<Literal>) {
        let byte_slice = &self.source[self.cursor.start..self.cursor.current];

        match str::from_utf8(byte_slice) {
            Ok(string_slice) => {
                if let Some(Literal::String(_)) = literal {
                    literal = Some(Literal::String(string_slice.to_string()));
                }
                self.tokens.push(Token::new(
                    token_type,
                    string_slice.to_string(),
                    literal,
                    self.cursor.line,
                ));
            }
            Err(err) => {
                let msg = format!(
                    "Invalid UTF-8 found on line {}, bytes: {:?}, with error: {}",
                    self.cursor.line, byte_slice, err
                );
                self.errors.push(LoxError::new(self.cursor.line, msg));
            }
        }
    }

    fn add_conditional_token(&mut self, expected: u8, matched: TokenType, unmatched: TokenType) {
        let token_type = if self.match_current(expected) {
            matched
        } else {
            unmatched
        };
        self.add_token(token_type, None);
    }

    fn match_current(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        match self.peek() {
            Some(char) if expected == char => {
                self.cursor.current += 1;
                true
            }
            _ => false,
        }
    }

    fn handle_string(&mut self) {
        let line_before = self.cursor.line;
        while self.peek() != Some(b'"') && !self.is_at_end() {
            if self.peek() == Some(b'\n') {
                self.cursor.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(LoxError::new(
                self.cursor.line,
                "unterminated string".into(),
            ));

            self.cursor.current = self.cursor.start + 1;
            self.cursor.line = line_before;
        }

        self.advance();

        // we'll let the add token handle the string slicing
        self.add_token(TokenType::String, Some(Literal::String(String::new())));
    }

    fn handle_number(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek().is_some_and(|c| c == b'.')
            && self.peek_next().is_some_and(|c| c.is_ascii_digit())
        {
            self.advance();

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        let bytes = &self.source[self.cursor.start..self.cursor.current];

        let str = match str::from_utf8(bytes) {
            Ok(str) => str,
            Err(err) => {
                let msg = format!(
                    "Invalid UTF-8 found on line {}, bytes: {:?}, with error: {}",
                    self.cursor.line, bytes, err
                );
                self.errors.push(LoxError::new(self.cursor.line, msg));
                return;
            }
        };

        let number_literal: f64 = str.parse().unwrap(); // unwrap assuming we checked all digits

        self.add_token(TokenType::Number, Some(Literal::Number(number_literal)));
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::scanner::token_type::TokenType;

    #[test]
    fn operators_scanned_properly() {
        let mut scanner = Scanner::new("!==<=>=".to_string());
        let tokens = scanner.scan_tokens();

        assert!(matches!(tokens[0].token_type, TokenType::BangEqual));
        assert_eq!(tokens[0].lexeme, "!=");

        assert!(matches!(tokens[1].token_type, TokenType::Equal));
        assert_eq!(tokens[1].lexeme, "=");

        assert!(matches!(tokens[2].token_type, TokenType::LessEqual));
        assert_eq!(tokens[2].lexeme, "<=");

        assert!(matches!(tokens[3].token_type, TokenType::GreaterEqual));
        assert_eq!(tokens[3].lexeme, ">=");

        assert!(matches!(tokens[4].token_type, TokenType::EOF));
    }
}
