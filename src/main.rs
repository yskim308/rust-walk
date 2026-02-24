use std::{
    env::{self},
    fs, io,
};

use rlox::{
    ast::expression::{Expr, LiteralValue},
    scanner::{token::Token, token_type::TokenType},
};

fn main() {
    println!("Hello, world!");
    check_pretty_print();
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length > 2 {
        print!("USAGE: ./binary /path/to/file \n OR \n cargo run -- path/to/file")
    } else if length == 2 {
        run_from_file(args.get(1).unwrap()).unwrap_or_else(|err| {
            panic!("failed to run from file: {:?}", err);
        });
    } else {
        run_by_prompt().unwrap_or_else(|err| {
            panic!("failed to run interactive prompt: {:?}", err);
        });
    }
}

fn run_from_file(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    run(content);
    Ok(())
}

fn run_by_prompt() -> io::Result<()> {
    loop {
        print!("> ");

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line)?;

        if bytes_read == 0 {
            print!("");
            break;
        }

        run(line);
    }

    Ok(())
}

fn run(_source: String) {
    todo!("scanner + tokens must be implemented in run");
}

fn check_pretty_print() {
    let expr = Expr::Binary {
        left_expr: Box::new(Expr::Unary {
            token: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(123.),
            }),
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right_expr: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        }),
    };
    println!("{expr}");
}
