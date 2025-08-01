mod config;
mod error;
mod modules;

use clap::Parser;
use error::{ClaudeCtlError, Result};
use modules::cli::{Cli, Commands, WorkspaceCommands};

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(cli) {
        // Print user-friendly error message
        match &e {
            ClaudeCtlError::Git(msg) => {
                eprintln!("Git error: {msg}");
                eprintln!("Make sure you're in a git repository and have committed changes.");
            }
            ClaudeCtlError::Config(msg) => {
                eprintln!("Configuration error: {msg}");
            }
            ClaudeCtlError::Validation(msg) => {
                eprintln!("Validation error: {msg}");
            }
            ClaudeCtlError::Environment(msg) => {
                eprintln!("Environment error: {msg}");
                eprintln!("Make sure required environment variables are set.");
            }
            _ => eprintln!("Error: {e}"),
        }
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Workspace { command } => match command {
            WorkspaceCommands::New { name } => {
                let workspace_name = name.unwrap_or_else(|| "New Workspace".to_string());
                modules::workspace::initialize(&workspace_name)?;
            }
            WorkspaceCommands::List => {
                modules::workspace::list()?;
            }
        },
    }
    
    Ok(())
}