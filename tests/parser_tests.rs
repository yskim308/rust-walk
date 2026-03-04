mod common;

use common::{is_static_error, parse_source};
use rlox::interpreter::stmt::Stmt;

#[test]
fn parses_all_statement_and_expression_forms() {
    let source = r#"
        var a = 1;
        var b;
        b = a + 2 * (3 - 1);
        print -1;
        if (true and !false or false) print b; else print a;
        while (false) print 0;
        for (var i = 0; i < 1; i = i + 1) print i;
        { print nil; }
        "x" + "y";
    "#;

    let (statements, errors) = parse_source(source);
    assert!(errors.is_empty());

    assert!(statements.iter().any(|s| matches!(s, Stmt::Var(..))));
    assert!(statements.iter().any(|s| matches!(s, Stmt::Expression(..))));
    assert!(statements.iter().any(|s| matches!(s, Stmt::Print(..))));
    assert!(statements.iter().any(|s| matches!(s, Stmt::If(..))));
    assert!(statements.iter().any(|s| matches!(s, Stmt::While(..))));
    assert!(statements.iter().any(|s| matches!(s, Stmt::Block(..))));
}

#[test]
fn parser_error_paths_are_static_errors() {
    for source in ["1 = 2;", "print 1"] {
        let (_statements, errors) = parse_source(source);
        assert!(!errors.is_empty());
        assert!(errors.iter().all(is_static_error));
    }
}
