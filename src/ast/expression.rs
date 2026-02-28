use crate::scanner::token::Token;
use std::fmt;

#[derive(Debug)]
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
    Variable {
        token: Token,
    },
    Assignment {
        name: Token,
        value: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Expr {
    pub fn literal(value: LiteralValue) -> Self {
        Expr::Literal { value }
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn unary(token: Token, expression: Expr) -> Self {
        Expr::Unary {
            token,
            expression: Box::new(expression),
        }
    }

    pub fn binary(left_expr: Expr, operator: Token, right_expr: Expr) -> Self {
        Expr::Binary {
            left_expr: Box::new(left_expr),
            operator,
            right_expr: Box::new(right_expr),
        }
    }

    pub fn variable(token: Token) -> Self {
        Expr::Variable { token }
    }

    pub fn assignment(name: Token, value: Expr) -> Self {
        Expr::Assignment {
            name,
            value: Box::new(value),
        }
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Literal {
            value: LiteralValue::Number(value),
        }
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::Literal {
            value: LiteralValue::Number(value as f64),
        }
    }
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
            Expr::Variable { token } => write!(f, "{token}"),
        }
    }
}
