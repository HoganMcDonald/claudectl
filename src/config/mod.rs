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

    pub fn save(&self, workspace_dir: &str) -> Result<(), String> {
        let config_content = serde_json::to_string_pretty(&self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        let config_path = format!("{}/config.json", workspace_dir);
        fs::write(&config_path, config_content)
            .map_err(|e| format!("Error creating workspace config: {}", e))?;
        
        Ok(())
    }

    pub fn load(config_path: &Path) -> Result<Self, String> {
        let config_content = fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        
        serde_json::from_str(&config_content)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }
}