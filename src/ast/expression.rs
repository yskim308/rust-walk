use crate::scanner::token::Token;

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

impl Expr {
    pub fn pretty_print(&self) -> String {
        match self {
            Expr::Literal { value } => match value {
                LiteralValue::Number(n) => n.to_string(),
                LiteralValue::String(s) => s.clone(),
                LiteralValue::Boolean(b) => b.to_string(),
                LiteralValue::Nil => "Nil".to_string(),
            },
            Expr::Grouping { expression } => expression.pretty_print(),
            Expr::Unary { token, expression } => {
                format!("{} {}", token.lexeme, expression.pretty_print())
            }
            Expr::Binary {
                left_expr,
                operator,
                right_expr,
            } => {
                format!(
                    "{} {} {}",
                    left_expr.pretty_print(),
                    operator.lexeme,
                    right_expr.pretty_print()
                )
            }
        }
    }
}
