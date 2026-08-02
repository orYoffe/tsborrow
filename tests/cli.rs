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
