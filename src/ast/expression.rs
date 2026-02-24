use crate::scanner::token_type::TokenType;

pub enum Expr {
    Literal {
        value: LiteralValue,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        token: TokenType,
        expression: Box<Expr>,
    },
    Binary {
        left_expr: Box<Expr>,
        operator: TokenType,
        right_expr: Box<Expr>,
    },
}

pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
