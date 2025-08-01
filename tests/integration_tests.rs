use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A command-line tool for managing Claude projects"));
}

#[test]
fn test_workspace_help() {
    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    cmd.args(&["workspace", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Manage workspaces"));
}

#[test]
fn test_workspace_new_help() {
    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    cmd.args(&["workspace", "new", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialize a new workspace"));
}

#[test]
fn test_workspace_list_help() {
    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    cmd.args(&["workspace", "list", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("List all workspaces"));
}

#[test]
fn test_invalid_workspace_name() {
    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    cmd.args(&["workspace", "new", "invalid/name"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Validation error: Workspace name cannot contain path separators"));
}

#[test]
fn test_empty_workspace_name() {
    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    cmd.args(&["workspace", "new", ""]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Validation error: Workspace name cannot be empty"));
}

#[test]
fn test_long_workspace_name() {
    let long_name = "a".repeat(101);
    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    cmd.args(&["workspace", "new", &long_name]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Validation error: Workspace name too long"));
}

#[test]
fn test_workspace_list_empty() {
    // Create a temporary directory and run the command there
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.args(&["workspace", "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No workspaces found"));
}

#[test]
fn test_no_args_shows_help() {
    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    cmd.assert()
        .failure()  // clap exits with failure when no required args provided
        .stderr(predicate::str::contains("Usage:"));
}