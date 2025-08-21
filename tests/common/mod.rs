use std::fs;
use tempfile::TempDir;

/// Set up a temporary directory with a .git folder for testing
pub fn setup_git_repo() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir(&temp_dir).unwrap();
    fs::create_dir(".git").unwrap();
    std::env::set_current_dir(original_dir).unwrap();

    temp_dir
}

/// Set up a temporary directory without git for testing
pub fn setup_non_git_repo() -> TempDir {
    TempDir::new().unwrap()
}

/// Helper to change to a directory and restore it automatically
pub struct TempDirGuard {
    temp_dir: TempDir,
    original_dir: std::path::PathBuf,
}

impl TempDirGuard {
    pub fn new_git_repo() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();
        fs::create_dir(".git").unwrap();

        Self {
            temp_dir,
            original_dir,
        }
    }

    pub fn new_empty() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();

        Self {
            temp_dir,
            original_dir,
        }
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.original_dir).unwrap();
    }
}
