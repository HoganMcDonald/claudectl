use clap::Args;
use crate::commands::CommandResult;
use crate::utils::errors::CommandError;
use crate::utils::output::{blank, standard};

#[derive(Args)]
pub struct InitCommand {
}

impl InitCommand {
    pub fn execute(&self) -> CommandResult<()> {
        let project_name = "claudectl";

        let initialization_message = format!(
            "Initializing project '{}' for use with claudectl...",
            project_name
        );
        standard(&initialization_message);
        blank();

        Err(CommandError::new("This command is not yet implemented."))
    }
}
