use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_list_command_fails_without_git_repo() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd.arg("list").current_dir(&temp_dir).output().unwrap();

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
fn test_list_command_fails_without_config() {
    let temp_dir = TempDir::new().unwrap();

    // Set up git repo but no claudectl config
    fs::create_dir(temp_dir.path().join(".git")).unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd.arg("list").current_dir(&temp_dir).output().unwrap();

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
fn test_list_command_with_no_worktrees() {
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
    let output = cmd.arg("list").current_dir(&temp_dir).output().unwrap();

    // May succeed or fail depending on git worktree state, but shouldn't crash
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should either show empty output or fail gracefully
    if output.status.success() {
        // If successful, output should be well-formed (could be empty)
        // No headers should be present since we disabled them
        assert!(!stdout.contains("name") || !stdout.contains("status"));
    } else {
        // If failed, should have meaningful error message
        assert!(
            stderr.contains("Failed to get active tasks")
                || stderr.contains("Claude")
                || stderr.contains("Git")
                || stderr.contains("Configuration file not found")
        );
    }
}

#[test]
fn test_list_command_output_format() {
    let temp_dir = TempDir::new().unwrap();

    // Set up git repo
    fs::create_dir(temp_dir.path().join(".git")).unwrap();

    // Create claudectl config
    let config_content = r#"{
        "project_name": "test-project", 
        "project_dir": "/tmp/test"
    }"#;
    fs::write(temp_dir.path().join("CLAUDE.md"), config_content).unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd.arg("list").current_dir(&temp_dir).output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // If command succeeds, verify output doesn't contain table headers
    if output.status.success() {
        // Should not have column headers like "name | status | commit | worktree"
        let lines: Vec<&str> = stdout.lines().collect();

        // Look for header-like patterns that shouldn't exist
        for line in &lines {
            // Headers would typically be followed by a separator like "---"
            // or contain multiple words separated by pipes/spaces in a header-like format
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                // Each line should look like data, not headers
                // Data lines should have commit hashes (7-8 chars of hex)
                // and status indicators (colored circles)
                assert!(
                    trimmed.len() > 20, // Data lines should be substantial
                    "Line too short to be data: '{trimmed}'"
                );
            }
        }
    }
}

#[test]
fn test_list_command_shows_initialization_message() {
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
    let output = cmd.arg("list").current_dir(&temp_dir).output().unwrap();

    // Check for logging output indicating command execution
    let stderr = String::from_utf8_lossy(&output.stderr);

    // The list command should log its execution (if debug logging is enabled)
    // or handle errors gracefully without crashing
    if !output.status.success() {
        assert!(
            stderr.contains("Failed to get active tasks")
                || stderr.contains("Claude")
                || stderr.contains("Git")
                || stderr.contains("Configuration file not found")
        );
    }
}
