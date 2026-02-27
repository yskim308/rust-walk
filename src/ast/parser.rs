use crate::{
    ast::expression::{Expr, LiteralValue},
    error::LoxError,
    interpreter::stmt::Stmt,
    scanner::{
        token::{Literal, Token},
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        match self.peek().token_type {
            TokenType::Print => {
                self.advance();
                self.print_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value".to_string())?;
        Ok(Stmt::Print(expression))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value".to_string())?;
        Ok(Stmt::Expression(expression))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.check_current_type(TokenType::BangEqual)
            || self.check_current_type(TokenType::EqualEqual)
        {
            let operator = self.advance();
            let right = self.comparison()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.check_current_type(TokenType::LessEqual)
            || self.check_current_type(TokenType::Less)
            || self.check_current_type(TokenType::Greater)
            || self.check_current_type(TokenType::GreaterEqual)
        {
            let operator = self.advance();
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.check_current_type(TokenType::Minus) || self.check_current_type(TokenType::Plus)
        {
            let operator = self.advance();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.check_current_type(TokenType::Star) || self.check_current_type(TokenType::Slash)
        {
            let op = self.advance();
            let right = self.unary()?;
            expr = Expr::binary(expr, op, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.check_current_type(TokenType::Bang) || self.check_current_type(TokenType::Minus) {
            let op = self.advance();
            let right = self.unary()?;
            return Ok(Expr::unary(op, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        let token = self.advance();
        Ok(match token.token_type {
            TokenType::False => Expr::literal(LiteralValue::Boolean(false)),
            TokenType::True => Expr::literal(LiteralValue::Boolean(true)),
            TokenType::Nil => Expr::literal(LiteralValue::Nil),
            TokenType::Number => {
                let Literal::Number(number) = token.literal.unwrap() else {
                    panic!("Error while handling token, TokenType::Number does not have a Literal::Number payload");
                };
                Expr::literal(LiteralValue::Number(number))
            }
            TokenType::String => {
                let Literal::String(s) = token.literal.unwrap() else {
                    panic!("Error while handling token, TokenType::String does not have a Literal::String payload");
                };
                Expr::literal(LiteralValue::String(s))
            }
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(
                    TokenType::RightParen,
                    "Expected ')' after expression".to_string(),
                )?;
                Expr::grouping(expr)
            }
            t => {
                return Err(LoxError::new(
                    token.line,
                    format!("unexpected token in primary expression: {t}"),
                ))
            }
        })
    }

    fn consume(&mut self, token_type: TokenType, msg: String) -> Result<(), LoxError> {
        if self.check_current_type(token_type) {
            self.advance();
            return Ok(());
        }
        Err(LoxError::new(self.peek().line, msg))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.peek().token_type {
                TokenType::Class => todo!(),
                TokenType::Fun => todo!(),
                TokenType::Var => todo!(),
                TokenType::For => todo!(),
                TokenType::If => todo!(),
                TokenType::While => todo!(),
                TokenType::Print => todo!(),
                TokenType::Return => todo!(),
                _ => todo!(),
            }
            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.tokens[self.current].token_type == TokenType::EOF
    }

    fn check_current_type(&self, token_type: TokenType) -> bool {
        if self.current >= self.tokens.len() {
            return false;
        }
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
