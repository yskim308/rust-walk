mod common;

use rlox::scanner::token_type::TokenType::*;

use common::{is_static_error, scan_types};

#[test]
fn scans_every_token_type_once() {
    let source = "(){} ,.-+;* ! != = == < <= > >= / id \"s\" 1 and class else false fun for if nil or print return super this true var while";
    let (types, errors) = scan_types(source);

    assert!(errors.is_empty());
    assert_eq!(
        types,
        vec![
            LeftParen,
            RightParen,
            LeftBrace,
            RightBrace,
            Comma,
            Dot,
            Minus,
            Plus,
            Semicolon,
            Star,
            Bang,
            BangEqual,
            Equal,
            EqualEqual,
            Less,
            LessEqual,
            Greater,
            GreaterEqual,
            Slash,
            Identifier,
            String,
            Number,
            And,
            Class,
            Else,
            False,
            Fun,
            For,
            If,
            Nil,
            Or,
            Print,
            Return,
            Super,
            This,
            True,
            Var,
            While,
            EOF,
        ]
    );
}

#[test]
fn scanner_error_paths_are_static_errors() {
    for source in ["@", "\"", "/*"] {
        let (_types, errors) = scan_types(source);
        assert_eq!(errors.len(), 1);
        assert!(is_static_error(&errors[0]));
    }
}

#[test]
fn scanner_skips_comments() {
    let (types, errors) = scan_types("// line\n/* block */ + /");
    assert!(errors.is_empty());
    assert_eq!(types, vec![Plus, Slash, EOF]);
}
