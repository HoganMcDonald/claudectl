use crate::error::{ClaudeCtlError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub id: Uuid,
    pub name: String,
    pub created: DateTime<Utc>,
    pub version: String,
    pub worktree_path: String,
}

impl WorkspaceConfig {
    pub fn new(id: Uuid, name: String, worktree_path: String) -> Self {
        Self {
            id,
            name,
            created: Utc::now(),
            version: "1.0".to_string(),
            worktree_path,
        }
    }

    pub fn save(&self, workspace_dir: &str) -> Result<()> {
        let config_content = serde_json::to_string_pretty(&self)?;
        let config_path = format!("{workspace_dir}/config.json");
        fs::write(&config_path, config_content)?;
        Ok(())
    }

    pub fn load(config_path: &Path) -> Result<Self> {
        let config_content = fs::read_to_string(config_path)
            .map_err(|e| ClaudeCtlError::Config(format!("Failed to read config at {}: {e}", config_path.display())))?;
        
        serde_json::from_str(&config_content)
            .map_err(|e| ClaudeCtlError::Config(format!("Failed to parse config at {}: {e}", config_path.display())).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_workspace_config_creation() {
        let uuid = Uuid::now_v7();
        let config = WorkspaceConfig::new(
            uuid,
            "Test Workspace".to_string(),
            "/path/to/worktree".to_string(),
        );

        assert_eq!(config.id, uuid);
        assert_eq!(config.name, "Test Workspace");
        assert_eq!(config.version, "1.0");
        assert_eq!(config.worktree_path, "/path/to/worktree");
        assert!(config.created <= Utc::now());
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_dir = temp_dir.path().to_str().unwrap();
        
        let uuid = Uuid::now_v7();
        let original_config = WorkspaceConfig::new(
            uuid,
            "Test Workspace".to_string(),
            "/path/to/worktree".to_string(),
        );

        // Save config
        original_config.save(workspace_dir).unwrap();

        // Load config
        let config_path = temp_dir.path().join("config.json");
        let loaded_config = WorkspaceConfig::load(&config_path).unwrap();

        assert_eq!(loaded_config.id, original_config.id);
        assert_eq!(loaded_config.name, original_config.name);
        assert_eq!(loaded_config.version, original_config.version);
        assert_eq!(loaded_config.worktree_path, original_config.worktree_path);
    }

    #[test]
    fn test_config_load_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent.json");
        
        let result = WorkspaceConfig::load(&config_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ClaudeCtlError::Config(_)));
    }

    #[test]
    fn test_config_load_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid.json");
        
        fs::write(&config_path, "invalid json content").unwrap();
        
        let result = WorkspaceConfig::load(&config_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ClaudeCtlError::Config(_)));
    }

    #[test]
    fn test_config_save_invalid_directory() {
        let uuid = Uuid::now_v7();
        let config = WorkspaceConfig::new(
            uuid,
            "Test".to_string(),
            "/path/to/worktree".to_string(),
        );

        // Try to save to an invalid directory
        let result = config.save("/invalid/directory/that/does/not/exist");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ClaudeCtlError::Filesystem(_)));
    }
}