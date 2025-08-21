use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_init_in_git_repo() {
    let temp_dir = TempDir::new().unwrap();

    // Set up git repo
    fs::create_dir(temp_dir.path().join(".git")).unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd.arg("init").current_dir(&temp_dir).output().unwrap();

    // The command might fail due to claude not being installed, which is expected
    if output.status.success() {
        // If it succeeds, verify expected output
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Initializing project"));
    } else {
        // If it fails, verify it's due to expected dependency issues
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Could fail due to missing claude or other dependency issues
        assert!(
            stderr.contains("Claude is not installed")
                || stderr.contains("Git Error")
                || stderr.contains("Claude Error")
        );
    }
}

#[test]
fn test_init_not_in_git_repo() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd
        .arg("init")
        .current_dir(&temp_dir) // Run command in temp dir instead of changing global cwd
        .output()
        .unwrap();

    // Debug output if test fails
    if output.status.success() {
        println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        println!("Temp dir: {:?}", temp_dir.path());
    }

    // Should fail because not in git repo
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Git Error") || stderr.contains("not a git repository"));
}

#[test]
fn test_init_shows_help_message() {
    let temp_dir = TempDir::new().unwrap();

    // Set up git repo to pass first check
    fs::create_dir(temp_dir.path().join(".git")).unwrap();

    let mut cmd = Command::cargo_bin("claudectl").unwrap();
    let output = cmd.arg("init").current_dir(&temp_dir).output().unwrap();

    // Check that initialization message is shown
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Initializing project"));
    assert!(stdout.contains("Verifying Dependencies"));
}
