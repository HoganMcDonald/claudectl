use crate::error::{ClaudeCtlError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
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