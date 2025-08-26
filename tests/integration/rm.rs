use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_rm_command_fails_without_git_repo() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd
        .arg("rm")
        .arg("some-task")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    // Should fail because not in git repo or no config
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Configuration file not found")
            || stderr.contains("Git Error")
            || stderr.contains("not a git repository")
    );
}

#[test]
fn test_rm_command_fails_without_config() {
    let temp_dir = TempDir::new().unwrap();

    // Set up git repo but no claudectl config
    fs::create_dir(temp_dir.path().join(".git")).unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd
        .arg("rm")
        .arg("some-task")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    // Should fail because no configuration file
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Configuration file not found")
            || stderr.contains("Failed to read local configuration")
            || stderr.contains("No such file or directory")
    );
}

#[test]
fn test_rm_command_fails_for_nonexistent_task() {
    let temp_dir = TempDir::new().unwrap();

    // Set up git repo
    fs::create_dir(temp_dir.path().join(".git")).unwrap();

    // Create claudectl config directory and file
    let config_dir = temp_dir.path().join(".claudectl");
    fs::create_dir(&config_dir).unwrap();
    let config_content = r#"{
        "project_name": "test-project",
        "project_dir": "/tmp/test"
    }"#;
    fs::write(config_dir.join("config.json"), config_content).unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd
        .arg("rm")
        .arg("nonexistent-task")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    // Should fail because task doesn't exist
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Task 'nonexistent-task' not found")
            || stderr.contains("Failed to get tasks")
    );
}

#[test]
fn test_rm_command_requires_task_argument() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd.arg("rm").current_dir(&temp_dir).output().unwrap();

    // Should fail because no task name provided
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required") || stderr.contains("TASK_NAME"));
}
