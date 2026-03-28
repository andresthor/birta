use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn nonexistent_file_exits_with_error() {
    Command::cargo_bin("birta")
        .unwrap()
        .arg("nonexistent.md")
        .assert()
        .failure()
        .stderr(predicate::str::contains("file not found"));
}

#[test]
fn help_flag_exits_successfully() {
    Command::cargo_bin("birta")
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn version_flag_shows_version() {
    Command::cargo_bin("birta")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("birta"));
}
