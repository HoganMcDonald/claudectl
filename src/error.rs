use thiserror::Error;

/// Custom errors for claudectl operations
#[derive(Debug, Error)]
pub enum ClaudeCtlError {
    #[error("Git operation failed: {0}")]
    Git(String),
    
    #[error("Workspace configuration error: {0}")]
    Config(String),
    
    #[error("Workspace validation error: {0}")]
    Validation(String),
    
    #[error("File system operation failed: {0}")]
    Filesystem(#[from] std::io::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Environment variable error: {0}")]
    Environment(String),
    
    #[error("Workspace '{name}' already exists")]
    WorkspaceExists { name: String },
    
    #[error("Workspace '{name}' not found")]
    WorkspaceNotFound { name: String },
}

pub type Result<T> = std::result::Result<T, ClaudeCtlError>;

impl From<std::env::VarError> for ClaudeCtlError {
    fn from(err: std::env::VarError) -> Self {
        ClaudeCtlError::Environment(err.to_string())
    }
}