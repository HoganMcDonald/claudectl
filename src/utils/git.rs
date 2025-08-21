use crate::utils::errors::{GitAction, GitError};

pub fn is_git_repository() -> Result<bool, GitError> {
    if std::path::Path::new(".git").exists() {
        Ok(true)
    } else {
        Err(GitError::new(
            "Current directory is not a git repository.",
            GitAction::Repo,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_git_repository_when_git_exists() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory and create .git
        std::env::set_current_dir(&temp_dir).unwrap();
        fs::create_dir(".git").unwrap();

        let result = is_git_repository();

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        match result {
            Ok(value) => assert_eq!(value, true),
            Err(e) => panic!("Expected Ok but got Err: {:?}", e),
        }
    }

    #[test]
    fn test_is_git_repository_when_git_missing() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory without .git
        std::env::set_current_dir(&temp_dir).unwrap();

        let result = is_git_repository();

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.message, "Current directory is not a git repository.");
        assert!(matches!(error.action, GitAction::Repo));
    }
}
