use rlox::scanner::{scanner::Scanner, token::Literal, token_type::TokenType};

#[test]
fn operators_scanned_properly() {
    let mut scanner = Scanner::new("!==<=>=".to_string());
    let (tokens, errors) = scanner.scan_tokens();
    assert_eq!(errors.len(), 0);

    assert!(matches!(tokens[0].token_type, TokenType::BangEqual));
    assert_eq!(tokens[0].lexeme, "!=");

    assert!(matches!(tokens[1].token_type, TokenType::Equal));
    assert_eq!(tokens[1].lexeme, "=");

    assert!(matches!(tokens[2].token_type, TokenType::LessEqual));
    assert_eq!(tokens[2].lexeme, "<=");

    assert!(matches!(tokens[3].token_type, TokenType::GreaterEqual));
    assert_eq!(tokens[3].lexeme, ">=");

    assert!(matches!(tokens[4].token_type, TokenType::EOF));
}

#[test]
fn literals_scanned_properly() {
    let mut scanner = Scanner::new("\"hello\"123.45".to_string());
    let (tokens, errors) = scanner.scan_tokens();
    assert_eq!(errors.len(), 0);

    assert!(matches!(tokens[0].token_type, TokenType::String));
    assert!(matches!(tokens[0].literal, Some(Literal::String(_))));
    assert_eq!(tokens[0].lexeme, "\"hello\"");

    assert!(matches!(tokens[1].token_type, TokenType::Number));
    assert!(matches!(tokens[1].literal, Some(Literal::Number(_))));
    assert_eq!(tokens[1].lexeme, "123.45");

    assert!(matches!(tokens[2].token_type, TokenType::EOF));
}

#[test]
fn identifiers_scanned_properly() {
    let mut scanner = Scanner::new("myVar".to_string());
    let (tokens, errors) = scanner.scan_tokens();
    assert_eq!(errors.len(), 0);

    assert!(matches!(tokens[0].token_type, TokenType::Identifier));
    assert_eq!(tokens[0].lexeme, "myVar");
    assert!(tokens[0].literal.is_none());

    assert!(matches!(tokens[1].token_type, TokenType::EOF));
}

mod operator_errors {
    use super::*;

    #[test]
    fn unexpected_operator_character_is_reported() {
        let mut scanner = Scanner::new("@".to_string());
        let (_tokens, errors) = scanner.scan_tokens();
        assert_eq!(errors.len(), 1);
    }
}

mod literal_errors {
    use super::*;

    #[test]
    fn unterminated_string_is_reported() {
        let mut scanner = Scanner::new("\"".to_string());
        let (_tokens, errors) = scanner.scan_tokens();
        assert_eq!(errors.len(), 1);
    }
}
