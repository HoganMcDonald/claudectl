use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claudectl")]
#[command(author, version)]
#[command(about = "A command-line tool for managing Claude projects", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Manage workspaces")]
    Workspace {
        #[command(subcommand)]
        command: WorkspaceCommands,
    },
}

#[derive(Subcommand)]
pub enum WorkspaceCommands {
    #[command(
        about = "Initialize a new workspace in the current repository",
        long_about = "Initialize a new workspace in the current repository.\n\n\
                     This creates:\n\
                     - A workspace directory at ./.claudectl/workspaces/<uuid>\n\
                     - A git worktree at ~/.claudectl/projects/<repo>/<uuid>\n\n\
                     Each workspace has its own git worktree, allowing parallel development.\n\n\
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