use crate::{
    ast::expression::Expr,
    scanner::{
        token::{self, Token},
        token_type::TokenType,
    },
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.check_current_type(TokenType::BangEqual)
            || self.check_current_type(TokenType::EqualEqual)
        {
            let operator = self.advance();
            let right = self.comparison();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn comparison(&self) -> Expr {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn check_current_type(&self, token_type: TokenType) -> bool {
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
