use crate::config::WorkspaceConfig;
use crate::modules::git;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn initialize(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_id = Uuid::now_v7();
    println!("Initializing workspace: {}", name);
    
    // Get repository name from git remote
    let repo_name = git::get_repository_name();
    
    // Create local workspace directory
    let workspace_dir = format!("./.claudectl/workspaces/{}", workspace_id);
    fs::create_dir_all(&workspace_dir)?;
    
    // Create worktree directory
    let home_dir = std::env::var("HOME")?;
    let worktree_path = format!("{home_dir}/.claudectl/projects/{repo_name}/{workspace_id}");
    let worktree_parent = format!("{home_dir}/.claudectl/projects/{repo_name}");
    
    fs::create_dir_all(&worktree_parent)?;
    
    // Get current branch
    let current_branch = git::get_current_branch()?;
    
    // Create a new branch for the worktree
    let worktree_branch = format!("claudectl/{}", workspace_id);
    
    // Create git worktree with new branch
    println!("Creating git worktree at: {}", worktree_path);
    
    if let Err(e) = git::create_worktree(&worktree_path, &worktree_branch, &current_branch) {
        // Clean up workspace directory on failure
        let _ = fs::remove_dir_all(&workspace_dir);
        return Err(e.into());
    }
    
    // Create and save config
    let config = WorkspaceConfig::new(workspace_id, name.to_string(), worktree_path.clone());
    config.save(&workspace_dir)?;
    
    println!("Workspace '{}' (ID: {}) initialized successfully", name, workspace_id);
    println!("Git worktree created at: {}", worktree_path);
    
    Ok(())
}

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    let workspaces_dir = "./.claudectl/workspaces";
    
    if !Path::new(workspaces_dir).exists() {
        println!("No workspaces found. Create one with: claudectl workspace new [name]");
        return Ok(());
    }
    
    let entries = fs::read_dir(workspaces_dir)?;
    let mut workspaces = Vec::new();
    
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let config_path = entry.path().join("config.json");
                    if config_path.exists() {
                        if let Ok(config) = WorkspaceConfig::load(&config_path) {
                            workspaces.push(config);
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
        }
    }
    
    Ok(())
}