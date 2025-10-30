use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_cat_single_file() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "hello world").unwrap();
    
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
}

#[test]
fn test_cat_multiple_files() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file1, "first file").unwrap();
    writeln!(file2, "second file").unwrap();
    
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("first file"))
        .stdout(predicate::str::contains("second file"));
}

#[test]
fn test_cat_number_lines() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "line one").unwrap();
    writeln!(file, "line two").unwrap();
    
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-n").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("     1\t"))
        .stdout(predicate::str::contains("     2\t"));
}

#[test]
fn test_cat_number_nonblank() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "line one").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "line three").unwrap();
    
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-b").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("     1\tline one"))
        .stdout(predicate::str::contains("     2\tline three"));
}

#[test]
fn test_cat_squeeze_blank() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "line one").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "line five").unwrap();
    
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-s").arg(file.path());
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    
    // Should have: line one, one blank, line five = 3 lines
    assert_eq!(lines.len(), 3);
}

#[test]
fn test_cat_nonexistent_file() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("nonexistent_file_12345.txt");
    cmd.assert()
        .failure();
}

#[test]
fn test_cat_stdin() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-");
    cmd.write_stdin("hello from stdin\n");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello from stdin"));
}

#[test]
fn test_cat_show_all() {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "hello\tworld").unwrap();
    
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("-A").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("^I")); // Tab shown as ^I
}

