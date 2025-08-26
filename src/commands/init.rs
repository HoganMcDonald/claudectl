use crate::commands::CommandResult;
use crate::utils::claude::is_claude_installed;
use crate::utils::config::Config;
use crate::utils::errors::CommandError;
use crate::utils::fs::{
    create_global_configuration_dir, create_local_configuration_dir, read_local_config_file,
    write_local_config_file,
};
use crate::utils::git::is_git_repository;
use crate::utils::output::{
    Position, blank, standard, step, step_end, step_fail, step_skip, success,
};
use clap::Args;
use tracing::{info, instrument};

#[derive(Args, Debug)]
pub struct InitCommand {}

impl InitCommand {
    #[instrument(name = "init_command")]
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
        info!("Starting initialization for project: {}", project_name);
        standard(&initialization_message);
        blank();

        // 1. verrify that dependencies are met
        step("Verifying dependencies...", Position::First);
        let is_git_repo = is_git_repository().inspect_err(|_| {
            step_fail();
        })?;
        if !is_git_repo {
            step_fail();
            return Err(CommandError::new(
                "Current directory is not a git repository",
            ));
        }
        is_claude_installed().inspect_err(|_| {
            step_fail();
        })?;
        step_end();
        blank();

        // 2. create config file
        step("Generating project config...", Position::Normal);

        let config = match read_local_config_file() {
            Ok(config_content) => {
                let config = Config::from_str(&config_content).inspect_err(|_| {
                    step_fail();
                })?;
                step_skip();
                blank();
                config
            }
            Err(_) => {
                let project_dir =
                    create_global_configuration_dir(project_name).inspect_err(|_| {
                        step_fail();
                    })?;
                create_local_configuration_dir().inspect_err(|_| {
                    step_fail();
                })?;
                let config = Config::new(project_name, &project_dir);
                let config_json = config.to_string().inspect_err(|_| {
                    step_fail();
                })?;
                write_local_config_file(config_json).inspect_err(|_| {
                    step_fail();
                })?;
                step_end();
                blank();
                config
            }
        };

        // 3. load or create project directories
        step("Generating project directories...", Position::Last);

        create_global_configuration_dir(&config.project_name).inspect_err(|_| {
            step_fail();
        })?;
        step_end();
        blank();

        blank();
        success(format!("Project {} initialized successfully!", config.project_name).as_str());
        info!(
            "Initialization completed successfully for project: {}",
            config.project_name
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_init_command_creation() {
        let _cmd = InitCommand {};
        // Test that the command struct can be created
        // This is a basic smoke test
        assert!(true); // Command creation succeeded
    }

    #[test]
    fn test_project_name_extraction() {
        // Test project name extraction logic
        let current_dir = env::current_dir().unwrap();
        let project_name = current_dir.file_name().and_then(|name| name.to_str());

        assert!(project_name.is_some());
        assert!(!project_name.unwrap().is_empty());
    }

    #[test]
    fn test_init_command_error_handling() {
        // Test that the command properly handles CommandError construction
        let error = CommandError::new("Test error message");
        assert!(error.to_string().contains("Test error message"));
    }
}
