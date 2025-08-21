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
            action: action,
        }
    }

    fn description(&self) -> String {
        format!("[Git Error] {:?} - {}", self.action, self.message)
    }
}

impl Into<CommandError> for GitError {
    fn into(self) -> CommandError {
        CommandError::new(&self.description())
    }
}
