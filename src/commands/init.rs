use crate::commands::CommandResult;
use crate::utils::claude::is_claude_installed;
use crate::utils::errors::CommandError;
use crate::utils::git::is_git_repository;
use crate::utils::output::{Position, blank, standard, step};
use clap::Args;

#[derive(Args)]
pub struct InitCommand {}

impl InitCommand {
    pub fn execute(&self) -> CommandResult<()> {
        let current_dir = std::env::current_dir()
            .map_err(|e| CommandError::new(&format!("Failed to get current directory: {e}")))?;

        let project_name = current_dir
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                CommandError::new("Failed to get project name from current directory.")
            })?;

        let initialization_message =
            format!("Initializing project '{project_name}' for use with claudectl...");
        standard(&initialization_message);
        blank();

        // 1. verrify that dependencies are met
        step("Verifying Dependencies...", Position::First);
        is_git_repository()?;
        is_claude_installed()?;

        Ok(())
    }
}
