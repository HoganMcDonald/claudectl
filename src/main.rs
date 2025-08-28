use crate::utils::output::{blank, error};
use clap::Parser;
use tracing::{error as log_error, info};

mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "claudectl")]
#[command(
    about = "A CLI tool for orchestrating Claude Code agents through the use of git worktrees.",
    help_template = "{about}\n\nUsage: claudectl [OPTIONS] [COMMAND]\n\nCommands:\n  init         Initialize the project for claudectl\n  task         Create a new task worktree\n  list         List all task worktrees\n  rm           Remove a task worktree\n\nUtility:\n  completions  Generate shell completions\n  repair       Repair shell completions and configuration\n  help         Print this message or the help of the given subcommand(s)\n\n{options}"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<commands::Commands>,

    /// Enable debug logging
    #[arg(long, global = true, help = "Enable debug logging output")]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();

    // Initialize logging based on debug flag
    init_logging(cli.debug);

    info!("Starting claudectl");

    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            // This shouldn't happen with clap's validation, but just in case
            error("No command provided");
            std::process::exit(1);
        }
    };

    if let Err(err) = commands::handle_command(command) {
        log_error!("Command failed: {}", err);
        blank();
        error(&err.message());
        std::process::exit(1);
    }

    info!("Command completed successfully");
}

fn init_logging(debug: bool) {
    if debug {
        // Enable debug logging to stderr when --debug flag is used
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("claudectl=debug")),
            )
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .init();
    } else {
        // Disable logging when debug flag is not used
        tracing_subscriber::fmt()
            .with_writer(std::io::sink)
            .with_env_filter(tracing_subscriber::EnvFilter::new("off"))
            .init();
    }
}
