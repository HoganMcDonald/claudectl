use clap::Args;
use crate::commands::CommandResult;
use crate::utils::errors::CommandError;
use crate::utils::output::{blank, standard, step, Position};
use crate::utils::git::is_git_repository;

#[derive(Args)]
pub struct InitCommand {
}

impl InitCommand {
    pub fn execute(&self) -> CommandResult<()> {
        let current_dir = std::env::current_dir()
            .map_err(|e| CommandError::new(&format!("Failed to get current directory: {}", e)))?;

        let project_name = current_dir
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| CommandError::new("Failed to get project name from current directory."))?;

        let initialization_message = format!(
            "Initializing project '{}' for use with claudectl...",
            project_name
        );
        standard(&initialization_message);
        blank();

        // 1. verrify that current directory is a git repository, claude is installed
        step("Verifying Dependencies...", Position::First);
        is_git_repository()
            .map_err(|e| e.into())?;


        Ok(())
    }
}
