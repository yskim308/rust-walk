#![allow(dead_code)]

use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use rlox::{
    ast::parser::Parser,
    error::LoxError,
    interpreter::{stmt::Stmt, Interpreter},
    scanner::{token_type::TokenType, Scanner},
};

pub fn scan_types(source: &str) -> (Vec<TokenType>, Vec<LoxError>) {
    let mut scanner = Scanner::new(source.to_string());
    let (tokens, errors) = scanner.scan_tokens();
    (tokens.into_iter().map(|t| t.token_type).collect(), errors)
}

pub fn parse_source(source: &str) -> (Vec<Stmt>, Vec<LoxError>) {
    let mut scanner = Scanner::new(source.to_string());
    let (tokens, scan_errors) = scanner.scan_tokens();
    if !scan_errors.is_empty() {
        return (Vec::new(), scan_errors);
    }

    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn is_static_error(err: &LoxError) -> bool {
    err.to_string().starts_with("Static Error")
}

pub fn run_with_interpreter(source: &str) {
    let (statements, errors) = parse_source(source);
    assert!(errors.is_empty(), "expected parse success");

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&statements);
}

pub fn run_cli(source: &str) -> Output {
    let path = temp_lox_file();
    fs::write(&path, source).expect("failed to write temp lox file");

    let output = Command::new(env!("CARGO_BIN_EXE_rlox"))
        .arg(&path)
        .output()
        .expect("failed to run rlox binary");

    let _ = fs::remove_file(path);
    output
}

fn temp_lox_file() -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("rlox-test-{}-{now}.lox", std::process::id()))
}
