use crate::{
    ast::expression::{Expr, LiteralValue},
    error::RuntimeSignal,
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

    pub fn parse(&mut self) -> (Vec<Stmt>, Vec<RuntimeSignal>) {
        let mut statements: Vec<Stmt> = Vec::new();
        let mut errors: Vec<RuntimeSignal> = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    errors.push(e);
                    self.synchronize();
                }
            }
        }

        (statements, errors)
    }

    fn declaration(&mut self) -> Result<Stmt, RuntimeSignal> {
        match self.peek().token_type {
            TokenType::Fun => self.fun_declaration(),
            TokenType::Var => self.var_declaration(),
            _ => self.statement(),
        }
    }

    fn fun_declaration(&mut self) -> Result<Stmt, RuntimeSignal> {
        self.consume(TokenType::Fun, "'fun expected'".into())?;

        let name = self.consume(
            TokenType::Identifier,
            "Expected function name after 'fun'".into(),
        )?;

        self.consume(
            TokenType::LeftParen,
            "Expected '(' after function name".into(),
        )?;

        let mut parameters = Vec::new();

        if self.peek().token_type != TokenType::RightParen {
            loop {
                parameters
                    .push(self.consume(TokenType::Identifier, "Expected parameter name".into())?);

                if self.peek().token_type == TokenType::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters".into())?;

        self.consume(TokenType::LeftBrace, "Expect { before function body".into())?;

        let body = self.block()?;

        Ok(Stmt::function(name, parameters, body))
    }
    fn var_declaration(&mut self) -> Result<Stmt, RuntimeSignal> {
        self.advance();

        let name = self.consume(TokenType::Identifier, "Expected variable name".to_string())?;

        let mut initializer: Option<Expr> = None;

        if self.peek().token_type == TokenType::Equal {
            self.advance();
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ; after varialbe initialization".to_string(),
        )?;

        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, RuntimeSignal> {
        match self.peek().token_type {
            TokenType::If => {
                self.advance();
                self.if_statement()
            }
            TokenType::Print => {
                self.advance();
                self.print_statement()
            }
            TokenType::While => {
                self.advance();
                self.while_statement()
            }
            TokenType::LeftBrace => {
                self.advance();
                self.block()
            }
            TokenType::For => {
                self.advance();
                self.for_statement()
            }
            TokenType::Return => self.return_statement(),
            _ => self.expression_statement(),
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, RuntimeSignal> {
        let keyword = self.advance();

        let value = if self.peek().token_type == TokenType::Semicolon {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value".into());

        Ok(Stmt::Return(keyword, value))
    }

    fn for_statement(&mut self) -> Result<Stmt, RuntimeSignal> {
        self.consume(
            TokenType::LeftParen,
            "Expected '(' after 'for'.".to_string(),
        )?;

        let initalizer = if self.peek().token_type == TokenType::Semicolon {
            self.advance();
            None
        } else {
            Some(self.declaration()?)
        };

        let condition = if self.peek().token_type == TokenType::Semicolon {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after loop condition.".into(),
        )?;

        let increment = if self.peek().token_type == TokenType::RightParen {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(
            TokenType::RightParen,
            "Expected ')' after for clauses".into(),
        )?;

        let body = self.statement()?;

        let body = if let Some(inc) = increment {
            Stmt::Block(vec![body, Stmt::Expression(inc)])
        } else {
            body
        };

        let condition = if let Some(cond) = condition {
            cond
        } else {
            Expr::literal(LiteralValue::Boolean(true))
        };

        let body = Stmt::while_statement(condition, body);

        match initalizer {
            Some(init) => Ok(Stmt::Block(vec![init, body])),
            None => Ok(body),
        }
    }

    fn while_statement(&mut self) -> Result<Stmt, RuntimeSignal> {
        self.consume(
            TokenType::LeftParen,
            "Expected '(' after 'while'".to_string(),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expected ')' after while condition".to_string(),
        )?;

        let body = self.statement()?;

        Ok(Stmt::while_statement(condition, body))
    }

    fn if_statement(&mut self) -> Result<Stmt, RuntimeSignal> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'.".to_string())?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expected ')' after if condition".to_string(),
        )?;

        let then_branch = self.statement()?;

        let else_branch = match self.peek().token_type {
            TokenType::Else => {
                self.advance();
                Some(self.statement()?)
            }
            _ => None,
        };

        Ok(Stmt::if_statement(condition, then_branch, else_branch))
    }

    fn block(&mut self) -> Result<Stmt, RuntimeSignal> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() && self.peek().token_type != TokenType::RightBrace {
            statements.push(self.declaration()?);
        }

        self.consume(
            TokenType::RightBrace,
            "Expected '}' after block".to_string(),
        )?;

        Ok(Stmt::Block(statements))
    }

    fn print_statement(&mut self) -> Result<Stmt, RuntimeSignal> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value".to_string())?;
        Ok(Stmt::Print(expression))
    }

    fn expression_statement(&mut self) -> Result<Stmt, RuntimeSignal> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value".to_string())?;
        Ok(Stmt::Expression(expression))
    }

    fn expression(&mut self) -> Result<Expr, RuntimeSignal> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, RuntimeSignal> {
        let expr = self.or()?;

        if self.peek().token_type == TokenType::Equal {
            let token = self.peek().clone();
            self.advance();

            let value = self.assignment()?;

            match expr {
                Expr::Variable { token } => return Ok(Expr::assignment(token, value)),
                _ => {
                    return Err(RuntimeSignal::static_error(
                        token.line,
                        "Invalid assignment target".to_string(),
                    ))
                }
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, RuntimeSignal> {
        let mut expr = self.and()?;

        while self.peek().token_type == TokenType::Or {
            let operator = self.peek().clone();
            self.advance();
            let right = self.and()?;
            expr = Expr::logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, RuntimeSignal> {
        let mut expr = self.equality()?;

        while self.peek().token_type == TokenType::And {
            let operator = self.peek().clone();
            self.advance();
            let right = self.equality()?;
            expr = Expr::logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, RuntimeSignal> {
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

    fn comparison(&mut self) -> Result<Expr, RuntimeSignal> {
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

    fn term(&mut self) -> Result<Expr, RuntimeSignal> {
        let mut expr = self.factor()?;

        while self.check_current_type(TokenType::Minus) || self.check_current_type(TokenType::Plus)
        {
            let operator = self.advance();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, RuntimeSignal> {
        let mut expr = self.unary()?;

        while self.check_current_type(TokenType::Star) || self.check_current_type(TokenType::Slash)
        {
            let op = self.advance();
            let right = self.unary()?;
            expr = Expr::binary(expr, op, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, RuntimeSignal> {
        if self.check_current_type(TokenType::Bang) || self.check_current_type(TokenType::Minus) {
            let op = self.advance();
            let right = self.unary()?;
            return Ok(Expr::unary(op, right));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, RuntimeSignal> {
        let mut expr = self.primary()?;

        // suposedly has to be this way for some future feature?
        loop {
            if self.peek().token_type == TokenType::LeftParen {
                self.advance();
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, RuntimeSignal> {
        let mut arguments = Vec::new();

        if self.peek().token_type != TokenType::RightParen {
            loop {
                arguments.push(self.expression()?);

                if self.peek().token_type == TokenType::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            "Expect ')' after function arguments".into(),
        )?;

        Ok(Expr::call(callee, paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, RuntimeSignal> {
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
            TokenType::Identifier => Expr::variable(token),
            t => {
                return Err(RuntimeSignal::static_error(
                    token.line,
                    format!("unexpected token in primary expression: {t}"),
                ))
            }
        })
    }

    fn consume(&mut self, token_type: TokenType, msg: String) -> Result<Token, RuntimeSignal> {
        if self.check_current_type(token_type) {
            return Ok(self.advance());
        }
        Err(RuntimeSignal::static_error(self.peek().line, msg))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            // match self.peek().token_type {
            //     TokenType::Class => todo!(),
            //     TokenType::Fun => todo!(),
            //     TokenType::Var => todo!(),
            //     TokenType::For => todo!(),
            //     TokenType::If => todo!(),
            //     TokenType::While => todo!(),
            //     TokenType::Print => todo!(),
            //     TokenType::Return => todo!(),
            //     _ => todo!(),
            // }
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
