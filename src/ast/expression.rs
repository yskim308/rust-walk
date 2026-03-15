use crate::scanner::token::Token;
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Assignment {
        id: u32,
        name: Token,
        value: Box<Expr>,
    },
    Logical {
        id: u32,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        id: u32,
        left_expr: Box<Expr>,
        operator: Token,
        right_expr: Box<Expr>,
    },
    Unary {
        id: u32,
        token: Token,
        expression: Box<Expr>,
    },
    Call {
        id: u32,
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        id: u32,
        expression: Box<Expr>,
    },
    Literal {
        id: u32,
        value: LiteralValue,
    },
    Variable {
        id: u32,
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
    pub fn id(&self) -> u32 {
        match self {
            Expr::Assignment { id, .. } => *id,
            Expr::Logical { id, .. } => *id,
            Expr::Binary { id, .. } => *id,
            Expr::Unary { id, .. } => *id,
            Expr::Call { id, .. } => *id,
            Expr::Grouping { id, .. } => *id,
            Expr::Literal { id, .. } => *id,
            Expr::Variable { id, .. } => *id,
        }
    }

    pub fn assignment(id: u32, name: Token, value: Expr) -> Self {
        Expr::Assignment {
            id,
            name,
            value: Box::new(value),
        }
    }

    pub fn logical(id: u32, left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Logical {
            id,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn binary(id: u32, left_expr: Expr, operator: Token, right_expr: Expr) -> Self {
        Expr::Binary {
            id,
            left_expr: Box::new(left_expr),
            operator,
            right_expr: Box::new(right_expr),
        }
    }

    pub fn unary(id: u32, token: Token, expression: Expr) -> Self {
        Expr::Unary {
            id,
            token,
            expression: Box::new(expression),
        }
    }

    pub fn call(id: u32, callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Expr::Call {
            id,
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }

    pub fn grouping(id: u32, expression: Expr) -> Self {
        Expr::Grouping {
            id,
            expression: Box::new(expression),
        }
    }

    pub fn literal(id: u32, value: LiteralValue) -> Self {
        Expr::Literal { id, value }
    }

    pub fn variable(id: u32, token: Token) -> Self {
        Expr::Variable { id, token }
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
            Expr::Assignment { name, value, .. } => write!(f, "{} = {value}", name.lexeme),
            Expr::Logical {
                id: _,
                left,
                operator,
                right,
            } => write!(f, "{left} {} {right}", operator.lexeme),
            Expr::Binary {
                id: _,
                left_expr,
                operator,
                right_expr,
            } => {
                write!(f, "({left_expr} {} {right_expr})", operator.lexeme)
            }
            Expr::Unary { token, expression, .. } => {
                write!(f, "({} {expression})", token.lexeme)
            }
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => write!(f, "{callee}({:?})", arguments),
            Expr::Grouping { expression, .. } => write!(f, "(group {expression})"),
            Expr::Literal { value, .. } => write!(f, "{value}"),
            Expr::Variable { token, .. } => write!(f, "{token}"),
        }
    }
}
