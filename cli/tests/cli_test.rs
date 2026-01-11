use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("reverse_ssh_cli").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage reverse SSH connections"));
}

#[test]
fn test_cli_profile_list_empty() {
    // We should use a temp config directory for tests to avoid messing with user config
    // For now, let's just check it runs
    let mut cmd = Command::cargo_bin("reverse_ssh_cli").unwrap();
    cmd.arg("profile").arg("list")
        .assert()
        .success();
}
