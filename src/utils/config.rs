use crate::utils::errors::ConfigError;
use serde::{Deserialize, Serialize};

type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub project_name: String,
    pub project_dir: String,
}

impl Config {
    pub fn new(project_name: &str, project_dir: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
            project_dir: project_dir.to_string(),
        }
    }

    pub fn from_str(json_str: &str) -> ConfigResult<Self> {
        serde_json::from_str(json_str)
            .map_err(|e| ConfigError::new(&format!("Failed to parse configuration JSON: {e}")))
    }

    pub fn to_string(&self) -> ConfigResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::serialize_failed(&format!("JSON serialization error: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_str_valid_json() {
        let json = r#"{
            "project_name": "test-project",
            "project_dir": "/path/to/project"
        }"#;

        let config = Config::from_str(json).unwrap();
        assert_eq!(config.project_name, "test-project");
        assert_eq!(config.project_dir, "/path/to/project");
    }

    #[test]
    fn test_config_from_str_invalid_json() {
        let invalid_json = "{ invalid json }";
        let result = Config::from_str(invalid_json);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to parse configuration")
        );
    }

    #[test]
    fn test_config_to_string() {
        let config = Config::new("test-project", "/test/dir");
        let json_string = config.to_string().unwrap();

        // Parse it back to verify it's valid JSON
        let parsed_config = Config::from_str(&json_string).unwrap();
        assert_eq!(config.project_name, parsed_config.project_name);
        assert_eq!(config.project_dir, parsed_config.project_dir);
    }
}
