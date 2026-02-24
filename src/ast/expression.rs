use crate::scanner::token::Token;
use std::fmt;

pub enum Expr {
    Literal {
        value: LiteralValue,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        token: Token,
        expression: Box<Expr>,
    },
    Binary {
        left_expr: Box<Expr>,
        operator: Token,
        right_expr: Box<Expr>,
    },
}

pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{n}"),
            LiteralValue::String(s) => write!(f, "{s}"),
            LiteralValue::Boolean(b) => write!(f, "{b}"),
            LiteralValue::Nil => write!(f, "Nil"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal { value } => write!(f, "{value}"),
            Expr::Grouping { expression } => write!(f, "(group {expression})"),
            Expr::Unary { token, expression } => {
                write!(f, "({} {expression})", token.lexeme)
            }
            Expr::Binary {
                left_expr,
                operator,
                right_expr,
            } => {
                write!(f, "({left_expr} {} {right_expr})", operator.lexeme)
            }
        }
    }
}
