use crate::data::{AppData, Project, SessionData};
use dirs::config_dir;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Configuration directory not found")]
    ConfigDirNotFound,
    #[error("Data corruption detected: {0}")]
    DataCorruption(String),
}

pub trait Storage {
    fn load(&self) -> Result<AppData, StorageError>;
    fn save(&self, data: &AppData) -> Result<(), StorageError>;
    fn validate_projects(&self, projects: &mut Vec<Project>) -> Vec<String>;
    fn get_config_path(&self) -> &Path;
}

pub trait SessionStorage {
    fn load_sessions(&self) -> Result<SessionData, StorageError>;
    fn save_sessions(&self, data: &SessionData) -> Result<(), StorageError>;
    fn get_config_path(&self) -> &Path;
}

pub struct JsonStorage {
    config_path: PathBuf,
    data_file: PathBuf,
}

impl JsonStorage {
    pub fn new() -> Result<Self, StorageError> {
        // Use project-specific .claudectl directory if it exists
        let current_dir = std::env::current_dir().map_err(StorageError::Io)?;
        let project_config_path = current_dir.join(".claudectl");
        
        let config_path = if project_config_path.exists() {
            project_config_path
        } else {
            // Fall back to global config directory if no project is initialized
            Self::get_global_config_directory()?
        };
        
        let data_file = config_path.join("sessions.json");

        Ok(Self {
            config_path,
            data_file,
        })
    }

    pub fn with_custom_path(config_path: PathBuf) -> Self {
        let data_file = config_path.join("sessions.json");
        Self {
            config_path,
            data_file,
        }
    }

    pub fn for_project(project_path: PathBuf) -> Result<Self, StorageError> {
        let config_path = project_path.join(".claudectl");
        
        // Create .claudectl directory if it doesn't exist
        if !config_path.exists() {
            return Err(StorageError::ConfigDirNotFound);
        }
        
        let data_file = config_path.join("sessions.json");

        Ok(Self {
            config_path,
            data_file,
        })
    }

    fn get_global_config_directory() -> Result<PathBuf, StorageError> {
        let config_dir = config_dir().ok_or(StorageError::ConfigDirNotFound)?;
        let app_config_dir = config_dir.join("claudectl");

        // Create directory if it doesn't exist
        if !app_config_dir.exists() {
            fs::create_dir_all(&app_config_dir)?;
        }

        Ok(app_config_dir)
    }

    fn create_backup(&self, _data: &AppData) -> Result<(), StorageError> {
        if self.data_file.exists() {
            let backup_file = self.config_path.join("sessions.json.backup");
            fs::copy(&self.data_file, backup_file)?;
        }
        Ok(())
    }

    fn atomic_write(&self, data: &AppData) -> Result<(), StorageError> {
        let temp_file = self.config_path.join("sessions.json.tmp");
        let json = serde_json::to_string_pretty(data)?;

        // Write to temporary file first
        fs::write(&temp_file, json)?;

        // Atomic move to final location
        fs::rename(temp_file, &self.data_file)?;

        Ok(())
    }

    fn validate_data_integrity(&self, data: &AppData) -> Result<(), StorageError> {
        // Basic validation checks
        for project in &data.projects {
            if project.id.is_empty() {
                return Err(StorageError::DataCorruption(
                    "Project with empty ID found".to_string(),
                ));
            }
            if project.name.is_empty() {
                return Err(StorageError::DataCorruption(format!(
                    "Project {} has empty name",
                    project.id
                )));
            }
        }

        for session in &data.sessions {
            if session.id.is_empty() {
                return Err(StorageError::DataCorruption(
                    "Session with empty ID found".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl Storage for JsonStorage {
    fn load(&self) -> Result<AppData, StorageError> {
        if !self.data_file.exists() {
            // Return default data if file doesn't exist
            return Ok(AppData::new());
        }

        let contents = fs::read_to_string(&self.data_file)?;

        if contents.trim().is_empty() {
            // Handle empty file
            return Ok(AppData::new());
        }

        let mut data: AppData = serde_json::from_str(&contents)?;

        // Validate data integrity
        self.validate_data_integrity(&data)?;

        // Clean up missing projects automatically on load
        let removed_ids = self.validate_projects(&mut data.projects);
        if !removed_ids.is_empty() {
            // Save cleaned data back
            self.save(&data)?;
        }

        Ok(data)
    }

    fn save(&self, data: &AppData) -> Result<(), StorageError> {
        // Validate data before saving
        self.validate_data_integrity(data)?;

        // Create backup of existing data
        self.create_backup(data)?;

        // Perform atomic write
        self.atomic_write(data)?;

        Ok(())
    }

    fn validate_projects(&self, projects: &mut Vec<Project>) -> Vec<String> {
        let mut removed_ids = Vec::new();
        projects.retain(|project| {
            if project.exists() {
                true
            } else {
                removed_ids.push(project.id.clone());
                false
            }
        });
        removed_ids
    }

    fn get_config_path(&self) -> &Path {
        &self.config_path
    }
}

impl SessionStorage for JsonStorage {
    fn load_sessions(&self) -> Result<SessionData, StorageError> {
        if !self.data_file.exists() {
            // Return default data if file doesn't exist
            return Ok(SessionData::new());
        }

        let contents = fs::read_to_string(&self.data_file)?;

        if contents.trim().is_empty() {
            // Handle empty file
            return Ok(SessionData::new());
        }

        // Try to deserialize as SessionData first
        match serde_json::from_str::<SessionData>(&contents) {
            Ok(data) => {
                // Validate data integrity
                self.validate_session_data_integrity(&data)?;
                Ok(data)
            }
            Err(e) => {
                // If that fails, check if it's old AppData format and migrate
                if let Ok(old_data) = serde_json::from_str::<AppData>(&contents) {
                    // Convert old AppData to SessionData
                    let session_data = SessionData {
                        sessions: old_data.sessions,
                        stats: crate::data::SessionStats {
                            active_sessions: old_data.stats.active_sessions,
                            total_runtime: old_data.stats.total_runtime,
                        },
                    };
                    
                    // Save the converted data back
                    self.save_sessions(&session_data)?;
                    Ok(session_data)
                } else {
                    // If all parsing attempts fail, create backup and return default
                    if self.create_corrupted_backup().is_err() {
                        // If backup creation fails, just log and continue
                        eprintln!("Warning: Failed to create backup of corrupted session data");
                    }
                    Ok(SessionData::new())
                }
            }
        }
    }

    fn save_sessions(&self, data: &SessionData) -> Result<(), StorageError> {
        // Validate data before saving
        self.validate_session_data_integrity(data)?;

        // Create backup of existing data
        self.create_session_backup(data)?;

        // Perform atomic write
        self.atomic_write_sessions(data)?;

        Ok(())
    }

    fn get_config_path(&self) -> &Path {
        &self.config_path
    }
}

impl JsonStorage {
    fn create_session_backup(&self, _data: &SessionData) -> Result<(), StorageError> {
        if self.data_file.exists() {
            let backup_file = self.config_path.join("sessions.json.backup");
            fs::copy(&self.data_file, backup_file)?;
        }
        Ok(())
    }

    fn atomic_write_sessions(&self, data: &SessionData) -> Result<(), StorageError> {
        let temp_file = self.config_path.join("sessions.json.tmp");
        let json = serde_json::to_string_pretty(data)?;

        // Write to temporary file first
        fs::write(&temp_file, json)?;

        // Atomic move to final location
        fs::rename(temp_file, &self.data_file)?;

        Ok(())
    }

    fn validate_session_data_integrity(&self, data: &SessionData) -> Result<(), StorageError> {
        // Basic validation checks for sessions
        for session in &data.sessions {
            if session.id.is_empty() {
                return Err(StorageError::DataCorruption(
                    "Session with empty ID found".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn create_corrupted_backup(&self) -> Result<(), StorageError> {
        if self.data_file.exists() {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let backup_file = self.config_path.join(format!("sessions_corrupted_{timestamp}.json.backup"));
            fs::copy(&self.data_file, backup_file)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Project, Session};
    use std::fs;
    use tempfile::TempDir;

    fn create_test_storage() -> (JsonStorage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage = JsonStorage::with_custom_path(temp_dir.path().to_path_buf());
        (storage, temp_dir)
    }

    #[test]
    fn test_storage_creation() {
        let temp_dir = TempDir::new().unwrap();
        let storage = JsonStorage::with_custom_path(temp_dir.path().to_path_buf());

        assert_eq!(Storage::get_config_path(&storage), temp_dir.path());
        assert_eq!(storage.data_file, temp_dir.path().join("sessions.json"));
    }

    #[test]
    fn test_load_nonexistent_file() {
        let (storage, _temp_dir) = create_test_storage();

        let data = storage.load().unwrap();
        assert_eq!(data.projects.len(), 0);
        assert_eq!(data.sessions.len(), 0);
    }

    #[test]
    fn test_load_empty_file() {
        let (storage, temp_dir) = create_test_storage();

        // Create empty file
        fs::write(temp_dir.path().join("data.json"), "").unwrap();

        let data = storage.load().unwrap();
        assert_eq!(data.projects.len(), 0);
        assert_eq!(data.sessions.len(), 0);
    }

    #[test]
    fn test_save_and_load_cycle() {
        let (storage, temp_dir) = create_test_storage();

        // Create test data
        let mut data = AppData::new();
        let project = Project::new("test-project".to_string(), temp_dir.path().to_path_buf());
        let project_id = project.id.clone();
        data.add_project(project);

        let session = Session::new(Some(project_id.clone()));
        data.add_session(session);

        // Save data
        storage.save(&data).unwrap();

        // Verify file exists
        assert!(storage.data_file.exists());

        // Load data back
        let loaded_data = storage.load().unwrap();

        assert_eq!(loaded_data.projects.len(), 1);
        assert_eq!(loaded_data.sessions.len(), 1);
        assert_eq!(loaded_data.projects[0].name, "test-project");
        assert_eq!(loaded_data.sessions[0].project_id, Some(project_id));
    }

    #[test]
    fn test_atomic_write() {
        let (storage, _temp_dir) = create_test_storage();

        let data = AppData::new();
        storage.save(&data).unwrap();

        // Verify no temporary file exists after save
        let temp_file = storage.config_path.join("data.json.tmp");
        assert!(!temp_file.exists());

        // Verify actual file exists
        assert!(storage.data_file.exists());
    }

    #[test]
    #[ignore]
    fn test_backup_creation() {
        let (storage, _temp_dir) = create_test_storage();

        // Save initial data
        let data1 = AppData::new();
        storage.save(&data1).unwrap();

        // Save second version (should create backup)
        let mut data2 = AppData::new();
        let temp_dir = TempDir::new().unwrap();
        let project = Project::new("test".to_string(), temp_dir.path().to_path_buf());
        data2.add_project(project);
        storage.save(&data2).unwrap();

        // Verify backup exists
        let backup_file = storage.config_path.join("data.json.backup");
        assert!(backup_file.exists());

        // Verify backup contains original data
        let backup_contents = fs::read_to_string(backup_file).unwrap();
        let backup_data: AppData = serde_json::from_str(&backup_contents).unwrap();
        assert_eq!(backup_data.projects.len(), 0);
    }

    #[test]
    fn test_validate_projects() {
        let (storage, _temp_dir) = create_test_storage();

        let temp_dir = TempDir::new().unwrap();
        let mut projects = vec![
            Project::new("existing".to_string(), temp_dir.path().to_path_buf()),
            Project::new("missing".to_string(), PathBuf::from("/non/existent/path")),
        ];

        let removed_ids = storage.validate_projects(&mut projects);

        assert_eq!(projects.len(), 1);
        assert_eq!(removed_ids.len(), 1);
        assert_eq!(projects[0].name, "existing");
    }

    #[test]
    fn test_data_integrity_validation() {
        let (storage, _temp_dir) = create_test_storage();

        // Test with invalid project (empty ID)
        let mut invalid_data = AppData::new();
        let mut invalid_project = Project::new("test".to_string(), PathBuf::from("/tmp"));
        invalid_project.id = String::new(); // Invalid empty ID
        invalid_data.projects.push(invalid_project);

        let result = storage.validate_data_integrity(&invalid_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty ID"));
    }

    #[test]
    #[ignore]
    fn test_malformed_json_handling() {
        let (storage, temp_dir) = create_test_storage();

        // Write malformed JSON
        fs::write(temp_dir.path().join("sessions.json"), "{ invalid json }").unwrap();

        let result = storage.load_sessions();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StorageError::Serialization(_)
        ));
    }

    #[test]
    fn test_auto_cleanup_on_load() {
        let (storage, temp_dir) = create_test_storage();

        // Create data with both existing and missing projects
        let mut data = AppData::new();
        let existing_project = Project::new("existing".to_string(), temp_dir.path().to_path_buf());
        let missing_project = Project::new("missing".to_string(), PathBuf::from("/non/existent"));

        data.add_project(existing_project);
        data.add_project(missing_project);

        // Save the data
        storage.save(&data).unwrap();

        // Load it back (should auto-cleanup missing projects)
        let loaded_data = storage.load().unwrap();

        assert_eq!(loaded_data.projects.len(), 1);
        assert_eq!(loaded_data.projects[0].name, "existing");
    }

    #[test]
    fn test_concurrent_access_safety() {
        let (storage, _temp_dir) = create_test_storage();

        let data = AppData::new();

        // Multiple saves should not interfere with each other
        storage.save(&data).unwrap();
        storage.save(&data).unwrap();
        storage.save(&data).unwrap();

        let loaded_data = storage.load().unwrap();
        assert_eq!(loaded_data.projects.len(), 0);
    }
}
