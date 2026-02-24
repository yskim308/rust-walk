use crate::{
    error::LoxError,
    scanner::{
        token::{Literal, Token},
        token_type::TokenType,
    },
};

// helper 'hashmap' for reserved keywords
fn get_keyword(to_find: &str) -> Option<TokenType> {
    match to_find {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "fun" => Some(TokenType::Fun),
        "for" => Some(TokenType::For),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

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
                } else if self.match_current(b'*') {
                    self.skip_bulk_comments();
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            b'\n' => self.cursor.line += 1,

            // =============== LITERALS AND IDENTIFIERS ==================
            b'"' => self.handle_string(),
            c => {
                if c.is_ascii_digit() {
                    self.handle_number();
                } else if c.is_ascii_alphanumeric() {
                    self.handle_identifier();
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

    fn skip_bulk_comments(&mut self) {
        // looking for */ pattern
        let original_line = self.cursor.line;
        while (self.peek() != Some(b'*') || self.peek_next() != Some(b'/')) && !self.is_at_end() {
            if self.peek() == Some(b'\n') {
                self.cursor.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(LoxError::new(
                original_line,
                "unterminated bulk comment".to_string(),
            ));
            return;
        }

        // skip */
        self.advance();
        self.advance();
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let Some(str) = self.get_str_from_current_idx() else {
            return
        };

        let lexeme_str = str.to_string();
        self.tokens.push(Token::new(
            token_type,
            lexeme_str,
            literal,
            self.cursor.line,
        ));
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

        let Some(owned_string) = self.get_str_from_current_idx().map(|s| s.to_string()) else {return};

        self.add_token(TokenType::String, Some(Literal::String(owned_string)));
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

        let Some(str) = self.get_str_from_current_idx() else {return};

        let number_literal: f64 = str.parse().unwrap(); // unwrap assuming we checked all digits

        self.add_token(TokenType::Number, Some(Literal::Number(number_literal)));
    }

    fn handle_identifier(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric()) {
            self.advance();
        }

        let Some(owned_str) = self.get_str_from_current_idx().map(|s| s.to_string()) else {
            return
        };

        match get_keyword(&owned_str) {
            Some(token) => self.add_token(token, None),
            None => self.add_token(TokenType::Identifier, None),
        }
    }

    fn get_str_from_current_idx(&mut self) -> Option<&str> {
        let bytes = &self.source[self.cursor.start..self.cursor.current];

        let str = match str::from_utf8(bytes) {
            Ok(str) => str,
            Err(err) => {
                let msg = format!(
                    "Invalid UTF-8 found on line {}, bytes: {:?}, with error: {}",
                    self.cursor.line, bytes, err
                );
                self.errors.push(LoxError::new(self.cursor.line, msg));
                return None;
            }
        };

        Some(str)
    }
}
