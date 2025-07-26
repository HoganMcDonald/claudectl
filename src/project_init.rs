use serde::{Deserialize, Serialize};
use std::{fs, io};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectInitError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Project directory already exists")]
    AlreadyExists,
    #[error("Invalid project name: {0}")]
    InvalidName(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ProjectConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            created_at: chrono::Utc::now(),
        }
    }
}

pub struct ProjectInitializer;

impl ProjectInitializer {
    /// Check if a .claudectl directory exists in the current directory
    pub fn has_claudectl_dir<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().join(".claudectl").exists()
    }

    /// Get the current directory name as the default project name
    pub fn get_default_project_name<P: AsRef<Path>>(path: P) -> String {
        path.as_ref()
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("untitled-project")
            .to_string()
    }

    /// Validate project name
    pub fn validate_project_name(name: &str) -> Result<(), ProjectInitError> {
        if name.trim().is_empty() {
            return Err(ProjectInitError::InvalidName("Name cannot be empty".to_string()));
        }

        if name.len() > 100 {
            return Err(ProjectInitError::InvalidName("Name too long (max 100 characters)".to_string()));
        }

        // Check for invalid characters that might cause issues
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(ProjectInitError::InvalidName("Name contains invalid characters".to_string()));
        }

        Ok(())
    }

    /// Initialize a new project in the given directory
    pub fn initialize_project<P: AsRef<Path>>(
        path: P,
        project_name: String,
    ) -> Result<(), ProjectInitError> {
        let path = path.as_ref();
        let claudectl_dir = path.join(".claudectl");

        // Check if .claudectl directory already exists
        if claudectl_dir.exists() {
            return Err(ProjectInitError::AlreadyExists);
        }

        // Validate project name
        Self::validate_project_name(&project_name)?;

        // Create .claudectl directory
        fs::create_dir_all(&claudectl_dir)?;

        // Create project.json file
        let project_config = ProjectConfig::new(project_name);
        let project_json = serde_json::to_string_pretty(&project_config)?;
        
        let project_file = claudectl_dir.join("project.json");
        fs::write(project_file, project_json)?;

        Ok(())
    }

    /// Load project configuration from the .claudectl directory
    pub fn load_project_config<P: AsRef<Path>>(
        path: P,
    ) -> Result<ProjectConfig, ProjectInitError> {
        let project_file = path.as_ref().join(".claudectl").join("project.json");
        
        if !project_file.exists() {
            return Err(ProjectInitError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                "project.json not found",
            )));
        }

        let contents = fs::read_to_string(project_file)?;
        let config: ProjectConfig = serde_json::from_str(&contents)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_has_claudectl_dir() {
        let temp_dir = TempDir::new().unwrap();
        
        // Initially no .claudectl directory
        assert!(!ProjectInitializer::has_claudectl_dir(temp_dir.path()));
        
        // Create .claudectl directory
        fs::create_dir(temp_dir.path().join(".claudectl")).unwrap();
        assert!(ProjectInitializer::has_claudectl_dir(temp_dir.path()));
    }

    #[test]
    fn test_get_default_project_name() {
        let temp_dir = TempDir::new().unwrap();
        let name = ProjectInitializer::get_default_project_name(temp_dir.path());
        assert!(!name.is_empty());
        assert_ne!(name, "untitled-project"); // TempDir creates a unique name
    }

    #[test]
    fn test_validate_project_name() {
        // Valid names
        assert!(ProjectInitializer::validate_project_name("my-project").is_ok());
        assert!(ProjectInitializer::validate_project_name("Project123").is_ok());
        assert!(ProjectInitializer::validate_project_name("my_project").is_ok());

        // Invalid names
        assert!(ProjectInitializer::validate_project_name("").is_err());
        assert!(ProjectInitializer::validate_project_name("   ").is_err());
        assert!(ProjectInitializer::validate_project_name("project/with/slash").is_err());
        assert!(ProjectInitializer::validate_project_name("project:with:colon").is_err());
        
        // Too long name
        let long_name = "a".repeat(101);
        assert!(ProjectInitializer::validate_project_name(&long_name).is_err());
    }

    #[test]
    fn test_initialize_project() {
        let temp_dir = TempDir::new().unwrap();
        let project_name = "test-project".to_string();

        // Initialize project
        ProjectInitializer::initialize_project(temp_dir.path(), project_name.clone()).unwrap();

        // Check that .claudectl directory was created
        let claudectl_dir = temp_dir.path().join(".claudectl");
        assert!(claudectl_dir.exists());
        assert!(claudectl_dir.is_dir());

        // Check that project.json was created
        let project_file = claudectl_dir.join("project.json");
        assert!(project_file.exists());

        // Load and verify project config
        let config = ProjectInitializer::load_project_config(temp_dir.path()).unwrap();
        assert_eq!(config.name, project_name);
    }

    #[test]
    fn test_initialize_project_already_exists() {
        let temp_dir = TempDir::new().unwrap();
        let project_name = "test-project".to_string();

        // Initialize project first time
        ProjectInitializer::initialize_project(temp_dir.path(), project_name.clone()).unwrap();

        // Try to initialize again - should fail
        let result = ProjectInitializer::initialize_project(temp_dir.path(), project_name);
        assert!(matches!(result, Err(ProjectInitError::AlreadyExists)));
    }

    #[test]
    fn test_load_project_config() {
        let temp_dir = TempDir::new().unwrap();
        let project_name = "test-project".to_string();

        // Initialize project
        ProjectInitializer::initialize_project(temp_dir.path(), project_name.clone()).unwrap();

        // Load config
        let config = ProjectInitializer::load_project_config(temp_dir.path()).unwrap();
        assert_eq!(config.name, project_name);
        assert!(config.created_at <= chrono::Utc::now());
    }

    #[test]
    fn test_load_project_config_not_found() {
        let temp_dir = TempDir::new().unwrap();
        
        let result = ProjectInitializer::load_project_config(temp_dir.path());
        assert!(result.is_err());
    }
}