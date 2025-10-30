use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_echo_simple() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("hello").arg("world");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello world\n"));
}

#[test]
fn test_echo_no_newline() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("-n").arg("hello");
    cmd.assert()
        .success()
        .stdout(predicate::eq("hello"));
}

#[test]
fn test_echo_with_escape() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("-e").arg("hello\\nworld");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello\nworld\n"));
}

#[test]
fn test_echo_without_escape() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("hello\\nworld");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello\\nworld\n"));
}

#[test]
fn test_echo_explicit_no_escape() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("-E").arg("hello\\nworld");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello\\nworld\n"));
}

#[test]
fn test_echo_escape_overrides_no_escape() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("-e").arg("-E").arg("hello\\nworld");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello\\nworld\n"));
}

#[test]
fn test_echo_multiple_args() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("one").arg("two").arg("three");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("one two three\n"));
}

#[test]
fn test_echo_empty() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::eq("\n"));
}

#[test]
fn test_echo_tab_escape() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("-e").arg("hello\\tworld");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello\tworld\n"));
}

#[test]
fn test_echo_backslash_escape() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("-e").arg("hello\\\\world");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello\\world\n"));
}

