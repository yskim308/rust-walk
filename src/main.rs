use std::{
    env::{self},
    fs, io,
};

use rlox::{
    ast::{expression::Expr, parser::Parser},
    interpreter::Interpreter,
    scanner::{token::Token, token_type::TokenType, Scanner},
};

fn main() {
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
    // scan tokens
    let mut scanner = Scanner::new(source);
    let (tokens, errors) = scanner.scan_tokens();
    if !errors.is_empty() {
        for error in errors {
            println!("{error}")
        }
        return;
    }

    // parse tokens into AST
    let mut parser = Parser::new(tokens);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(_) => {
            todo!("handle synchronizations after statements");
            return;
        }
    };

    // interpret the AST
    let interpreter = Interpreter::new();
    interpreter.interpret(statements);
    todo!("finish")
    // 1. synchronizing in the parser to return a list of errors
}
