use rlox::{ast::parser::Parser, scanner::scanner::Scanner};

fn parse_expression(source: &str) -> Result<String, String> {
    let mut scanner = Scanner::new(source.to_string());
    let (tokens, scan_errors) = scanner.scan_tokens();

    if let Some(error) = scan_errors.first() {
        return Err(error.to_string());
    }

    let mut parser = Parser::new(tokens);
    parser.parse().map(|expr| expr.to_string()).map_err(|e| e.to_string())
}

#[test]
fn parses_literal_expressions() {
    assert_eq!(parse_expression("123").unwrap(), "123");
    assert_eq!(parse_expression("\"hello\"").unwrap(), "\"hello\"");
    assert_eq!(parse_expression("true").unwrap(), "true");
    assert_eq!(parse_expression("false").unwrap(), "false");
    assert_eq!(parse_expression("nil").unwrap(), "Nil");
}

#[test]
fn parses_unary_expressions() {
    assert_eq!(parse_expression("-123").unwrap(), "(- 123)");
    assert_eq!(parse_expression("!false").unwrap(), "(! false)");
}

#[test]
fn parses_binary_precedence() {
    assert_eq!(parse_expression("1+2*3").unwrap(), "(1 + (2 * 3))");
    assert_eq!(parse_expression("1<2==false").unwrap(), "((1 < 2) == false)");
}

#[test]
fn parses_grouping_and_basic_nesting() {
    assert_eq!(parse_expression("!(1+2)").unwrap(), "(! (group (1 + 2)))");
}

