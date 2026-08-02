use std::process::Command;

#[test]
fn successful_check_prints_an_explicit_summary() {
    let output = Command::new(env!("CARGO_BIN_EXE_ts-borrow-checker"))
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
    let output = Command::new(env!("CARGO_BIN_EXE_ts-borrow-checker"))
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
    let output = Command::new(env!("CARGO_BIN_EXE_ts-borrow-checker"))
        .args(["check", "tests/fixtures/fail"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(!output.stdout.is_empty());
}
