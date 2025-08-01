use clap::{Parser, Subcommand};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "claudectl")]
#[command(author, version)]
#[command(about = "A command-line tool for managing Claude projects", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Manage workspaces")]
    Workspace {
        #[command(subcommand)]
        command: WorkspaceCommands,
    },
}

#[derive(Subcommand)]
enum WorkspaceCommands {
    #[command(
        about = "Initialize a new workspace in the current repository",
        long_about = "Initialize a new workspace in the current repository.\n\n\
                     This creates a workspace directory at ./.claudectl/workspaces/<uuid>\n\
                     with a config.json file containing workspace metadata.\n\n\
                     Examples:\n  \
                       claudectl workspace new                    # Creates workspace named 'New Workspace'\n  \
                       claudectl workspace new \"My Project\"       # Creates workspace named 'My Project'"
    )]
    New {
        #[arg(help = "Name of the workspace (defaults to 'New Workspace' if not specified)")]
        name: Option<String>,
    },
    #[command(
        about = "List all workspaces",
        long_about = "List all workspaces in the current repository.\n\n\
                     Shows workspace IDs, names, and creation times.\n\n\
                     Example:\n  \
                       claudectl workspace list"
    )]
    List,
}

#[derive(Serialize, Deserialize)]
struct WorkspaceConfig {
    id: Uuid,
    name: String,
    created: DateTime<Utc>,
    version: String,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Workspace { command } => match command {
            WorkspaceCommands::New { name } => {
                let workspace_name = name.unwrap_or_else(|| "New Workspace".to_string());
                initialize_workspace(&workspace_name);
            }
            WorkspaceCommands::List => {
                list_workspaces();
            }
        },
    }
}

fn initialize_workspace(name: &str) {
    let workspace_id = Uuid::now_v7();
    println!("Initializing workspace: {}", name);
    
    let workspace_dir = format!("./.claudectl/workspaces/{}", workspace_id);
    
    if let Err(e) = fs::create_dir_all(&workspace_dir) {
        eprintln!("Error creating workspace directory: {}", e);
        std::process::exit(1);
    }
    
    let config = WorkspaceConfig {
        id: workspace_id,
        name: name.to_string(),
        created: Utc::now(),
        version: "1.0".to_string(),
    };
    
    let config_content = serde_json::to_string_pretty(&config).unwrap();
    
    let config_path = format!("{}/config.json", workspace_dir);
    if let Err(e) = fs::write(&config_path, config_content) {
        eprintln!("Error creating workspace config: {}", e);
        std::process::exit(1);
    }
    
    println!("Workspace '{}' (ID: {}) initialized successfully", name, workspace_id);
}

fn list_workspaces() {
    let workspaces_dir = "./.claudectl/workspaces";
    
    if !Path::new(workspaces_dir).exists() {
        println!("No workspaces found. Create one with: claudectl workspace new [name]");
        return;
    }
    
    let entries = match fs::read_dir(workspaces_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading workspaces directory: {}", e);
            std::process::exit(1);
        }
    };
    
    let mut workspaces = Vec::new();
    
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let config_path = entry.path().join("config.json");
                    if config_path.exists() {
                        if let Ok(config_content) = fs::read_to_string(&config_path) {
                            if let Ok(config) = serde_json::from_str::<WorkspaceConfig>(&config_content) {
                                workspaces.push(config);
                            }
                        }
                    }
                }
            }
        }
    }
    
    if workspaces.is_empty() {
        println!("No workspaces found. Create one with: claudectl workspace new [name]");
    } else {
        workspaces.sort_by(|a, b| a.created.cmp(&b.created));
        
        println!("Workspaces:");
        for workspace in workspaces {
            println!("  - {} ({})", workspace.name, workspace.id);
            println!("    Created: {}", workspace.created.format("%Y-%m-%d %H:%M:%S UTC"));
        }
    }
}