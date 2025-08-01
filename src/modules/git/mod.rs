use crate::error::{ClaudeCtlError, Result};
use std::process::Command;

pub fn get_repository_name() -> String {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            extract_repo_name_from_url(&url)
        }
        _ => {
            // If no remote, use the current directory name
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            current_dir.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .to_string()
        }
    }
}

pub fn validate_git_repository() -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map_err(|e| ClaudeCtlError::Git(format!("Failed to check git repository: {e}")))?;
    
    if !output.status.success() {
        return Err(ClaudeCtlError::Git("Not inside a git repository".to_string()));
    }
    Ok(())
}

pub fn get_current_branch() -> Result<String> {
    validate_git_repository()?;
    
    // Check for detached HEAD first
    let output = Command::new("git")
        .args(["symbolic-ref", "-q", "HEAD"])
        .output()
        .map_err(|e| ClaudeCtlError::Git(format!("Failed to get branch: {e}")))?;
        
    if !output.status.success() {
        return Err(ClaudeCtlError::Git("In detached HEAD state. Please checkout a branch first.".to_string()));
    }
    
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|e| ClaudeCtlError::Git(format!("Failed to execute git rev-parse: {e}")))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(ClaudeCtlError::Git(format!("Error getting current branch: {}", String::from_utf8_lossy(&output.stderr))))
    }
}

pub fn create_worktree(worktree_path: &str, branch_name: &str, base_branch: &str) -> Result<()> {
    validate_git_repository()?;
    
    let output = Command::new("git")
        .args(["worktree", "add", "-b", branch_name, worktree_path, base_branch])
        .output()
        .map_err(|e| ClaudeCtlError::Git(format!("Failed to execute git worktree add: {e}")))?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(ClaudeCtlError::Git(format!("Error creating git worktree: {}", String::from_utf8_lossy(&output.stderr))))
    }
}

fn extract_repo_name_from_url(url: &str) -> String {
    let url = url.trim_end_matches(".git");
    
    if url.contains("github.com") || url.contains("gitlab.com") || url.contains("bitbucket.org") {
        // SSH format: git@github.com:user/repo
        // HTTPS format: https://github.com/user/repo
        url.split('/').next_back()
            .or_else(|| url.split(':').next_back()?.split('/').next_back())
            .unwrap_or("unknown")
            .to_string()
    } else {
        // For other URLs, just take the last path component
        url.split('/').next_back().unwrap_or("unknown").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_repo_name_from_github_https() {
        let url = "https://github.com/user/repo.git";
        assert_eq!(extract_repo_name_from_url(url), "repo");
    }

    #[test]
    fn test_extract_repo_name_from_github_https_no_git() {
        let url = "https://github.com/user/repo";
        assert_eq!(extract_repo_name_from_url(url), "repo");
    }

    #[test]
    fn test_extract_repo_name_from_github_ssh() {
        let url = "git@github.com:user/repo.git";
        assert_eq!(extract_repo_name_from_url(url), "repo");
    }

    #[test]
    fn test_extract_repo_name_from_gitlab() {
        let url = "https://gitlab.com/user/project.git";
        assert_eq!(extract_repo_name_from_url(url), "project");
    }

    #[test]
    fn test_extract_repo_name_from_bitbucket() {
        let url = "https://bitbucket.org/user/repo.git";
        assert_eq!(extract_repo_name_from_url(url), "repo");
    }

    #[test]
    fn test_extract_repo_name_from_custom_url() {
        let url = "https://git.example.com/user/custom-repo";
        assert_eq!(extract_repo_name_from_url(url), "custom-repo");
    }

    #[test]
    fn test_extract_repo_name_from_invalid_url() {
        let url = "not-a-url";
        assert_eq!(extract_repo_name_from_url(url), "not-a-url");
    }

    #[test]
    fn test_extract_repo_name_empty_url() {
        let url = "";
        assert_eq!(extract_repo_name_from_url(url), "");
    }
}