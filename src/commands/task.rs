use crate::commands::CommandResult;
use crate::utils::config::Config;
use crate::utils::errors::CommandError;
use crate::utils::fs::read_local_config_file;
use crate::utils::git::{create_worktree, fetch_origin, worktree_exists};
use crate::utils::output::{Position, blank, step, step_end, step_fail, success};
use clap::Args;
use tracing::{error, info, instrument, warn};

#[derive(Debug, Args)]
pub struct TaskCommand {
    /// The name of the task/branch (e.g., feat/new-feature)
    pub task_name: String,
}

impl TaskCommand {
    #[instrument(name = "task_command", fields(task_name = %self.task_name))]
    pub fn execute(&self) -> CommandResult<()> {
        info!("Executing task command for: {}", self.task_name);
        let raw_config = read_local_config_file()?;
        let config = Config::from_str(&raw_config)?;
        info!("Loaded configuration for project: {}", config.project_name);

        // 1. Fetch latest changes from origin
        step("Fetching latest changes from origin...", Position::First);
        fetch_origin().inspect_err(|e| {
            error!("Failed to fetch from origin: {}", e);
            step_fail();
        })?;
        info!("Successfully fetched latest changes from origin");
        step_end();
        blank();

        // 2. Check if worktree already exists
        let worktree_path = format!("{}/{}", config.project_dir, self.task_name);
        info!("Checking for existing worktree at: {}", worktree_path);
        step("Creating git worktree...", Position::Last);
        let exists = worktree_exists(&worktree_path).inspect_err(|e| {
            error!("Failed to check worktree existence: {}", e);
            step_fail();
        })?;
        if exists {
            warn!("Worktree already exists at path: {}", worktree_path);
            step_fail();
            return Err(CommandError::new(&format!(
                "Worktree already exists at path: {worktree_path}"
            )));
        }
        info!("Worktree path is available");
        create_worktree(&self.task_name, &worktree_path).inspect_err(|e| {
            error!("Failed to create worktree: {}", e);
            step_fail();
        })?;
        info!(
            "Successfully created worktree '{}' at: {}",
            self.task_name, worktree_path
        );
        step_end();
        blank();

        blank();
        success(&format!(
            "Task worktree '{}' created successfully at: {}",
            self.task_name, worktree_path
        ));

        info!("Task command completed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_command_creation() {
        let cmd = TaskCommand {
            task_name: "feat/test-feature".to_string(),
        };

        assert_eq!(cmd.task_name, "feat/test-feature");
    }

    #[test]
    fn test_task_command_with_different_name_formats() {
        let test_cases = vec![
            "feat/new-feature",
            "bugfix/issue-123",
            "hotfix/critical-fix",
            "feature/user-auth",
        ];

        for task_name in test_cases {
            let cmd = TaskCommand {
                task_name: task_name.to_string(),
            };
            assert_eq!(cmd.task_name, task_name);
            assert!(!cmd.task_name.is_empty());
        }
    }

    #[test]
    fn test_worktree_path_construction() {
        let task_name = "feat/test-feature";
        let config_project_dir = "/tmp/test-project";
        let expected_path = format!("{config_project_dir}/{task_name}");

        assert_eq!(expected_path, "/tmp/test-project/feat/test-feature");
    }

    #[test]
    fn test_task_command_debug_formatting() {
        let cmd = TaskCommand {
            task_name: "feat/debug-test".to_string(),
        };

        let debug_str = format!("{cmd:?}");
        assert!(debug_str.contains("TaskCommand"));
        assert!(debug_str.contains("feat/debug-test"));
    }
}
