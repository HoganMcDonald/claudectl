use crate::config::WorkspaceConfig;
use crate::error::{ClaudeCtlError, Result};
use crate::modules::git;
use std::fs;
use std::path::Path;
use uuid::Uuid;

/// Validates workspace name for security and usability
fn validate_workspace_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(ClaudeCtlError::Validation("Workspace name cannot be empty".to_string()));
    }
    if name.len() > 100 {
        return Err(ClaudeCtlError::Validation("Workspace name too long (max 100 characters)".to_string()));
    }
    if name.contains('/') || name.contains('\\') {
        return Err(ClaudeCtlError::Validation("Workspace name cannot contain path separators".to_string()));
    }
    if name.contains('\0') {
        return Err(ClaudeCtlError::Validation("Workspace name cannot contain null characters".to_string()));
    }
    Ok(())
}

/// Cleanup handler that removes directories on drop if cleanup is enabled
struct CleanupGuard {
    paths: Vec<String>,
    should_cleanup: bool,
}

impl CleanupGuard {
    fn new() -> Self {
        Self {
            paths: Vec::new(),
            should_cleanup: true,
        }
    }
    
    fn add_path(&mut self, path: String) {
        self.paths.push(path);
    }
    
    fn success(mut self) {
        self.should_cleanup = false;
    }
}

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        if self.should_cleanup {
            for path in &self.paths {
                let _ = fs::remove_dir_all(path);
            }
        }
    }
}

/// Initializes a new workspace with the given name
/// 
/// This function creates:
/// - A local workspace directory at `./.claudectl/workspaces/{uuid}`
/// - A git worktree at `~/.claudectl/projects/{repo}/{uuid}`
/// - A new git branch named `claudectl/{uuid}`
/// 
/// # Arguments
/// * `name` - The human-readable name for the workspace
/// 
/// # Errors
/// Returns an error if:
/// - Workspace name validation fails
/// - Not in a git repository
/// - Git worktree creation fails
/// - Filesystem operations fail
/// - Configuration cannot be saved
pub fn initialize(name: &str) -> Result<()> {
    validate_workspace_name(name)?;
    
    let workspace_id = Uuid::now_v7();
    println!("Initializing workspace: {name}");
    
    let mut cleanup_guard = CleanupGuard::new();
    
    // Get repository name from git remote
    let repo_name = git::get_repository_name();
    
    // Create local workspace directory
    let workspace_dir = format!("./.claudectl/workspaces/{workspace_id}");
    fs::create_dir_all(&workspace_dir)?;
    cleanup_guard.add_path(workspace_dir.clone());
    
    // Create worktree directory - use better home directory detection
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| ClaudeCtlError::Environment("Could not determine home directory (HOME or USERPROFILE not set)".to_string()))?;
    
    let worktree_path = format!("{home_dir}/.claudectl/projects/{repo_name}/{workspace_id}");
    let worktree_parent = format!("{home_dir}/.claudectl/projects/{repo_name}");
    
    fs::create_dir_all(&worktree_parent)?;
    cleanup_guard.add_path(worktree_path.clone());
    
    // Get current branch
    let current_branch = git::get_current_branch()?;
    
    // Create a new branch for the worktree
    let worktree_branch = format!("claudectl/{workspace_id}");
    
    // Create git worktree with new branch
    println!("Creating git worktree at: {worktree_path}");
    git::create_worktree(&worktree_path, &worktree_branch, &current_branch)?;
    
    // Create and save config
    let config = WorkspaceConfig::new(workspace_id, name.to_string(), worktree_path.clone());
    config.save(&workspace_dir)?;
    
    // If we reach here, disable cleanup
    cleanup_guard.success();
    
    println!("Workspace '{name}' (ID: {workspace_id}) initialized successfully");
    println!("Git worktree created at: {worktree_path}");
    
    Ok(())
}

/// Lists all workspaces in the current repository
/// 
/// Scans the `./.claudectl/workspaces` directory for workspace configurations
/// and displays them sorted by creation time. Shows workspace name, ID, 
/// creation time, and worktree path for each workspace.
/// 
/// # Errors
/// Returns an error if the workspaces directory cannot be read
pub fn list() -> Result<()> {
    let workspaces_dir = "./.claudectl/workspaces";
    
    if !Path::new(workspaces_dir).exists() {
        println!("No workspaces found. Create one with: claudectl workspace new [name]");
        return Ok(());
    }
    
    let entries = fs::read_dir(workspaces_dir)?;
    let mut workspaces = Vec::new();
    
    for entry in entries.flatten() {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_dir() {
                let config_path = entry.path().join("config.json");
                if config_path.exists() {
                    match WorkspaceConfig::load(&config_path) {
                        Ok(config) => workspaces.push(config),
                        Err(e) => {
                            eprintln!("Warning: Failed to load workspace config at {}: {e}", config_path.display());
                        }
                    }
                }
            }
        }
    }
    
    if workspaces.is_empty() {
        println!("No workspaces found. Create one with: claudectl workspace new [name]");
    } else {
        workspaces.sort_by(|a, b| a.created.cmp(&b.created));
        
        println!("Workspaces:");
        for workspace in workspaces {
            println!("  - {} ({})", workspace.name, workspace.id);
            println!("    Created: {}", workspace.created.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("    Worktree: {}", workspace.worktree_path);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_validate_workspace_name_valid() {
        assert!(validate_workspace_name("valid-name").is_ok());
        assert!(validate_workspace_name("ValidName123").is_ok());
        assert!(validate_workspace_name("My Project").is_ok());
    }

    #[test]
    fn test_validate_workspace_name_empty() {
        let result = validate_workspace_name("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ClaudeCtlError::Validation(_)));
    }

    #[test]
    fn test_validate_workspace_name_too_long() {
        let long_name = "a".repeat(101);
        let result = validate_workspace_name(&long_name);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ClaudeCtlError::Validation(_)));
    }

    #[test]
    fn test_validate_workspace_name_path_separators() {
        assert!(validate_workspace_name("name/with/slash").is_err());
        assert!(validate_workspace_name("name\\with\\backslash").is_err());
    }

    #[test]
    fn test_validate_workspace_name_null_character() {
        let name_with_null = "name\0with\0null";
        let result = validate_workspace_name(name_with_null);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ClaudeCtlError::Validation(_)));
    }

    #[test]
    fn test_cleanup_guard_success() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path().join("test");
        fs::create_dir_all(&test_dir).unwrap();
        
        {
            let mut guard = CleanupGuard::new();
            guard.add_path(test_dir.to_string_lossy().to_string());
            guard.success(); // Should prevent cleanup
        }
        
        // Directory should still exist
        assert!(test_dir.exists());
    }

    #[test]
    fn test_cleanup_guard_failure() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path().join("test");
        fs::create_dir_all(&test_dir).unwrap();
        
        {
            let mut guard = CleanupGuard::new();
            guard.add_path(test_dir.to_string_lossy().to_string());
            // Don't call success() - should trigger cleanup
        }
        
        // Directory should be cleaned up
        assert!(!test_dir.exists());
    }
}