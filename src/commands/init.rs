use crate::commands::CommandResult;
use crate::utils::claude::is_claude_installed;
use crate::utils::config::Config;
use crate::utils::errors::CommandError;
use crate::utils::fs::{
    create_global_configuration_dir, create_local_configuration_dir, read_local_config_file,
    write_local_config_file,
};
use crate::utils::git::is_git_repository;
use crate::utils::output::{Position, blank, standard, step, step_end, step_fail};
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
        is_git_repository().inspect_err(|_| {
            step_fail();
        })?;
        is_claude_installed().inspect_err(|_| {
            step_fail();
        })?;
        step_end();
        blank();

        // 2. load or create config structure
        step("Creating Configuration Structure...", Position::Normal);

        let config = match read_local_config_file() {
            Ok(config_content) => Config::from_str(&config_content).inspect_err(|_| {
                step_fail();
            })?,
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
                config
            }
        };

        create_global_configuration_dir(&config.project_name).inspect_err(|_| {
            step_fail();
        })?;

        step_end();
        blank();

        Ok(())
    }
}
