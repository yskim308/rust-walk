mod common;

use common::{run_cli, run_with_interpreter};

#[test]
fn interpreter_runs_happy_path_program() {
    run_with_interpreter(
        r#"
        var total = 0;
        for (var i = 0; i < 3; i = i + 1) total = total + i;
        if (total == 3) print "ok"; else print "bad";
        while (false) print "no";
        print "hi" + "!";
        "#,
    );
}

#[test]
fn runtime_error_paths_report_runtime_error_type() {
    for source in ["var a; print a;", "print -\"x\";", "missing = 1;"] {
        let output = run_cli(source);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Runtime Error"));
    }
}
