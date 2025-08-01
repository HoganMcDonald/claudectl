mod config;
mod modules;

use clap::Parser;
use modules::cli::{Cli, Commands, WorkspaceCommands};

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
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