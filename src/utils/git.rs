use crate::utils::errors::{GitAction, GitError};
use std::process::Command;
use tracing::{debug, info, instrument, warn};

type GitResult<T> = Result<T, GitError>;

#[instrument]
pub fn is_git_repository() -> GitResult<bool> {
    debug!("Checking if current directory is a git repository");
    if std::path::Path::new(".git").exists() {
        info!("Found .git directory");
        Ok(true)
    } else {
        info!("No .git directory found");
        Ok(false)
    }
}

#[instrument]
pub fn fetch_origin() -> GitResult<()> {
    info!("Fetching latest changes from origin");
    let output = Command::new("git")
        .args(["fetch", "origin"])
        .output()
        .map_err(|e| {
            GitError::new(
                &format!("Failed to execute git fetch command: {e}"),
                GitAction::Fetch,
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("Git fetch failed with stderr: {}", stderr);
        return Err(GitError::new(
            &format!("Git fetch failed: {stderr}"),
            GitAction::Fetch,
        ));
    }

    info!("Successfully fetched from origin");
    Ok(())
}

pub struct Worktree {
    pub path: String,
    pub commit: String,
    pub branch: Option<String>,
}

pub fn worktree_list() -> GitResult<Vec<Worktree>> {
    let output = Command::new("git")
        .args(["worktree", "list"])
        .output()
        .map_err(|e| {
            GitError::new(
                &format!("Failed to execute git worktree list command: {e}"),
                GitAction::WorktreeList,
            )
        })?;

    let stdout = String::from_utf8(output.stdout).map_err(|e| {
        GitError::new(
            &format!("Failed to parse output of git worktree list command: {e}"),
            GitAction::WorktreeList,
        )
    })?;

    let worktrees: Vec<Worktree> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let path = parts[0].to_string();
                let commit = parts[1].to_string();
                let branch =
                    if parts.len() > 2 && parts[2].starts_with('[') && parts[2].ends_with(']') {
                        Some(parts[2][1..parts[2].len() - 1].to_string()) // Remove brackets
                    } else {
                        None
                    };

                Some(Worktree {
                    path,
                    commit,
                    branch,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(worktrees)
}

pub fn worktree_exists(worktree_path: &str) -> GitResult<bool> {
    let output = Command::new("git")
        .args(["worktree", "list"])
        .output()
        .map_err(|e| {
            GitError::new(
                &format!("Failed to execute git worktree list command: {e}"),
                GitAction::WorktreeList,
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GitError::new(
            &format!("Git worktree list failed: {stderr}"),
            GitAction::WorktreeList,
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().any(|line| line.contains(worktree_path)))
}

#[instrument(fields(branch_name = %branch_name, worktree_path = %worktree_path))]
pub fn create_worktree(branch_name: &str, worktree_path: &str) -> GitResult<()> {
    info!(
        "Creating worktree '{}' at path: {}",
        branch_name, worktree_path
    );
    let output = Command::new("git")
        .args([
            "worktree",
            "add",
            "-b",
            branch_name,
            worktree_path,
            "origin/main",
        ])
        .output()
        .map_err(|e| {
            GitError::new(
                &format!("Failed to execute git worktree add command: {e}"),
                GitAction::WorktreeAdd,
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("Git worktree add failed with stderr: {}", stderr);
        return Err(GitError::new(
            &format!("Git worktree add failed: {stderr}"),
            GitAction::WorktreeAdd,
        ));
    }

    info!(
        "Successfully created worktree '{}' at: {}",
        branch_name, worktree_path
    );
    Ok(())
}

#[instrument(fields(worktree_path = %worktree_path))]
pub fn remove_worktree(worktree_path: &str) -> GitResult<()> {
    info!("Removing worktree at path: {}", worktree_path);
    let output = Command::new("git")
        .args(["worktree", "remove", worktree_path, "--force"])
        .output()
        .map_err(|e| {
            GitError::new(
                &format!("Failed to execute git worktree remove command: {e}"),
                GitAction::WorktreeRemove,
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("Git worktree remove failed with stderr: {}", stderr);
        return Err(GitError::new(
            &format!("Git worktree remove failed: {stderr}"),
            GitAction::WorktreeRemove,
        ));
    }

    info!("Successfully removed worktree at: {}", worktree_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_git_repository_when_git_exists() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory and create .git
        std::env::set_current_dir(&temp_dir).unwrap();
        fs::create_dir_all(".git").unwrap();

        let result = is_git_repository();

        // Restore original directory (ignore errors if original dir was temp)
        let _ = std::env::set_current_dir(&original_dir);

        match result {
            Ok(value) => assert!(value),
            Err(e) => panic!("Expected Ok but got Err: {e:?}"),
        }
    }

    #[test]
    fn test_is_git_repository_when_git_missing() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory without .git
        std::env::set_current_dir(&temp_dir).unwrap();

        let result = is_git_repository();

        // Restore original directory (ignore errors if original dir was temp)
        let _ = std::env::set_current_dir(&original_dir);

        match result {
            Ok(value) => assert!(!value),
            Err(e) => panic!("Expected Ok(false) but got Err: {e:?}"),
        }
    }

    #[test]
    fn test_worktree_exists_returns_false_for_nonexistent_path() {
        // This test will only work if we're in a git repository
        // For now, we'll just test that the function doesn't panic
        let result = worktree_exists("/definitely/does/not/exist");
        // Should either return Ok(false) or Err (if git command fails)
        match result {
            Ok(exists) => assert!(!exists),
            Err(_) => {
                // Expected if not in a git repo or git not available
            }
        }
    }
}
