#[derive(Debug)]
pub struct CommandError {
    pub message: String,
}

impl CommandError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

// =================================================
// GitError:
//      Custom error type for Git-related operations
// =================================================
#[derive(Debug)]
pub enum GitAction {
    /// Not a Git repository
    Repo,
}

#[derive(Debug)]
pub struct GitError {
    pub message: String,
    pub action: GitAction,
}

impl GitError {
    pub fn new(message: &str, action: GitAction) -> Self {
        Self {
            message: message.to_string(),
            action,
        }
    }

    fn description(&self) -> String {
        format!("[Git Error] {:?} - {}", self.action, self.message)
    }
}

impl From<GitError> for CommandError {
    fn from(val: GitError) -> Self {
        CommandError::new(&val.description())
    }
}

// =================================================
// ClaudeError:
//      Custom error type for Claude-related operations
// =================================================
#[derive(Debug)]
pub struct ClaudeError {
    pub message: String,
}

impl ClaudeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    fn description(&self) -> String {
        format!("[Claude Error] {}", self.message)
    }
}

impl From<ClaudeError> for CommandError {
    fn from(val: ClaudeError) -> Self {
        CommandError::new(&val.description())
    }
}

// =================================================
// FileSystemError:
//      Custom error type for file system operations
// =================================================
#[derive(Debug)]
pub struct FileSystemError {
    pub message: String,
    pub path: String,
}

impl FileSystemError {
    pub fn new(message: &str, path: &str) -> Self {
        Self {
            message: message.to_string(),
            path: path.to_string(),
        }
    }

    fn description(&self) -> String {
        format!(
            "[File System Error] {}.\n  Path: {}",
            self.message, self.path
        )
    }
}

impl From<FileSystemError> for CommandError {
    fn from(val: FileSystemError) -> Self {
        CommandError::new(&val.description())
    }
}

// =================================================
// ConfigError:
//      Custom error type for claudectl config
// =================================================
#[derive(Debug)]
pub struct ConfigError {
    pub message: String,
}

impl ConfigError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    fn description(&self) -> String {
        format!("[Config Error] {}", self.message)
    }
}

impl From<ConfigError> for CommandError {
    fn from(val: ConfigError) -> Self {
        CommandError::new(&val.description())
    }
}
