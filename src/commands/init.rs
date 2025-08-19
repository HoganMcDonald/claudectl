use clap::Args;
use crate::commands::CommandResult;
use crate::utils::errors::CommandError;
use crate::utils::output::blank;

#[derive(Args)]
pub struct InitCommand {
}

impl InitCommand {
    pub fn execute(&self) -> CommandResult<()> {
        blank();
        println!("Initializing claudectl");

        Err(CommandError::new("This command is not yet implemented."))
    }
}
