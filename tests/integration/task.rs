use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_task_command_fails_without_git_repo() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd
        .arg("task")
        .arg("feat/test-feature")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    // Should fail because project not initialized (early validation)
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Project not initialized") || stderr.contains("claudectl init"));
}

#[test]
fn test_task_command_fails_without_config() {
    let temp_dir = TempDir::new().unwrap();

    // Set up git repo but no claudectl config
    fs::create_dir(temp_dir.path().join(".git")).unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd
        .arg("task")
        .arg("feat/test-feature")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    // Should fail because project not initialized (early validation)
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Project not initialized") || stderr.contains("claudectl init"));
}

#[test]
fn test_task_command_shows_initialization_error() {
    let temp_dir = TempDir::new().unwrap();

    // Set up git repo to pass first check
    fs::create_dir(temp_dir.path().join(".git")).unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd
        .arg("task")
        .arg("feat/test-feature")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    // Should fail with helpful initialization message
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Configuration file not found"));
    assert!(stderr.contains("claudectl init"));
}

#[test]
fn test_task_command_validation_with_empty_task_name() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd
        .arg("task")
        .arg("")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    // Should fail with argument validation error
    assert!(!output.status.success());
}
