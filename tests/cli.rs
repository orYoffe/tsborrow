use std::process::Command;

#[test]
fn successful_check_prints_an_explicit_summary() {
    let output = Command::new(env!("CARGO_BIN_EXE_tsborrow"))
        .args(["check", "tests/fixtures/pass"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("ok: checked"), "stdout was: {stdout}");
    assert!(
        stdout.contains("no ownership violations found"),
        "stdout was: {stdout}"
    );
    assert!(
        stdout.contains("ownership contract(s)"),
        "stdout was: {stdout}"
    );
}

#[test]
fn unannotated_code_warns_instead_of_implying_full_coverage() {
    let output = Command::new(env!("CARGO_BIN_EXE_tsborrow"))
        .args(["check", "tests/samples/unannotated.ts"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stdout.contains("analyzed 0 ownership contract(s)"),
        "stdout was: {stdout}"
    );
    assert!(
        stderr.contains("found no ownership contracts"),
        "stderr was: {stderr}"
    );
}

#[test]
fn failing_check_returns_a_ci_failure_status() {
    let output = Command::new(env!("CARGO_BIN_EXE_tsborrow"))
        .args([
            "check",
            "tests/fixtures/fail/use-after-dispose.ts",
            "--format",
            "json",
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines = stdout.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 2, "stdout was: {stdout}");
    assert_eq!(
        lines[0],
        r#"{"file":"tests/fixtures/fail/use-after-dispose.ts","code":"TSB002","line":5,"column":1,"message":"use of disposed value: `file`"}"#
    );
    assert_eq!(
        lines[1],
        r#"{"status":"violations","files":1,"diagnostics":1,"trackedOwners":1,"trackedBorrows":0}"#
    );
}

#[test]
fn successful_json_check_reports_exact_counts() {
    let output = Command::new(env!("CARGO_BIN_EXE_tsborrow"))
        .args([
            "check",
            "tests/fixtures/pass/resource-disposed.ts",
            "--format",
            "json",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim_end(),
        r#"{"status":"ok","files":1,"diagnostics":0,"trackedOwners":1,"trackedBorrows":0}"#
    );
}
