mod common;

use common::{is_static_error, parse_source};
use rlox::{ast::expression::Expr, interpreter::stmt::Stmt};

#[test]
fn parses_every_statement_form() {
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
    assert_eq!(statements.len(), 9);

    assert!(matches!(statements[0], Stmt::Var(..)));
    assert!(matches!(statements[1], Stmt::Var(..)));
    assert!(matches!(statements[2], Stmt::Expression(..)));
    assert!(matches!(statements[3], Stmt::Print(..)));
    assert!(matches!(statements[4], Stmt::If(..)));
    assert!(matches!(statements[5], Stmt::While(..)));
    assert!(matches!(
        &statements[6],
        Stmt::Block(stmts)
            if matches!(stmts.first(), Some(Stmt::Var(..)))
                && matches!(stmts.get(1), Some(Stmt::While(..)))
    ));
    assert!(matches!(statements[7], Stmt::Block(..)));
    assert!(matches!(statements[8], Stmt::Expression(..)));
}

#[test]
fn parses_every_expression_form() {
    let (statements, errors) = parse_source(
        r#"
        1;
        (1);
        -1;
        !false;
        1 + 2;
        true and false;
        a;
        a = 1;
        nil;
        "s";
        "#,
    );

    assert!(errors.is_empty());
    assert_eq!(statements.len(), 10);
    assert!(matches!(
        statements[0],
        Stmt::Expression(Expr::Literal { .. })
    ));
    assert!(matches!(
        statements[1],
        Stmt::Expression(Expr::Grouping { .. })
    ));
    assert!(matches!(
        statements[2],
        Stmt::Expression(Expr::Unary { .. })
    ));
    assert!(matches!(
        statements[3],
        Stmt::Expression(Expr::Unary { .. })
    ));
    assert!(matches!(
        statements[4],
        Stmt::Expression(Expr::Binary { .. })
    ));
    assert!(matches!(
        statements[5],
        Stmt::Expression(Expr::Logical { .. })
    ));
    assert!(matches!(
        statements[6],
        Stmt::Expression(Expr::Variable { .. })
    ));
    assert!(matches!(
        statements[7],
        Stmt::Expression(Expr::Assignment { .. })
    ));
    assert!(matches!(
        statements[8],
        Stmt::Expression(Expr::Literal { .. })
    ));
    assert!(matches!(
        statements[9],
        Stmt::Expression(Expr::Literal { .. })
    ));
}

#[test]
fn parses_function_call_expressions() {
    let (statements, errors) = parse_source(
        r#"
        foo();
        foo(1, 2, 3);
        foo()(1);
        "#,
    );

    assert!(errors.is_empty());
    assert_eq!(statements.len(), 3);

    assert!(matches!(
        &statements[0],
        Stmt::Expression(Expr::Call { callee, arguments, .. })
            if matches!(**callee, Expr::Variable { .. }) && arguments.is_empty()
    ));
    assert!(matches!(
        &statements[1],
        Stmt::Expression(Expr::Call { callee, arguments, .. })
            if matches!(**callee, Expr::Variable { .. }) && arguments.len() == 3
    ));
    assert!(matches!(
        &statements[2],
        Stmt::Expression(Expr::Call { callee, arguments, .. })
            if matches!(**callee, Expr::Call { .. }) && arguments.len() == 1
    ));
}

#[test]
fn parser_error_paths_are_static_errors() {
    for source in ["1 = 2;", "print 1", "else print 1;"] {
        let (_statements, errors) = parse_source(source);
        assert!(!errors.is_empty());
        assert!(errors.iter().all(is_static_error));
    }
}
