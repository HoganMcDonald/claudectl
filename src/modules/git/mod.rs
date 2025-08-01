use std::process::Command;

pub fn get_repository_name() -> String {
    let output = Command::new("git")
        .args(&["remote", "get-url", "origin"])
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

pub fn get_current_branch() -> Result<String, String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|e| format!("Failed to execute git rev-parse: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(format!("Error getting current branch: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

pub fn create_worktree(worktree_path: &str, branch_name: &str, base_branch: &str) -> Result<(), String> {
    let output = Command::new("git")
        .args(&["worktree", "add", "-b", branch_name, worktree_path, base_branch])
        .output()
        .map_err(|e| format!("Failed to execute git worktree add: {}", e))?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Error creating git worktree: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

fn extract_repo_name_from_url(url: &str) -> String {
    let url = url.trim_end_matches(".git");
    
    if url.contains("github.com") || url.contains("gitlab.com") || url.contains("bitbucket.org") {
        // SSH format: git@github.com:user/repo
        // HTTPS format: https://github.com/user/repo
        url.split('/').last()
            .or_else(|| url.split(':').last()?.split('/').last())
            .unwrap_or("unknown")
            .to_string()
    } else {
        // For other URLs, just take the last path component
        url.split('/').last().unwrap_or("unknown").to_string()
    }
}