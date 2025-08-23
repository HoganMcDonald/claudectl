use crate::commands::CommandResult;
use crate::utils::config::Config;
use crate::utils::errors::CommandError;
use crate::utils::fs::read_local_config_file;
use crate::utils::git::{create_worktree, fetch_origin, is_git_repository, worktree_exists};
use crate::utils::output::{Position, blank, standard, step, step_end, step_fail, success};
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

        let task_creation_message = format!("Creating task worktree for '{}'...", self.task_name);
        standard(&task_creation_message);
        blank();

        // 1. Verify that we're in a git repository
        step("Verifying git repository...", Position::First);
        is_git_repository().inspect_err(|e| {
            error!("Git repository validation failed: {}", e);
            step_fail();
        })?;
        info!("Git repository validation passed");
        step_end();
        blank();

        // 2. Fetch latest changes from origin
        step("Fetching latest changes from origin...", Position::Normal);
        fetch_origin().inspect_err(|e| {
            error!("Failed to fetch from origin: {}", e);
            step_fail();
        })?;
        info!("Successfully fetched latest changes from origin");
        step_end();
        blank();

        // 3. Check if worktree already exists
        let worktree_path = format!("{}/{}", config.project_dir, self.task_name);
        info!("Checking for existing worktree at: {}", worktree_path);
        step("Checking for existing worktree...", Position::Normal);
        if worktree_exists(&worktree_path).inspect_err(|e| {
            error!("Failed to check worktree existence: {}", e);
            step_fail();
        })? {
            warn!("Worktree already exists at path: {}", worktree_path);
            step_fail();
            return Err(CommandError::new(&format!(
                "Worktree already exists at path: {worktree_path}"
            )));
        }
        info!("Worktree path is available");
        step_end();
        blank();

        // 4. Create the worktree
        step("Creating git worktree...", Position::Last);
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
