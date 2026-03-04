mod common;

use common::{run_cli, runtime_lines, stderr_text, stdout_runtime_lines};

#[test]
fn interprets_statement_and_expression_happy_paths() {
    let lines = runtime_lines(
        r#"
        var x = 1;
        x = x + 1;
        print x;

        { var x = 99; print x; }

        if (x == 2) print "IF"; else print "NO";
        if (false) print "NO"; else print "ELSE";

        var c = 0;
        while (c < 2) c = c + 1;
        print c;

        for (var i = 0; false; i = i + 1) print i;

        print !false;
        print -3;
        print 1 + 2;
        print 3 * 4;
        print 8 / 2;
        print 3 > 2;
        print 3 >= 3;
        print 2 < 3;
        print 2 <= 2;
        print 1 != 2;
        print 1 == 1;
        print true or false;
        print false and true;
        print "a" + "b";
        "#,
    );

    assert_eq!(
        lines,
        [
            "2", "99", "\"IF\"", "\"ELSE\"", "2", "true", "-3", "3", "12", "4", "true", "true",
            "true", "true", "true", "true", "true", "false", "\"a\"\"b\"",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>()
    );
}

#[test]
fn for_loop_increment_updates_loop_variable() {
    let output = run_cli("for (var i = 0; i < 2; i = i + 1) print i;");
    assert!(!stderr_text(&output).contains("Runtime Error"));
    assert_eq!(stdout_runtime_lines(&output), vec!["0", "1"]);
}

#[test]
fn runtime_error_paths_report_runtime_error_type() {
    for source in [
        "var a; print a;",
        "print -\"x\";",
        "print true - 1;",
        "print true + false;",
        "missing = 1;",
    ] {
        let output = run_cli(source);
        assert!(stderr_text(&output).contains("Runtime Error"));
    }
}
