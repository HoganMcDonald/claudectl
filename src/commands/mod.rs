pub mod init;

use crate::utils::errors::*;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize the project for claudectl
    Init(init::InitCommand),
}

pub fn handle_command(command: Commands) -> CommandResult<()> {
    match command {
        Commands::Init(cmd) => {
            cmd.execute()?;
            Ok(())
        }
    }
}

pub type CommandResult<T> = Result<T, CommandError>;
