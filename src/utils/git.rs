use crate::utils::errors::{GitAction, GitError};

pub fn is_git_repository() -> Result<bool, GitError> {
    if std::path::Path::new(".git").exists() {
        Ok(true)
    } else {
        Err(GitError::new("Current directory is not a git repository.", GitAction::Repo))
    }
}
