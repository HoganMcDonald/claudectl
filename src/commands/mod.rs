pub mod init;
pub mod list;
pub mod rm;
pub mod task;

use crate::utils::errors::CommandError;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize the project for claudectl
    Init(init::InitCommand),
    /// Create a new task worktree
    Task(task::TaskCommand),
    /// List all task worktrees
    List(list::ListCommand),
    /// Remove a task worktree
    Rm(rm::RmCommand),
}

pub fn handle_command(command: Commands) -> CommandResult<()> {
    match command {
        Commands::Init(cmd) => cmd.execute(),
        Commands::Task(cmd) => cmd.execute(),
        Commands::List(cmd) => cmd.execute(),
        Commands::Rm(cmd) => cmd.execute(),
    }
}

pub type CommandResult<T> = Result<T, CommandError>;
