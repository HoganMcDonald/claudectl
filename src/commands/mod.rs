pub mod init;

use clap::Subcommand;
use crate::utils::ValidationResult;

#[derive(Subcommand)]
pub enum Commands {
    Init(init::InitCommand),
}

pub fn handle_command(command: Commands) -> ValidationResult<()> {
    match command {
        Commands::Init(cmd) => cmd.execute(),
    }
}
