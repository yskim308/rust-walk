#![allow(dead_code)]

use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use rlox::{
    ast::parser::Parser,
    error::RuntimeSignal,
    interpreter::{stmt::Stmt, Interpreter},
    scanner::{token_type::TokenType, Scanner},
};

pub fn scan_types(source: &str) -> (Vec<TokenType>, Vec<RuntimeSignal>) {
    let mut scanner = Scanner::new(source.to_string());
    let (tokens, errors) = scanner.scan_tokens();
    (tokens.into_iter().map(|t| t.token_type).collect(), errors)
}

pub fn parse_source(source: &str) -> (Vec<Stmt>, Vec<RuntimeSignal>) {
    let mut scanner = Scanner::new(source.to_string());
    let (tokens, scan_errors) = scanner.scan_tokens();
    if !scan_errors.is_empty() {
        return (Vec::new(), scan_errors);
    }

    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn is_static_error(err: &RuntimeSignal) -> bool {
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

pub fn runtime_lines(source: &str) -> Vec<String> {
    let output = run_cli(source);
    stdout_runtime_lines(&output)
}

pub fn stdout_runtime_lines(output: &Output) -> Vec<String> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    let start = lines
        .iter()
        .rposition(|line| line.trim() == "]")
        .map_or(0, |idx| idx + 1);

    lines[start..]
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

pub fn stderr_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

fn temp_lox_file() -> PathBuf {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    std::env::temp_dir().join(format!("rlox-test-{}-{now}-{id}.lox", std::process::id()))
}
