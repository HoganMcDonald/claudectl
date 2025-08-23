use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("{0}")]
    General(String),

    #[error(transparent)]
    Git(#[from] GitError),

    #[error(transparent)]
    FileSystem(#[from] FileSystemError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Claude(#[from] ClaudeError),
}

impl CommandError {
    pub fn new(message: &str) -> Self {
        Self::General(message.to_string())
    }

    pub fn message(&self) -> String {
        self.to_string()
    }
}

// =================================================
// GitError:
//      Custom error type for Git-related operations
// =================================================
#[derive(Debug, Error)]
pub enum GitError {
    #[error("Not a Git repository: {message}")]
    NotRepository { message: String },

    #[error("Failed to fetch from origin: {message}")]
    FetchFailed { message: String },

    #[error("Failed to list worktrees: {message}")]
    WorktreeListFailed { message: String },

    #[error("Failed to add worktree: {message}")]
    WorktreeAddFailed { message: String },
}

impl GitError {
    pub fn new(message: &str, action: GitAction) -> Self {
        let message = message.to_string();
        match action {
            GitAction::Repo => Self::NotRepository { message },
            GitAction::Fetch => Self::FetchFailed { message },
            GitAction::WorktreeList => Self::WorktreeListFailed { message },
            GitAction::WorktreeAdd => Self::WorktreeAddFailed { message },
        }
    }
}

#[derive(Debug)]
pub enum GitAction {
    /// Not a Git repository
    #[allow(dead_code)]
    Repo,
    Fetch,
    WorktreeList,
    WorktreeAdd,
}

// =================================================
// ClaudeError:
//      Custom error type for Claude-related operations
// =================================================
#[derive(Debug, Error)]
pub enum ClaudeError {
    #[error("Claude is not installed: {message}")]
    NotInstalled { message: String },

    #[error("Claude command failed: {message}")]
    #[allow(dead_code)]
    CommandFailed { message: String },
}

impl ClaudeError {
    pub fn new(message: &str) -> Self {
        Self::NotInstalled {
            message: message.to_string(),
        }
    }
}

// =================================================
// FileSystemError:
//      Custom error type for file system operations
// =================================================
#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("Failed to access directory: {message}\nPath: {path}")]
    DirectoryAccess { message: String, path: String },

    #[error("Configuration file not found: {message}\nPath: {path}")]
    ConfigNotFound { message: String, path: String },

    #[error("Failed to read file: {message}\nPath: {path}")]
    ReadFailed { message: String, path: String },

    #[error("Failed to write file: {message}\nPath: {path}")]
    WriteFailed { message: String, path: String },
}

impl FileSystemError {
    pub fn new(message: &str, path: &str) -> Self {
        Self::DirectoryAccess {
            message: message.to_string(),
            path: path.to_string(),
        }
    }

    pub fn config_not_found(message: &str, path: &str) -> Self {
        Self::ConfigNotFound {
            message: message.to_string(),
            path: path.to_string(),
        }
    }

    pub fn read_failed(message: &str, path: &str) -> Self {
        Self::ReadFailed {
            message: message.to_string(),
            path: path.to_string(),
        }
    }

    pub fn write_failed(message: &str, path: &str) -> Self {
        Self::WriteFailed {
            message: message.to_string(),
            path: path.to_string(),
        }
    }
}

// =================================================
// ConfigError:
//      Custom error type for claudectl config
// =================================================
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to parse configuration: {message}")]
    ParseFailed { message: String },

    #[error("Failed to serialize configuration: {message}")]
    SerializeFailed { message: String },

    #[error("Invalid configuration: {message}")]
    #[allow(dead_code)]
    Invalid { message: String },
}

impl ConfigError {
    pub fn new(message: &str) -> Self {
        Self::ParseFailed {
            message: message.to_string(),
        }
    }

    pub fn serialize_failed(message: &str) -> Self {
        Self::SerializeFailed {
            message: message.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn invalid(message: &str) -> Self {
        Self::Invalid {
            message: message.to_string(),
        }
    }
}
