use std::io::{self, Write};

use clap::{Args, ValueHint};
use owo_colors::OwoColorize;
use tracing::{error, info, instrument, warn};

use crate::{
    commands::CommandResult,
    utils::{
        config::Config,
        errors::CommandError,
        fs::read_local_config_file,
        git::{remove_worktree, worktree_list},
        icons::ICONS,
        output::{error as output_error, success},
        theme::THEME,
    },
};

fn task_name_parser(s: &str) -> Result<String, String> {
    // Basic validation - just return the string as-is
    // The actual validation happens in execute()
    Ok(s.to_string())
}

#[derive(Args, Debug)]
pub struct RmCommand {
    /// The name of the task/branch to remove
    #[arg(
        value_parser = task_name_parser,
        value_hint = ValueHint::Other,
        help = "The name of the task to remove"
    )]
    pub task_name: String,
}

impl RmCommand {
    #[instrument(name = "rm_command", fields(task_name = %self.task_name))]
    pub fn execute(&self) -> CommandResult<()> {
        info!("Executing rm command for: {}", self.task_name);

        let raw_config = read_local_config_file()?;
        let config = Config::from_str(&raw_config)?;
        info!("Loaded configuration for project: {}", config.project_name);

        // 1. Get all worktrees to find the one to remove
        let worktrees = worktree_list().inspect_err(|e| {
            output_error(&format!("Failed to get tasks: {e}"));
        })?;

        // 2. Find the worktree that matches the task name
        let target_worktree = worktrees
            .into_iter()
            .find(|wt| wt.branch.as_ref() == Some(&self.task_name))
            .ok_or_else(|| CommandError::new(&format!("Task '{}' not found", self.task_name)))?;

        let worktree_path = &target_worktree.path;
        info!(
            "Found worktree for task '{}' at: {}",
            self.task_name, worktree_path
        );

        // 3. Confirmation prompt
        print!(
            "{} Are you sure you want to remove task '{}' and its worktree? (y/N): ",
            ICONS.status.warning.color(THEME.warning),
            self.task_name.color(THEME.info)
        );
        io::stdout()
            .flush()
            .map_err(|e| CommandError::new(&format!("Failed to flush stdout: {e}")))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| CommandError::new(&format!("Failed to read input: {e}")))?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            info!("Task removal cancelled by user");
            success("Task removal cancelled");
            return Ok(());
        }

        // 4. Remove the worktree
        info!("Removing worktree at: {}", worktree_path);
        remove_worktree(worktree_path).inspect_err(|e| {
            error!("Failed to remove worktree: {}", e);
            output_error(&format!("Failed to remove worktree: {e}"));
        })?;

        info!("Successfully removed task: {}", self.task_name);
        success(&format!(
            "Successfully removed task '{}' and its worktree",
            self.task_name
        ));

        Ok(())
    }
}

#[allow(dead_code)]
fn get_available_tasks() -> Vec<String> {
    // Get available task names for autocompletion
    match worktree_list() {
        Ok(worktrees) => worktrees
            .into_iter()
            .filter_map(|wt| wt.branch)
            .filter(|branch| branch != "main" && !branch.contains("HEAD"))
            .collect(),
        Err(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_name_parser() {
        let result = task_name_parser("test-task");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-task");
    }

    #[test]
    fn test_get_available_tasks_handles_errors() {
        // This test ensures the function doesn't panic even if git operations fail
        let _tasks = get_available_tasks();
        // Test passes if function doesn't panic
    }

    #[test]
    fn test_rm_command_creation() {
        let cmd = RmCommand {
            task_name: "test-task".to_string(),
        };
        assert_eq!(cmd.task_name, "test-task");
    }
}
