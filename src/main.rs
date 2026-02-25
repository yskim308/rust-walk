use std::{
    env::{self},
    fs, io,
};

use rlox::{
    ast::{expression::Expr, parser::Parser},
    scanner::{scanner::Scanner, token::Token, token_type::TokenType},
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

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let (tokens, errors) = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    todo!("finish the rest?")
}

fn check_pretty_print() {
    let left_expr = Expr::unary(
        Token::new(TokenType::Minus, "-".into(), None, 1),
        123.into(),
    );

    let star = Token::new(TokenType::Star, "*".into(), None, 1);

    let right_expr = Expr::grouping(45.67.into());

    let bin_expr = Expr::binary(left_expr, star, right_expr);
    println!("{bin_expr}");
}
