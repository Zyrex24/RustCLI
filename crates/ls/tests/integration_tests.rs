use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_ls_current_directory() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.assert().success();
}

#[test]
fn test_ls_specific_directory() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    File::create(&file_path).unwrap();
    
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg(temp_dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test_file.txt"));
}

#[test]
fn test_ls_hidden_files() {
    let temp_dir = TempDir::new().unwrap();
    File::create(temp_dir.path().join(".hidden")).unwrap();
    File::create(temp_dir.path().join("visible.txt")).unwrap();
    
    // Without -a flag
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg(temp_dir.path());
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains(".hidden"));
    assert!(stdout.contains("visible.txt"));
    
    // With -a flag
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg("-a").arg(temp_dir.path());
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(".hidden"));
    assert!(stdout.contains("visible.txt"));
}

#[test]
fn test_ls_long_format() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "test content").unwrap();
    
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg("-l").arg(temp_dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test.txt"));
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Should contain permissions, size, and filename
    assert!(stdout.contains("-") || stdout.contains("r") || stdout.contains("w"));
}

#[test]
fn test_ls_human_readable() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large_file.txt");
    let mut file = File::create(&file_path).unwrap();
    
    // Create a file larger than 1KB
    let content = "x".repeat(2048);
    write!(file, "{}", content).unwrap();
    
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg("-lh").arg(temp_dir.path());
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Should contain K, M, or G for human-readable sizes
    assert!(stdout.contains('K') || stdout.contains('M') || stdout.contains('G') || stdout.contains('B'));
}

#[test]
fn test_ls_sort_by_time() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create files with different modification times
    let file1 = temp_dir.path().join("older.txt");
    File::create(&file1).unwrap();
    
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let file2 = temp_dir.path().join("newer.txt");
    File::create(&file2).unwrap();
    
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg("-t").arg(temp_dir.path());
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    
    // Newer file should come first
    let newer_idx = lines.iter().position(|&l| l.contains("newer.txt"));
    let older_idx = lines.iter().position(|&l| l.contains("older.txt"));
    
    if let (Some(newer), Some(older)) = (newer_idx, older_idx) {
        assert!(newer < older, "Newer file should be listed first");
    }
}

#[test]
fn test_ls_reverse_order() {
    let temp_dir = TempDir::new().unwrap();
    File::create(temp_dir.path().join("aaa.txt")).unwrap();
    File::create(temp_dir.path().join("zzz.txt")).unwrap();
    
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg("-r").arg(temp_dir.path());
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    
    let aaa_idx = lines.iter().position(|&l| l.contains("aaa.txt"));
    let zzz_idx = lines.iter().position(|&l| l.contains("zzz.txt"));
    
    if let (Some(aaa), Some(zzz)) = (aaa_idx, zzz_idx) {
        assert!(zzz < aaa, "zzz should come before aaa in reverse order");
    }
}

#[test]
fn test_ls_nonexistent_directory() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg("nonexistent_directory_12345");
    cmd.assert()
        .failure();
}

#[test]
fn test_ls_file_instead_of_directory() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("single_file.txt");
    File::create(&file_path).unwrap();
    
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.arg(&file_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("single_file.txt"));
}

