use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_pwd_basic() {
    let mut cmd = Command::cargo_bin("pwd").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_pwd_output_is_absolute_path() {
    let mut cmd = Command::cargo_bin("pwd").unwrap();
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Output should be an absolute path
    assert!(stdout.contains(std::path::MAIN_SEPARATOR));
}

#[test]
fn test_pwd_logical_flag() {
    let mut cmd = Command::cargo_bin("pwd").unwrap();
    cmd.arg("-L");
    cmd.assert().success();
}

#[test]
fn test_pwd_physical_flag() {
    let mut cmd = Command::cargo_bin("pwd").unwrap();
    cmd.arg("-P");
    cmd.assert().success();
}

