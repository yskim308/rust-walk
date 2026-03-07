use crate::scanner::token::Token;
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Assignment {
        name: Token,
        value: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left_expr: Box<Expr>,
        operator: Token,
        right_expr: Box<Expr>,
    },
    Unary {
        token: Token,
        expression: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Variable {
        token: Token,
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
    pub fn assignment(name: Token, value: Expr) -> Self {
        Expr::Assignment {
            name,
            value: Box::new(value),
        }
    }

    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn binary(left_expr: Expr, operator: Token, right_expr: Expr) -> Self {
        Expr::Binary {
            left_expr: Box::new(left_expr),
            operator,
            right_expr: Box::new(right_expr),
        }
    }

    pub fn unary(token: Token, expression: Expr) -> Self {
        Expr::Unary {
            token,
            expression: Box::new(expression),
        }
    }

    pub fn call(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn literal(value: LiteralValue) -> Self {
        Expr::Literal { value }
    }

    pub fn variable(token: Token) -> Self {
        Expr::Variable { token }
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
            Expr::Assignment { name, value } => write!(f, "{} = {value}", name.lexeme),
            Expr::Logical {
                left,
                operator,
                right,
            } => write!(f, "{left} {} {right}", operator.lexeme),
            Expr::Binary {
                left_expr,
                operator,
                right_expr,
            } => {
                write!(f, "({left_expr} {} {right_expr})", operator.lexeme)
            }
            Expr::Unary { token, expression } => {
                write!(f, "({} {expression})", token.lexeme)
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => write!(f, "{callee}({:?})", arguments),
            Expr::Grouping { expression } => write!(f, "(group {expression})"),
            Expr::Literal { value } => write!(f, "{value}"),
            Expr::Variable { token } => write!(f, "{token}"),
        }
    }
}
