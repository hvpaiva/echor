use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_with_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs_with_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();

    cmd.arg("Hello, world!").assert().success();
}
