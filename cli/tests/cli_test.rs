use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("rssh").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage reverse SSH connections"));
}

#[test]
fn test_cli_profile_list() {
    let mut cmd = Command::cargo_bin("rssh").unwrap();
    cmd.arg("profile").arg("list")
        .assert()
        .success();
}

#[test]
fn test_cli_status() {
    let mut cmd = Command::cargo_bin("rssh").unwrap();
    cmd.arg("status")
        .assert()
        .success();
}
